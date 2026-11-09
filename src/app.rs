// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

use crate::{
    CardWindow, DayBar, ErrorWindow, FloatingTimer, HistoryRow, MainWindow, Palette, SessionView,
    WorkspaceView,
    domain::{
        progress::{self, Data, Outcome, Record},
        session::{Mode, Phase, Session, State},
    },
    persistence::{self, Store},
    platform::SleepMonitor,
    worker::Worker,
};
use chrono::{DateTime, Days, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use slint::{ComponentHandle, ModelRc, Timer, TimerMode, VecModel};
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    time::{Duration, Instant},
};

struct Controller {
    session: Session,
    data: Data,
    directory: PathBuf,
    worker: Worker,
    epoch: Instant,
    sleep: SleepMonitor,
    main: slint::Weak<MainWindow>,
    floating: slint::Weak<FloatingTimer>,
    timer: Timer,
    day_timer: Timer,
    name_timer: Timer,
    started_ms: Option<i64>,
    started_day: NaiveDate,
    recorded: bool,
    announced: (Phase, u32),
    alarm_events: crate::domain::alarm::AlarmEvents,
    history_page: usize,
    pending_delete: Option<u64>,
    diagnostics: bool,
}

fn configured(data: &Data, mode: Mode) -> Session {
    let settings = &data.settings;
    Session::configured(
        mode,
        Duration::from_secs(if mode == Mode::Interval {
            settings.interval_seconds
        } else {
            settings.deep_seconds
        }),
        Duration::from_secs(settings.break_seconds),
        settings.cycles,
    )
    .expect("validated settings")
}

fn mode_from_index(index: i32) -> Mode {
    match index {
        1 => Mode::Interval,
        2 => Mode::Flow,
        _ => Mode::DeepFocus,
    }
}

impl Controller {
    fn new_session(&mut self) {
        self.name_timer.stop();
        self.data.new_session();
        if let Some(window) = self.main.upgrade() {
            window.global::<WorkspaceView>().set_task_draft("".into());
            window
                .global::<WorkspaceView>()
                .set_saved_session("".into());
        }
        self.session = configured(&self.data, self.session.mode);
        self.started_ms = None;
        self.recorded = false;
        self.alarm_events = crate::domain::alarm::AlarmEvents::default();
        self.update_tasks();
        self.refresh();
        self.save(false);
    }
    fn notify(&self, body: &str) {
        if let Err(error) = self.worker.notify(
            crate::i18n::translate(self.data.settings.language, body),
            false,
        ) {
            self.message(error);
        }
    }
    fn now(&self) -> Duration {
        self.epoch.elapsed()
    }
    fn active(&self) -> bool {
        matches!(self.session.state, State::Running | State::Paused)
    }
    fn dialog_open(&self) -> bool {
        self.main
            .upgrade()
            .is_some_and(|w| w.global::<WorkspaceView>().get_dialog() != 0)
    }
    fn message(&self, text: impl Into<String>) {
        if let Some(w) = self.main.upgrade() {
            w.global::<WorkspaceView>().set_notice(text.into().into());
        }
    }

    fn save(&self, quit: bool) {
        if let Some(w) = self.main.upgrade() {
            let view = w.global::<WorkspaceView>();
            if quit && view.get_dialog() == 0 {
                view.set_dialog(6);
                view.set_dialog_title("Menyimpan dan keluar…".into());
                view.set_dialog_detail("Menunggu penulisan data lokal selesai.".into());
            }
            let generation = view.get_generation().saturating_add(1);
            view.set_generation(generation);
            view.set_busy(true);
            view.set_unsaved(true);
            view.set_io_status("Menyimpan di perangkat…".into());
            if let Err(error) = self.worker.save(self.data.clone(), generation, quit) {
                view.set_busy(false);
                if quit {
                    view.set_dialog_title("Perubahan belum tersimpan".into());
                    view.set_dialog_detail(error.clone().into());
                }
                view.set_io_status(error.into());
            }
        }
    }

    fn refresh(&self) {
        let now = self.now();
        let duration = if self.session.mode == Mode::Flow {
            self.session.focus_elapsed(now)
        } else {
            self.session.remaining(now)
        };
        let seconds = duration.as_secs()
            + u64::from(self.session.mode != Mode::Flow && duration.subsec_nanos() > 0);
        let time = if seconds >= 3600 {
            format!(
                "{:02}:{:02}:{:02}",
                seconds / 3600,
                seconds % 3600 / 60,
                seconds % 60
            )
        } else {
            format!("{:02}:{:02}", seconds / 60, seconds % 60)
        };
        let action = if self.session.awaiting_next {
            if self.session.phase == Phase::Break {
                "Mulai istirahat"
            } else {
                "Mulai fokus"
            }
        } else {
            match self.session.state {
                State::Running => "Jeda",
                State::Paused => "Lanjutkan",
                _ => "Mulai fokus",
            }
        };
        let status = if self.session.uncertain && self.session.state == State::Paused {
            "Dijeda · tidur atau jeda waktu"
        } else if self.session.awaiting_next {
            if self.session.phase == Phase::Break {
                "Waktunya beristirahat"
            } else {
                "Siap untuk fokus berikutnya"
            }
        } else {
            match self.session.state {
                State::Ready => "Siap saat kamu siap",
                State::Running if self.session.phase == Phase::Break => {
                    "Istirahat tidak menambah waktu fokus"
                }
                State::Running => "Satu hal, sepenuh perhatian",
                State::Paused => "Dijeda · ambil waktumu",
                State::Interrupted => "Dihentikan · sesi terputus",
                State::Completed => "Sesi selesai · terima kasih sudah hadir",
            }
        };
        let mode_label = if self.session.phase == Phase::Break {
            "ISTIRAHAT"
        } else {
            self.session.mode.label()
        };
        let caption = match self.session.mode {
            Mode::Interval => format!(
                "Siklus {} dari {} · {}",
                self.session.cycle,
                self.session.cycles,
                if self.session.phase == Phase::Break {
                    "Istirahat"
                } else {
                    "Fokus"
                }
            ),
            Mode::DeepFocus => "Satu sesi tanpa jeda paksa".into(),
            Mode::Flow => "Count-up · akhiri saat kamu siap".into(),
        };
        macro_rules! update {
            ($window:expr) => {
                if let Some(w) = $window.upgrade() {
                    let view = w.global::<SessionView>();
                    view.set_time(time.clone().into());
                    view.set_session_name(self.data.session_name.clone().into());
                    view.set_long_time(seconds >= 3600);
                    view.set_status(status.into());
                    view.set_action(action.into());
                    view.set_active(self.active());
                    view.set_running(self.session.state == State::Running);
                    view.set_completed(self.session.state == State::Completed);
                    view.set_mode(match self.session.mode {
                        Mode::DeepFocus => 0,
                        Mode::Interval => 1,
                        Mode::Flow => 2,
                    });
                    view.set_mode_label(mode_label.into());
                    view.set_mode_caption(caption.clone().into());
                    view.set_duration_seconds(self.session.focus_target().as_secs() as i32);
                    view.set_remaining(if self.session.mode == Mode::Flow {
                        0.0
                    } else {
                        (self.session.remaining(now).as_secs_f64()
                            / self.session.target().as_secs_f64())
                        .clamp(0.0, 1.0) as f32
                    });
                    view.set_show_scene(self.data.settings.show_scene);
                    view.set_platform_note(crate::platform::window_note().into());
                    w.global::<Palette>().set_dark(self.data.settings.dark);
                    w.global::<crate::StandardPalette>()
                        .set_color_scheme(w.global::<Palette>().get_standard_scheme());
                    w.global::<crate::I18n>().set_indonesian(
                        self.data.settings.language == crate::i18n::Language::Indonesian,
                    );
                }
            };
        }
        update!(self.main);
        update!(self.floating);
        if let Some(window) = self.floating.upgrade() {
            let expanded = self.active();
            if window.get_expanded() != expanded {
                window.set_expanded(expanded);
                window.window().set_size(slint::LogicalSize::new(
                    228.0,
                    if expanded { 232.0 } else { 112.0 },
                ));
            }
        }
        if self.session.state != State::Running {
            self.timer.stop();
        }
    }

