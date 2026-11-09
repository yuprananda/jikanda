// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

use crate::{MainWindow, WorkspaceView, domain::progress::Data, persistence::Store};
use slint::ComponentHandle;
use std::{
    path::PathBuf,
    sync::mpsc::{self, Sender},
};

enum Job {
    Alarm {
        sound: crate::domain::alarm::AlarmSound,
        test: bool,
    },
    Save {
        data: Data,
        generation: i32,
        quit: bool,
    },
    Png {
        path: PathBuf,
        pixels: Vec<u8>,
        width: u32,
        height: u32,
    },
    Notify {
        body: String,
        test: bool,
    },
}

pub struct Worker {
    sender: Sender<Job>,
}

impl Worker {
    pub fn new(store: Store, main: slint::Weak<MainWindow>) -> Result<Self, String> {
        let (sender, receiver) = mpsc::channel::<Job>();
        std::thread::Builder::new().name("jikanda-io".into()).spawn(move || {
            while let Ok(job) = receiver.recv() {
                match job {
                    Job::Alarm { sound, test } => {
                        let result = crate::platform::audio::play(sound, &store.directory);
                        let _ = main.upgrade_in_event_loop(move |window| {
                            let view = window.global::<WorkspaceView>();
                            if test {
                                view.set_alarm_pending(false);
                                view.set_alarm_status(match &result { Ok(()) => "Bunyi uji selesai. Jika tidak terdengar, periksa volume perangkat.".into(), Err(error) => error.clone().into() });
                            }
                            if let Err(error) = result { view.set_notice(error.into()); }
                        });
                    }

                    Job::Save { data, generation, quit } => {
                        let result = store.save(&data, generation as u64);
                        let _ = main.upgrade_in_event_loop(move |window| {
                            let view = window.global::<WorkspaceView>();
                            if view.get_generation() == generation {
                                view.set_busy(false);
                                view.set_unsaved(result.is_err());
                                view.set_io_status(match &result { Ok(()) => "Tersimpan lokal.".into(), Err(error) => error.clone().into() });
                                if quit && result.is_ok() { let _ = slint::quit_event_loop(); }
                                if quit && result.is_err() {
                                    view.set_dialog(6); view.set_dialog_title("Perubahan belum tersimpan".into());
                                    view.set_dialog_detail("Aplikasi belum dapat memastikan semua perubahan tersimpan. Coba simpan lagi, atau buang perubahan yang belum tersimpan untuk keluar. Data tidak direset.".into());
                                    let _ = window.show(); window.window().set_minimized(false);
                                }
                            }
                        });
                    }
                    Job::Png { path, pixels, width, height } => {
                        let result = path.parent().ok_or_else(|| "Folder ekspor tidak valid.".to_owned())
                            .and_then(crate::persistence::valid_export_directory)
                            .and_then(|()| crate::export::encode(&path, &pixels, width, height));
                        let _ = main.upgrade_in_event_loop(move |window| {
                            window.global::<WorkspaceView>().set_notice(match result {
                                Ok(()) => format!("PNG tersimpan: {}", path.display()).into(),
                                Err(error) => format!("Ekspor gagal: {error}").into(),
                            });
                        });
                    }
                    Job::Notify { body, test } => {
                        let result = crate::platform::show_notification(&body);
                        if let Err(error) = &result { eprintln!("Desktop notification failed: {error}"); }
                        let _ = main.upgrade_in_event_loop(move |window| {
                            let view = window.global::<WorkspaceView>();
                            if test { view.set_notification_pending(false); }
                            if result.is_err() {
                                let message = "Notifikasi desktop tidak tersedia atau diblokir OS. Status sesi tetap tampil di Jikanda.";
                                view.set_notice(message.into());
                                if test { view.set_notification_status(message.into()); }
                            } else if test {
                                view.set_notification_status("Permintaan notifikasi diterima Windows/desktop. Jika banner tidak muncul, periksa Do Not Disturb dan izin notifikasi Jikanda pada pengaturan OS.".into());
                            }
                        });
                    }
                }
            }
        }).map_err(|e| format!("Tidak dapat menjalankan worker data: {e}"))?;
        Ok(Self { sender })
    }

    pub fn save(&self, data: Data, generation: i32, quit: bool) -> Result<(), String> {
        self.sender
            .send(Job::Save {
                data,
                generation,
                quit,
            })
            .map_err(|_| "Worker data berhenti; data belum tersimpan.".into())
    }
    pub fn png(
        &self,
        path: PathBuf,
        pixels: Vec<u8>,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        self.sender
            .send(Job::Png {
                path,
                pixels,
                width,
                height,
            })
            .map_err(|_| "Worker ekspor berhenti.".into())
    }
    pub fn alarm(&self, sound: crate::domain::alarm::AlarmSound, test: bool) -> Result<(), String> {
        self.sender
            .send(Job::Alarm { sound, test })
            .map_err(|_| "Worker alarm berhenti.".into())
    }
    pub fn notify(&self, body: String, test: bool) -> Result<(), String> {
        self.sender
            .send(Job::Notify { body, test })
            .map_err(|_| "Worker notifikasi berhenti.".into())
    }
}