    fn reconcile(&mut self) {
        if self.sleep.uncertain_gap() {
            self.session.suspend();
        }
        self.session.sample(self.now());
        self.settle();
    }

    fn settle(&mut self) {
        if self.alarm_events.poll(&self.session)
            && self.data.settings.alarm_enabled
            && !self.diagnostics
            && let Err(error) = self.worker.alarm(self.data.settings.alarm_sound, false)
        {
            self.message(error);
        }
        let phase = (self.session.phase, self.session.cycle);
        if self.session.awaiting_next && phase != self.announced {
            self.announced = phase;
            if self.data.settings.notifications && !self.diagnostics {
                self.notify(if self.session.phase == Phase::Break {
                    "Segmen fokus selesai. Mulai istirahat saat kamu siap."
                } else {
                    "Istirahat selesai. Mulai fokus berikutnya saat kamu siap."
                });
            }
        }
        if self.session.state == State::Completed && !self.recorded {
            self.record(Outcome::Completed);
            if self.data.settings.notifications && !self.diagnostics {
                self.notify(
                    "Sesi selesai. Terima kasih sudah memberi waktu untuk hal yang kamu pilih.",
                );
            }
        }
    }

    fn record(&mut self, outcome: Outcome) {
        if self.recorded {
            return;
        }
        let Some(started_ms) = self.started_ms else {
            return;
        };
        let focus_ms = self
            .session
            .focus_elapsed(self.now())
            .as_millis()
            .min(u64::MAX as u128) as u64;
        if focus_ms == 0 {
            self.recorded = true;
            self.message("Sesi terlalu singkat untuk dicatat; tidak ada durasi yang dibuat-buat.");
            return;
        }
        let record = Record {
            id: self.data.next_id,
            mode: self.session.mode,
            started_ms,
            ended_ms: Utc::now().timestamp_millis(),
            local_day: self.started_day,
            focus_ms,
            outcome,
            note: String::new(),
            session_name: self.data.session_name.trim().to_owned(),
            tasks: self.data.tasks.clone(),
        };
        if let Err(error) = record.validate() {
            self.message(error);
            return;
        }
        if self.data.records.len() >= 10000 {
            self.message("Batas riwayat tercapai. Data sesi tetap ada di memori; hapus entri sebelum mencoba simpan lagi.");
            return;
        }
        self.data.next_id += 1;
        if let Some(window) = self.main.upgrade() {
            let view = window.global::<WorkspaceView>();
            view.set_card_id(record.id.to_string().into());
            view.set_saved_session(record.id.to_string().into());
        }
        self.data.records.push(record);
        self.recorded = true;
        self.history_page = 0;
        self.update_progress();
        self.save(false);
    }

    fn apply_window_settings(&self) {
        if let Some(w) = self.main.upgrade() {
            w.set_decorated(self.data.settings.main_decorated);
        }
        if let Some(w) = self.floating.upgrade() {
            w.set_decorated(self.data.settings.floating_decorated);
            w.set_keep_on_top(self.data.settings.always_on_top);
            if let Err(error) =
                crate::platform::set_opacity(w.window(), self.data.settings.floating_opacity)
            {
                self.message(error);
            }
        }
    }

    fn fill_settings(&self) {
        if let Some(w) = self.main.upgrade() {
            let view = w.global::<WorkspaceView>();
            view.set_app_version(env!("CARGO_PKG_VERSION").into());
            let s = &self.data.settings;
            let minutes = |seconds: u64| {
                if seconds.is_multiple_of(60) {
                    (seconds / 60).to_string()
                } else {
                    format!("{:.3}", seconds as f64 / 60.0)
                }
            };
            view.set_deep_minutes(minutes(s.deep_seconds).into());
            view.set_interval_minutes(minutes(s.interval_seconds).into());
            view.set_break_minutes(minutes(s.break_seconds).into());
            view.set_cycles(s.cycles.to_string().into());
            view.set_notifications(s.notifications);
            view.set_alarm_enabled(s.alarm_enabled);
            view.set_alarm_sound(s.alarm_sound.index());
            view.set_display_name(s.display_name.clone().into());
            view.set_topmost(s.always_on_top);
            view.set_framed(s.floating_decorated);
            view.set_opacity_supported(crate::platform::opacity_supported());
            view.set_floating_opacity(if crate::platform::opacity_supported() {
                s.floating_opacity as f32
            } else {
                100.0
            });
            view.set_scene(s.show_scene);
            view.set_export_directory(
                self.export_directory()
                    .to_string_lossy()
                    .into_owned()
                    .into(),
            );
            view.set_data_path(self.directory.to_string_lossy().into_owned().into());
        }
    }

    fn export_directory(&self) -> PathBuf {
        if self.data.settings.export_directory.is_empty() {
            self.directory.join("exports")
        } else {
            PathBuf::from(&self.data.settings.export_directory)
        }
    }

    fn update_progress(&self) {
        let Some(w) = self.main.upgrade() else {
            return;
        };
        let view = w.global::<WorkspaceView>();
        let today = progress::today();
        let start = progress::week_start(today);
        let daily = progress::summarize(&self.data.records, today, today);
        let weekly = progress::summarize(&self.data.records, start, today);
        view.set_today_total(progress::duration_label(daily.focus_ms).into());
        view.set_week_total(progress::duration_label(weekly.focus_ms).into());
        view.set_summary_detail(
            format!(
                "{} sesi dalam 7 hari · {} selesai · {} terputus yang dikonfirmasi",
                weekly.sessions,
                weekly.completed,
                weekly.sessions - weekly.completed
            )
            .into(),
        );
        let values: Vec<_> = (0..7)
            .map(|offset| {
                let day = start.checked_add_days(Days::new(offset)).unwrap_or(start);
                (
                    day,
                    progress::summarize(&self.data.records, day, day).focus_ms,
                )
            })
            .collect();
        let max = values.iter().map(|(_, ms)| *ms).max().unwrap_or(0).max(1);
        view.set_days(ModelRc::new(VecModel::from(
            values
                .into_iter()
                .map(|(day, ms)| DayBar {
                    label: day.format("%d/%m").to_string().into(),
                    value: ms as f32 / max as f32,
                    duration: progress::duration_label(ms).into(),
                })
                .collect::<Vec<_>>(),
        )));
        let count = self.data.records.len();
        let pages = count.div_ceil(20).max(1);
        let page = self.history_page.min(pages - 1);
        let mut ordered: Vec<_> = self.data.records.iter().collect();
        ordered.sort_unstable_by_key(|r| std::cmp::Reverse((r.started_ms, r.id)));
        let rows = ordered
            .into_iter()
            .skip(page * 20)
            .take(20)
            .map(|r| {
                let date = DateTime::from_timestamp_millis(r.started_ms)
                    .map(|d| d.with_timezone(&Local).format("%d/%m/%Y %H:%M").to_string())
                    .unwrap_or_default();
                HistoryRow {
                    id: r.id.to_string().into(),
                    title: r.mode.label().into(),
                    duration: progress::duration_label(r.focus_ms).into(),
                    detail: format!(
                        "{date} · {}",
                        if r.outcome == Outcome::Completed {
                            "Selesai"
                        } else {
                            "Terputus · dikonfirmasi"
                        }
                    )
                    .into(),
                    note: r.note.clone().into(),
                    session_name: r.session_name.clone().into(),
                    task_summary: if r.tasks.entries.is_empty() {
                        "".into()
                    } else {
                        format!(
                            "{} / {} tugas selesai",
                            r.tasks.entries.iter().filter(|task| task.completed).count(),
                            r.tasks.entries.len()
                        )
                        .into()
                    },
                }
            })
            .collect::<Vec<_>>();
        view.set_history(ModelRc::new(VecModel::from(rows)));
        view.set_history_page(format!("{} / {} · {} sesi", page + 1, pages, count).into());
        view.set_previous_page(page > 0);
        view.set_next_page(page + 1 < pages);
    }

    fn update_tasks(&self) {
        if let Some(window) = self.main.upgrade() {
            let rows: Vec<_> = self
                .data
                .tasks
                .entries
                .iter()
                .map(|task| crate::TaskRow {
                    id: task.id.to_string().into(),
                    title: task.title.clone().into(),
                    completed: task.completed,
                    selected: self.data.tasks.active_id == Some(task.id),
                })
                .collect();
            let model: ModelRc<crate::TaskRow> = ModelRc::new(VecModel::from(rows));
            window.global::<WorkspaceView>().set_tasks(model.clone());
            if let Some(floating) = self.floating.upgrade() {
                floating.global::<WorkspaceView>().set_tasks(model);
            }
        }
    }

    fn refresh_card(&self) -> Result<(), String> {
        let Some(w) = self.main.upgrade() else {
            return Ok(());
        };
        let view = w.global::<WorkspaceView>();
        view.set_card_ready(false);
        let language = self.data.settings.language;
        let tr = |text: &str| crate::i18n::translate(language, text);
        let options: Vec<slint::SharedString> = self
            .data
            .records
            .iter()
            .rev()
            .map(|r| {
                let title = if r.session_name.is_empty() {
                    tr("Sesi tanpa nama")
                } else {
                    r.session_name.clone()
                };
                format!(
                    "{} | {} | {}",
                    title,
                    r.local_day.format("%d/%m/%Y"),
                    progress::share_duration(r.focus_ms)
                )
                .into()
            })
            .collect();
        view.set_card_options(ModelRc::new(VecModel::from(options)));
        if view.get_card_id().is_empty()
            && let Some(record) = self.data.records.last()
        {
            view.set_card_id(record.id.to_string().into());
        }
        let id = view.get_card_id().parse::<u64>().ok();
        let selected = self
            .data
            .records
            .iter()
            .rev()
            .enumerate()
            .find(|(_, r)| Some(r.id) == id);
        let Some((index, record)) = selected else {
            view.set_card_index(-1);
            view.set_card_time("0m".into());
            view.set_card_caption(tr("Pilih sesi tersimpan untuk membuat kartu.").into());
            view.set_card_duration("0 detik".into());
            view.set_card_detail("".into());
            view.set_card_period("".into());
            view.set_card_tasks("".into());
            return Ok(());
        };
        view.set_card_index(index as i32);
        view.set_card_duration(progress::duration_label(record.focus_ms).into());
        view.set_card_time(progress::share_duration(record.focus_ms).into());
        let status = if record.outcome == Outcome::Completed {
            "Selesai"
        } else {
            "Terputus"
        };
        view.set_card_detail(format!("{} | {}", tr(record.mode.label()), tr(status)).into());
        view.set_card_period(record.local_day.format("%d/%m/%Y").to_string().into());
        view.set_card_caption(
            if record.session_name.is_empty() {
                tr("Sesi tanpa nama")
            } else {
                record.session_name.clone()
            }
            .into(),
        );
        view.set_card_name(self.data.settings.display_name.clone().into());
        view.set_card_tasks(record.share_tasks().into());
        view.set_card_ready(record.focus_ms > 0);
        Ok(())
    }

    fn export_card(&self) -> Result<(), String> {
        self.refresh_card()?;
        let Some(w) = self.main.upgrade() else {
            return Ok(());
        };
        let view = w.global::<WorkspaceView>();
        if view.get_busy() || view.get_unsaved() {
            return Err("Tunggu data tersimpan sebelum mengekspor kartu.".into());
        }
        if !view.get_card_ready() {
            return Err("Pilih sesi tersimpan untuk membuat kartu.".into());
        }
        let card = CardWindow::new().map_err(|e| e.to_string())?;
        crate::i18n::bind(card.global::<crate::I18n>(), self.data.settings.language);
        card.set_duration(view.get_card_time());
        card.set_detail(view.get_card_detail());
        card.set_period(view.get_card_period());
        card.set_personal_name(if view.get_include_name() {
            view.get_card_name()
        } else {
            "".into()
        });
        card.set_caption(view.get_card_caption());
        card.set_tasks(if view.get_include_tasks() {
            view.get_card_tasks()
        } else {
            "".into()
        });
        card.set_purple(view.get_card_template() == 1);
        card.set_format(view.get_card_format());
        let (width, height) = if view.get_card_format() == 1 {
            (1080, 1350)
        } else {
            (1080, 1920)
        };
        card.window()
            .dispatch_event(slint::platform::WindowEvent::ScaleFactorChanged { scale_factor: 1.0 });
        card.window()
            .set_size(slint::PhysicalSize::new(width, height));
        let pixels = card.window().take_snapshot().map_err(|e| e.to_string())?;
        if (pixels.width(), pixels.height()) != (width, height) {
            return Err("Ukuran render kartu tidak sesuai format pilihan.".into());
        }
        let directory = self.export_directory();
        // Directory creation and PNG encoding happen on the worker; never upload.
        let path = directory.join(format!(
            "jikanda-{}-{}.png",
            Utc::now().format("%Y%m%d-%H%M%S-%3f"),
            self.data.next_id
        ));
        self.worker.png(
            path,
            pixels.as_bytes().to_vec(),
            pixels.width(),
            pixels.height(),
        )?;
        self.message("Membuat PNG lokal…");
        Ok(())
    }

    fn dialog(&self, kind: i32, title: &str, detail: &str) {
        if let Some(w) = self.main.upgrade() {
            let view = w.global::<WorkspaceView>();
            view.set_dialog(kind);
            view.set_dialog_title(title.into());
            view.set_dialog_detail(detail.into());
            view.set_notice("".into());
            let _ = w.show();
            w.window().set_minimized(false);
        }
    }

    fn request_stop(&mut self, quit: bool) {
        self.reconcile();
        if self.active() {
            self.session.pause(self.now());
            self.settle();
            self.refresh();
            if self.active() {
                self.dialog(if quit { 2 } else { 1 }, if quit { "Akhiri sesi dan keluar?" } else { "Simpan waktu yang sudah kamu beri?" },
                "Sesi dijeda. Simpan sebagai terputus, buang sesi ini, atau kembali untuk melanjutkan. Waktu istirahat tidak ikut dicatat.");
                return;
            }
        }
        if quit {
            let Some(w) = self.main.upgrade() else {
                return;
            };
            let view = w.global::<WorkspaceView>();
            if view.get_busy() {
                self.message("Tunggu penyimpanan selesai, lalu coba keluar lagi.");
            } else if view.get_unsaved() {
                self.dialog(6, "Perubahan belum tersimpan", "Coba simpan dan keluar, atau buang perubahan yang belum tersimpan. Data yang sudah tersimpan tidak dihapus.");
            } else {
                self.save(true);
            }
        }
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .renderer_name("software".into())
        .select()?;
    let diagnostics = std::env::args().any(|a| a.starts_with("--probe-") || a == "--verify-native");
    if diagnostics {
        std::panic::set_hook(Box::new(|info| {
            let path = format!("artifacts/panic-{}.txt", std::process::id());
            let _ = std::fs::write(path, info.to_string());
            std::process::exit(1);
        }));
    }
    let args: Vec<_> = std::env::args().collect();
    let directory = if diagnostics {
        std::env::current_dir()?.join(format!(
            "artifacts/state-{}-{}",
            std::process::id(),
            Utc::now().timestamp_millis()
        ))
    } else if let Some(index) = args.iter().position(|a| a == "--data-dir") {
        PathBuf::from(
            args.get(index + 1)
                .ok_or("--data-dir needs an absolute path")?,
        )
    } else {
        persistence::default_directory()?
    };
    if !directory.is_absolute() {
        return Err("Data directory must be absolute".into());
    }
    let (store, mut data) = match Store::open(directory.clone()) {
        Ok(result) => result,
        Err(error) => {
            let window = ErrorWindow::new()?;
            crate::i18n::bind(
                window.global::<crate::I18n>(),
                crate::i18n::Language::English,
            );
            window.set_message(
                format!(
                    "{}\n\n{}",
                    crate::i18n::english(&error),
                    directory.display()
                )
                .into(),
            );
            window.run()?;
            return Ok(());
        }
    };
    data.settings.use_fixed_durations();
    let main = MainWindow::new()?;
    let floating = FloatingTimer::new()?;
    crate::i18n::bind(main.global::<crate::I18n>(), data.settings.language);
    crate::i18n::bind(floating.global::<crate::I18n>(), data.settings.language);
    let worker = Worker::new(store, main.as_weak())?;
    let session = configured(&data, data.settings.last_mode);
    let controller = Rc::new(RefCell::new(Controller {
        session,
        data,
        directory,
        worker,
        epoch: Instant::now(),
        sleep: SleepMonitor::new(),
        main: main.as_weak(),
        floating: floating.as_weak(),
        timer: Timer::default(),
        day_timer: Timer::default(),
        name_timer: Timer::default(),
        started_ms: None,
        started_day: progress::today(),
        recorded: false,
        announced: (Phase::Focus, 1),
        alarm_events: crate::domain::alarm::AlarmEvents::default(),
        history_page: 0,
        pending_delete: None,
        diagnostics,
    }));
    macro_rules! wire { ($window:expr) => {{
        let view = $window.global::<SessionView>();
        let c = controller.clone();
        view.on_edit_session_name(move |text| {
            let mut app = c.borrow_mut();
            if app.active() || app.dialog_open() { return; }
            app.data.session_name = text.chars().take(120).collect();
            app.refresh();
            let weak = Rc::downgrade(&c);
            app.name_timer.start(TimerMode::SingleShot, Duration::from_millis(400), move || { if let Some(c) = weak.upgrade() { c.borrow().save(false); } });
        });
        let c = controller.clone();
        view.on_toggle(move || {
            let mut app = c.borrow_mut();
            if app.dialog_open() { restore_main(&app, false); return; }
            if !app.active() {
                if app.data.records.len() >= 10000 { app.message("Batas 10000 entri tercapai. Hapus entri yang tidak diperlukan sebelum memulai sesi baru."); return; }
                app.started_ms = Some(Utc::now().timestamp_millis()); app.started_day = progress::today(); app.recorded = false; app.alarm_events = crate::domain::alarm::AlarmEvents::default(); app.announced = (Phase::Focus, 1);
            }
            let now = app.now(); let uncertain = app.sleep.uncertain_gap();
            if toggle_session(&mut app.session, now, uncertain) {
                app.sleep.reset(); let weak = Rc::downgrade(&c);
                app.timer.start(TimerMode::Repeated, Duration::from_secs(1), move || {
                    if let Some(c) = weak.upgrade() { let mut app = c.borrow_mut(); app.reconcile(); app.refresh(); }
                });
            }
            app.settle(); app.refresh();
        });
        let c = controller.clone(); view.on_stop(move || { c.borrow_mut().request_stop(false); });
        let c = controller.clone(); view.on_quit(move || { c.borrow_mut().request_stop(true); });
        let c = controller.clone(); view.on_finish_flow(move || {
            let mut app = c.borrow_mut();
            if app.dialog_open() { restore_main(&app, false); return; }
            let now = app.now(); app.session.finish_flow(now); app.settle(); app.refresh();
        });
        let c = controller.clone(); view.on_select_mode(move |index| {
            let mut app = c.borrow_mut(); if app.active() { return; }
            let mode = mode_from_index(index); app.data.settings.last_mode = mode; app.session = configured(&app.data, mode);
            app.started_ms = None; app.recorded = false; app.refresh(); app.save(false);
        });
        let c = controller.clone(); view.on_duration(move |seconds| { set_duration(&mut c.borrow_mut(), seconds as i64); });
        let c = controller.clone(); view.on_custom_duration(move |text| {
            let mut app = c.borrow_mut();
            match text.trim().parse::<i64>() { Ok(seconds) => set_duration(&mut app, seconds), Err(_) => duration_message(&app, "Masukkan bilangan bulat 1–86400 detik.") }
        });
        let c = controller.clone(); view.on_theme(move || {
            let mut app = c.borrow_mut(); app.data.settings.dark = !app.data.settings.dark; app.refresh(); app.save(false);
        });
        let c = controller.clone(); view.on_floating(move || {
            let app = c.borrow(); if let Some(w) = app.floating.upgrade() {
                match w.show() { Ok(()) => { w.window().set_minimized(false); if let Some(main) = app.main.upgrade() { report(main.hide()); } }, Err(e) => app.message(e.to_string()) }
            }
            schedule_opacity(&c);
        });
        let c = controller.clone(); view.on_close_floating(move || { restore_main(&c.borrow(), true); });
    }}; }
    wire!(main);
    wire!(floating);
    wire_workspace(&main, &controller);
    wire_tasks(floating.global::<WorkspaceView>(), &controller);
    let c = controller.clone();
    main.window().on_close_requested(move || {
        c.borrow_mut().request_stop(true);
        slint::CloseRequestResponse::KeepWindowShown
    });
    let c = controller.clone();
    floating.window().on_close_requested(move || {
        if restore_main(&c.borrow(), false) {
            slint::CloseRequestResponse::HideWindow
        } else {
            slint::CloseRequestResponse::KeepWindowShown
        }
    });
    {
        let app = controller.borrow();
        app.apply_window_settings();
        app.fill_settings();
        app.update_progress();
        app.refresh();
        app.update_tasks();
        report(app.refresh_card());
    }
    main.show()?;
    schedule_day_refresh(&controller);
    if args
        .iter()
        .any(|a| a == "--probe-floating" || a == "--probe-both")
    {
        floating.show()?;
        if args.iter().any(|a| a == "--probe-floating") {
            main.hide()?;
        }
    }
    if args.iter().any(|a| a == "--probe-running") {
        main.global::<SessionView>().invoke_toggle();
    }
    let _verification = crate::verification::schedule(&main, &floating)?;
    schedule_opacity(&controller);
    slint::run_event_loop()?;
    Ok(())
}

fn schedule_opacity(controller: &Rc<RefCell<Controller>>) {
    let weak = Rc::downgrade(controller);
    slint::Timer::single_shot(Duration::from_millis(100), move || {
        if let Some(controller) = weak.upgrade() {
            controller.borrow().apply_window_settings();
        }
    });
}

fn schedule_day_refresh(controller: &Rc<RefCell<Controller>>) {
    let now = Local::now();
    let next = now
        .date_naive()
        .succ_opt()
        .and_then(|day| day.and_hms_opt(0, 0, 0))
        .and_then(|time| Local.from_local_datetime(&time).earliest());
    let delay = next
        .and_then(|next| (next - now).to_std().ok())
        .unwrap_or(Duration::from_secs(86400))
        .max(Duration::from_secs(1));
    let weak = Rc::downgrade(controller);
    controller
        .borrow()
        .day_timer
        .start(TimerMode::SingleShot, delay, move || {
            if let Some(controller) = weak.upgrade() {
                {
                    let app = controller.borrow();
                    app.update_progress();
                    report(app.refresh_card());
                }
                schedule_day_refresh(&controller);
            }
        });
}

fn wire_tasks(view: WorkspaceView<'_>, controller: &Rc<RefCell<Controller>>) {
    let c = controller.clone();
    view.on_add_task(move |title| {
        let mut app = c.borrow_mut();
        if app.dialog_open() {
            restore_main(&app, false);
            return;
        }
        if let Err(error) = app.data.tasks.add(&title) {
            app.message(error);
            return;
        }
        if let Some(window) = app.main.upgrade() {
            window.global::<WorkspaceView>().set_task_draft("".into());
        }
        app.update_tasks();
        app.save(false);
    });
    let c = controller.clone();
    view.on_toggle_task(move |id| {
        let mut app = c.borrow_mut();
        if app.dialog_open() {
            restore_main(&app, false);
            return;
        }
        if let Ok(id) = id.parse() {
            app.data.tasks.toggle(id);
            app.update_tasks();
            app.save(false);
        }
    });
    let c = controller.clone();
    view.on_select_task(move |id| {
        let mut app = c.borrow_mut();
        if app.dialog_open() {
            restore_main(&app, false);
            return;
        }
        if let Ok(id) = id.parse() {
            app.data.tasks.select(id);
            app.update_tasks();
            app.save(false);
        }
    });
    let c = controller.clone();
    view.on_delete_task(move |id| {
        let mut app = c.borrow_mut();
        if app.dialog_open() {
            restore_main(&app, false);
            return;
        }
        if let Ok(id) = id.parse() {
            app.data.tasks.delete(id);
            app.update_tasks();
            app.save(false);
        }
    });
}

fn wire_workspace(main: &MainWindow, controller: &Rc<RefCell<Controller>>) {
    wire_tasks(main.global::<WorkspaceView>(), controller);
    let view = main.global::<WorkspaceView>();
    let c = controller.clone();
    view.on_new_session(move || {
        let mut app = c.borrow_mut();
        if app.active() || app.dialog_open() { return; }
        if !app.data.session_name.trim().is_empty() || !app.data.tasks.entries.is_empty() {
            app.dialog(7, "Mulai sesi baru?", "Nama dan daftar tugas saat ini akan dikosongkan. Riwayat sesi yang sudah tersimpan tetap utuh.");
        } else { app.new_session(); }
    });
    let weak = main.as_weak();
    view.on_theme(move || {
        if let Some(window) = weak.upgrade() {
            window.global::<SessionView>().invoke_theme();
        }
    });
    let c = controller.clone();
    view.on_opacity(move |percent| {
        if !percent.is_finite() || !crate::platform::opacity_supported() {
            return;
        }
        let mut app = c.borrow_mut();
        app.data.settings.floating_opacity = percent.round().clamp(50.0, 100.0) as u8;
        if let Some(window) = app.main.upgrade() {
            window
                .global::<WorkspaceView>()
                .set_floating_opacity(app.data.settings.floating_opacity as f32);
        }
        app.apply_window_settings();
        app.save(false);
    });
    let c = controller.clone();
    view.on_language(move |indonesian| {
        let mut app = c.borrow_mut();
        app.data.settings.language = if indonesian {
            crate::i18n::Language::Indonesian
        } else {
            crate::i18n::Language::English
        };
        app.refresh();
        app.update_progress();
        report(app.refresh_card());
        app.save(false);
    });
    let c = controller.clone();
    view.on_navigate(move |page| {
        let app = c.borrow();
        if let Some(w) = app.main.upgrade() {
            w.global::<WorkspaceView>().set_page(page.clamp(0, 3));
            app.update_progress();
            if page == 2
                && let Err(e) = app.refresh_card()
            {
                app.message(e);
            }
            if page == 3 {
                app.fill_settings();
            }
        }
    });
    let c = controller.clone();
    view.on_page_history(move |delta| {
        let mut app = c.borrow_mut();
        let last = app.data.records.len().div_ceil(20).saturating_sub(1);
        app.history_page = app
            .history_page
            .saturating_add_signed(delta as isize)
            .min(last);
        app.update_progress();
    });
    let c = controller.clone();
    view.on_edit(move |id| {
        let app = c.borrow(); let Some(record) = id.parse::<u64>().ok().and_then(|id| app.data.records.iter().find(|r| r.id == id)) else { return; };
        if let Some(w) = app.main.upgrade() {
            let v = w.global::<WorkspaceView>(); v.set_edit_id(id); v.set_edit_note(record.note.clone().into());
            v.set_edit_session_name(record.session_name.clone().into());
            let tasks: Vec<_> = record.tasks.entries.iter().map(|task| crate::TaskRow { id: task.id.to_string().into(), title: task.title.clone().into(), completed: task.completed, selected: record.tasks.active_id == Some(task.id) }).collect();
            v.set_edit_tasks(ModelRc::new(VecModel::from(tasks)));
            v.set_edit_mode(match record.mode { Mode::DeepFocus => 0, Mode::Interval => 1, Mode::Flow => 2 });
            v.set_edit_seconds(format!("{:.3}", record.focus_ms as f64 / 1000.0).into());
            let formatted = |ms| DateTime::from_timestamp_millis(ms).unwrap().with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string();
            v.set_edit_start(formatted(record.started_ms).into()); v.set_edit_end(formatted(record.ended_ms).into()); v.set_edit_completed(record.outcome == Outcome::Completed);
            app.dialog(5, "Koreksi sesi", "Durasi fokus boleh lebih pendek dari rentang waktu karena jeda/istirahat. Koreksi tanggal, durasi, catatan, atau status dengan sengaja.");
        }
    });
    let c = controller.clone();
    view.on_delete(move |id| {
        let mut app = c.borrow_mut(); app.pending_delete = id.parse().ok(); app.dialog(4, "Hapus entri ini?", "Entri dan catatannya akan dihapus dari riwayat lokal. Tindakan ini tidak dapat dibatalkan.");
    });
    let c = controller.clone();
    view.on_clear_history(move || { c.borrow().dialog(3, "Hapus seluruh riwayat?", "Semua sesi tersimpan dan catatannya dihapus. Preferensi tetap dipertahankan. Kartu PNG yang pernah diekspor tidak dihapus."); });
    let c = controller.clone();
    view.on_save_edit(move || {
        let result = save_edit(&mut c.borrow_mut());
        if let Err(e) = result {
            c.borrow().message(e);
        }
    });
    let c = controller.clone();
    view.on_save_settings(move || {
        let result = save_settings(&mut c.borrow_mut());
        if let Err(e) = result {
            c.borrow().message(e);
        }
    });
    let c = controller.clone();
    view.on_select_card(move |index| {
        let app = c.borrow();
        if app.dialog_open() {
            return;
        }
        if let Some(record) = app.data.records.iter().rev().nth(index.max(0) as usize)
            && let Some(window) = app.main.upgrade()
        {
            window
                .global::<WorkspaceView>()
                .set_card_id(record.id.to_string().into());
            report(app.refresh_card());
        }
    });
    let c = controller.clone();
    view.on_share_session(move |id| {
        let app = c.borrow();
        if app.dialog_open() {
            return;
        }
        if let Some(window) = app.main.upgrade() {
            let view = window.global::<WorkspaceView>();
            view.set_card_id(id);
            view.set_page(2);
            report(app.refresh_card());
        }
    });
    let c = controller.clone();
    view.on_refresh_card(move || {
        let app = c.borrow();
        if let Err(e) = app.refresh_card() {
            if let Some(w) = app.main.upgrade() {
                w.global::<WorkspaceView>().set_card_ready(false);
            }
            app.message(e);
        }
    });
    let c = controller.clone();
    view.on_export_card(move || {
        let app = c.borrow();
        if let Err(e) = app.export_card() {
            app.message(e);
        }
    });
    let c = controller.clone();
    view.on_retry_save(move || {
        c.borrow().save(false);
    });
    let c = controller.clone();
    view.on_open_data(move || {
        let app = c.borrow();
        if let Err(e) = crate::platform::open_directory(&app.directory) {
            app.message(e);
        }
    });
    let c = controller.clone();
    view.on_open_exports(move || {
        let app = c.borrow();
        let path = app.export_directory();
        if path.is_dir() {
            if let Err(e) = crate::platform::open_directory(&path) {
                app.message(e);
            }
        } else {
            app.message("Folder ekspor akan dibuat setelah ekspor pertama.");
        }
    });
    let c = controller.clone();
    view.on_test_alarm(move || {
        let app = c.borrow();
        let Some(window) = app.main.upgrade() else {
            return;
        };
        let view = window.global::<WorkspaceView>();
        if view.get_alarm_pending() || app.dialog_open() {
            return;
        }
        if app.diagnostics {
            view.set_alarm_status("Mode pengujian: alarm tidak diputar.".into());
            return;
        }
        let sound = match crate::domain::alarm::AlarmSound::from_index(view.get_alarm_sound()) {
            Ok(sound) => sound,
            Err(error) => {
                view.set_alarm_status(error.into());
                return;
            }
        };
        view.set_alarm_pending(true);
        view.set_alarm_status("Memutar...".into());
        if let Err(error) = app.worker.alarm(sound, true) {
            view.set_alarm_pending(false);
            view.set_alarm_status(error.into());
        }
    });
    let c = controller.clone();
    view.on_test_notification(move || {
        let app = c.borrow();
        let Some(window) = app.main.upgrade() else {
            return;
        };
        let view = window.global::<WorkspaceView>();
        if view.get_notification_pending() || app.dialog_open() {
            return;
        }
        if app.diagnostics {
            view.set_notification_status(
                "Mode pengujian: notifikasi desktop tidak dikirim.".into(),
            );
            return;
        }
        view.set_notification_pending(true);
        view.set_notification_status("Mengirim notifikasi uji...".into());
        let body = crate::i18n::translate(
            app.data.settings.language,
            "Ini notifikasi uji Jikanda. Tidak ada data sesi atau catatan yang disertakan.",
        );
        if let Err(error) = app.worker.notify(body, true) {
            view.set_notification_pending(false);
            view.set_notification_status(error.into());
        }
    });
    let c = controller.clone();
    view.on_dialog_cancel(move || {
        if let Some(w) = c.borrow().main.upgrade() {
            w.global::<WorkspaceView>().set_dialog(0);
        }
    });
    let c = controller.clone();
    view.on_dialog_save(move || {
        let mut app = c.borrow_mut();
        let Some(w) = app.main.upgrade() else {
            return;
        };
        let v = w.global::<WorkspaceView>();
        let kind = v.get_dialog();
        match kind {
            1 | 2 => {
                let now = app.now();
                app.session.stop(now);
                app.record(Outcome::Interrupted);
                app.refresh();
                if kind == 2 {
                    app.save(true);
                } else {
                    v.set_dialog(0);
                }
            }
            3 => {
                app.data.records.clear();
                app.history_page = 0;
                app.update_progress();
                v.set_dialog(0);
                app.save(false);
            }
            4 => {
                if let Some(id) = app.pending_delete.take() {
                    app.data.records.retain(|r| r.id != id);
                }
                app.update_progress();
                v.set_dialog(0);
                app.save(false);
            }
            6 => app.save(true),
            7 => {
                app.new_session();
                v.set_dialog(0);
            }
            _ => {}
        }
    });
    let c = controller.clone();
    view.on_dialog_discard(move || {
        let mut app = c.borrow_mut();
        let Some(w) = app.main.upgrade() else {
            return;
        };
        let v = w.global::<WorkspaceView>();
        let kind = v.get_dialog();
        if kind == 6 {
            let _ = slint::quit_event_loop();
            return;
        }
        let now = app.now();
        app.session.stop(now);
        app.recorded = true;
        app.refresh();
        if kind == 2 {
            app.save(true);
        } else {
            v.set_dialog(0);
        }
    });
}

fn set_duration(app: &mut Controller, seconds: i64) {
    if !app.diagnostics || app.active() || app.session.mode == Mode::Flow {
        return;
    }
    if !(1..=86400).contains(&seconds) {
        duration_message(app, "Masukkan bilangan bulat 1–86400 detik.");
        return;
    }
    let mode = app.session.mode;
    if mode == Mode::Interval {
        app.data.settings.interval_seconds = seconds as u64;
    } else {
        app.data.settings.deep_seconds = seconds as u64;
    }
    app.session = configured(&app.data, mode);
    app.started_ms = None;
    app.recorded = false;
    app.refresh();
    app.save(false);
    duration_message(app, &format!("Durasi fokus {seconds} detik."));
}
fn duration_message(app: &Controller, message: &str) {
    if let Some(w) = app.main.upgrade() {
        w.global::<SessionView>().set_duration_note(message.into());
    }
}

fn save_settings(app: &mut Controller) -> Result<(), String> {
    let Some(w) = app.main.upgrade() else {
        return Ok(());
    };
    let view = w.global::<WorkspaceView>();
    let mut settings = app.data.settings.clone();
    let minutes = |text: slint::SharedString, maximum: u64| -> Result<u64, String> {
        let value: f64 = text
            .trim()
            .parse()
            .map_err(|_| "Durasi harus berupa angka menit.".to_owned())?;
        let seconds = (value * 60.0).round();
        if !seconds.is_finite() || seconds < 1.0 || seconds > maximum as f64 {
            return Err("Durasi fokus 1 detik–24 jam, istirahat 1 detik–120 menit.".into());
        }
        Ok(seconds as u64)
    };
    if app.diagnostics {
        settings.deep_seconds = minutes(view.get_deep_minutes(), 86400)?;
        settings.interval_seconds = minutes(view.get_interval_minutes(), 86400)?;
        settings.break_seconds = minutes(view.get_break_minutes(), 7200)?;
        settings.cycles = view
            .get_cycles()
            .trim()
            .parse()
            .map_err(|_| "Jumlah siklus harus 1–12.".to_owned())?;
    } else {
        settings.use_fixed_durations();
    }
    settings.display_name = view.get_display_name().trim().to_owned();
    settings.notifications = view.get_notifications();
    settings.alarm_enabled = view.get_alarm_enabled();
    settings.alarm_sound = crate::domain::alarm::AlarmSound::from_index(view.get_alarm_sound())?;
    settings.always_on_top = view.get_topmost();
    settings.floating_decorated = view.get_framed();
    if crate::platform::opacity_supported() {
        let opacity = view.get_floating_opacity();
        if !opacity.is_finite() || !(50.0..=100.0).contains(&opacity) {
            return Err("Opacity timer mini harus 50–100%.".into());
        }
        settings.floating_opacity = opacity.round() as u8;
    }
    settings.show_scene = view.get_scene();
    settings.export_directory = view.get_export_directory().trim().to_owned();
    settings.validate()?;
    app.data.settings = settings;
    if !app.active() {
        app.session = configured(&app.data, app.session.mode);
    }
    app.apply_window_settings();
    app.refresh();
    app.refresh_card()?;
    app.save(false);
    app.message("Preferensi diperbarui. Durasi sesi aktif tidak diubah.");
    Ok(())
}

fn parse_local(text: &str) -> Result<DateTime<Local>, String> {
    let naive = NaiveDateTime::parse_from_str(text.trim(), "%Y-%m-%d %H:%M:%S")
        .map_err(|_| "Waktu harus YYYY-MM-DD HH:MM:SS.".to_owned())?;
    Local.from_local_datetime(&naive).single().ok_or_else(|| {
        "Waktu lokal ambigu/tidak ada karena perubahan zona waktu. Pilih waktu lain.".into()
    })
}
fn save_edit(app: &mut Controller) -> Result<(), String> {
    let Some(w) = app.main.upgrade() else {
        return Ok(());
    };
    let v = w.global::<WorkspaceView>();
    let id: u64 = v
        .get_edit_id()
        .parse()
        .map_err(|_| "Entri tidak ditemukan.".to_owned())?;
    let start = parse_local(&v.get_edit_start())?;
    let end = parse_local(&v.get_edit_end())?;
    let seconds: f64 = v
        .get_edit_seconds()
        .trim()
        .parse()
        .map_err(|_| "Durasi fokus harus angka detik.".to_owned())?;
    if !seconds.is_finite() || !(0.001..=366.0 * 86400.0).contains(&seconds) {
        return Err("Durasi fokus tidak valid.".into());
    }
    let focus_ms = (seconds * 1000.0).round() as u64;
    let wall_ms = (end - start).num_milliseconds();
    if wall_ms < 0 || focus_ms > wall_ms as u64 + 1000 {
        return Err("Waktu selesai harus setelah mulai, dan durasi fokus tidak boleh melebihi rentang waktu.".into());
    }
    let note = v.get_edit_note().to_string();
    let session_name = v.get_edit_session_name().trim().to_owned();
    if session_name.chars().count() > 120 {
        return Err("Nama sesi maksimal 120 karakter.".into());
    }
    if note.chars().count() > 160 {
        return Err("Catatan maksimal 160 karakter.".into());
    }
    let record = app
        .data
        .records
        .iter_mut()
        .find(|r| r.id == id)
        .ok_or("Entri sudah dihapus.")?;
    *record = Record {
        id,
        mode: mode_from_index(v.get_edit_mode()),
        started_ms: start.timestamp_millis(),
        ended_ms: end.timestamp_millis(),
        local_day: start.date_naive(),
        focus_ms,
        outcome: if v.get_edit_completed() {
            Outcome::Completed
        } else {
            Outcome::Interrupted
        },
        note,
        session_name,
        tasks: record.tasks.clone(),
    };
    app.update_progress();
    v.set_dialog(0);
    app.save(false);
    Ok(())
}

fn restore_main(app: &Controller, hide_floating: bool) -> bool {
    let Some(w) = app.main.upgrade() else {
        return false;
    };
    if let Err(e) = w.show() {
        app.message(e.to_string());
        return false;
    }
    w.window().set_minimized(false);
    if hide_floating && let Some(w) = app.floating.upgrade() {
        report(w.hide());
    }
    true
}
fn report(result: Result<(), impl std::fmt::Display>) {
    if let Err(e) = result {
        eprintln!("Operation failed: {e}");
    }
}

fn toggle_session(session: &mut Session, now: Duration, uncertain: bool) -> bool {
    let requested_pause = session.state == State::Running;
    if uncertain {
        session.suspend();
    }
    session.sample(now);
    if requested_pause {
        session.pause(now);
    } else {
        session.start_or_resume(now);
    }
    session.state == State::Running
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pause_at_deadline_does_not_restart_completed_session() {
        let mut session = Session::new(Duration::from_secs(2)).unwrap();
        session.start_or_resume(Duration::ZERO);
        assert!(!toggle_session(&mut session, Duration::from_secs(2), false));
        assert_eq!(session.state, State::Completed);
    }
    #[test]
    fn pause_after_sleep_does_not_resume_automatically() {
        let mut session = Session::new(Duration::from_secs(10)).unwrap();
        session.start_or_resume(Duration::ZERO);
        session.sample(Duration::from_secs(1));
        assert!(!toggle_session(
            &mut session,
            Duration::from_secs(200),
            true
        ));
        assert_eq!(session.state, State::Paused);
        assert_eq!(
            session.remaining(Duration::from_secs(200)),
            Duration::from_secs(9)
        );
    }
}
