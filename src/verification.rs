// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

//! Opt-in integration check, isolated from real user data and OS notifications.
use crate::{FloatingTimer, MainWindow, Palette, SessionView, WorkspaceView};
use slint::{ComponentHandle, Model, Timer, TimerMode};
use std::{
    cell::Cell,
    path::{Path, PathBuf},
    rc::Rc,
    time::Duration,
};

fn snapshot(path: &Path, window: &slint::Window) {
    crate::export::write_pixels(path, &window.take_snapshot().unwrap()).unwrap();
}

fn snapshot_responsive(main: &MainWindow, floating: &FloatingTimer, directory: &Path) {
    let directory = directory.join("responsive");
    std::fs::create_dir(&directory).unwrap();
    let view = main.global::<SessionView>();
    let work = main.global::<WorkspaceView>();
    let language = main.global::<crate::I18n>();
    let saved_size = main.window().size();
    let saved_language = language.get_indonesian();
    let saved_page = work.get_page();
    let saved_tab = work.get_preferences_tab();
    let saved = (
        view.get_time(),
        view.get_long_time(),
        view.get_status(),
        view.get_session_name(),
        view.get_mode(),
        view.get_mode_label(),
        view.get_mode_caption(),
    );
    let saved_tasks = work.get_tasks();
    let rows = vec![crate::TaskRow { id: "fixture".into(), title: "Review the entire release checklist and verify the desktop focus timer layout at the smallest supported window size".into(), completed: false, selected: true }];
    work.set_tasks(slint::ModelRc::new(slint::VecModel::from(rows)));
    view.set_time("1234:56:59".into());
    view.set_long_time(true);
    view.set_session_name("A deliberately long session name to check how the heading wraps or truncates without entering the neighbouring panel".into());
    view.set_status("Sesi selesai \u{b7} terima kasih sudah hadir".into());
    for indonesian in [false, true] {
        language.set_indonesian(indonesian);
        let label = if indonesian { "id" } else { "en" };
        for (width, height) in [(1000.0, 680.0), (1280.0, 800.0)] {
            main.window()
                .set_size(slint::LogicalSize::new(width, height));
            work.set_page(0);
            for mode in 0..3 {
                view.set_mode(mode);
                view.set_mode_label(
                    match mode {
                        1 => "Pomodoro",
                        2 => "Flow",
                        _ => "Deep Focus",
                    }
                    .into(),
                );
                view.set_mode_caption("Siklus 4 dari 4".into());
                snapshot(
                    &directory.join(format!("focus-{mode}-{width}-{label}.png")),
                    main.window(),
                );
            }
            if width == 1000.0 {
                for page in [1, 2, 3] {
                    work.set_page(page);
                    for tab in 0..(if page == 3 { 4 } else { 1 }) {
                        work.set_preferences_tab(tab);
                        snapshot(
                            &directory.join(format!("page-{page}-tab-{tab}-{label}.png")),
                            main.window(),
                        );
                        if page == 3 && tab == 0 {
                            main.window().dispatch_event(
                                slint::platform::WindowEvent::PointerScrolled {
                                    position: slint::LogicalPosition::new(650.0, 450.0),
                                    delta_x: 0.0,
                                    delta_y: -360.0,
                                },
                            );
                            snapshot(
                                &directory.join(format!("preferences-alarm-{label}.png")),
                                main.window(),
                            );
                            main.window().dispatch_event(
                                slint::platform::WindowEvent::PointerScrolled {
                                    position: slint::LogicalPosition::new(650.0, 450.0),
                                    delta_x: 0.0,
                                    delta_y: 1600.0,
                                },
                            );
                        }
                    }
                }
            }
        }
    }
    work.set_tasks(saved_tasks);
    work.set_page(saved_page);
    work.set_preferences_tab(saved_tab);
    language.set_indonesian(saved_language);
    main.window().set_size(saved_size);
    view.set_time(saved.0);
    view.set_long_time(saved.1);
    view.set_status(saved.2);
    view.set_session_name(saved.3);
    view.set_mode(saved.4);
    view.set_mode_label(saved.5);
    view.set_mode_caption(saved.6);
    let mini = floating.global::<SessionView>();
    let saved_mini = (mini.get_time(), mini.get_long_time());
    mini.set_time("1234:56:59".into());
    mini.set_long_time(true);
    snapshot(&directory.join("mini-long-time.png"), floating.window());
    mini.set_time(saved_mini.0);
    mini.set_long_time(saved_mini.1);
}

fn snapshot_card_layouts(directory: &Path) {
    let card = crate::CardWindow::new().unwrap();
    crate::i18n::bind(card.global::<crate::I18n>(), crate::i18n::Language::English);
    card.set_duration("5m 28s".into());
    card.set_caption("Jikanda Project".into());
    card.set_personal_name("Yu Prananda".into());
    card.set_period("09/11/2026".into());
    card.set_detail("Flow | Completed".into());
    card.set_tasks("\u{2713} Debugging\n\u{2713} Release".into());
    card.window()
        .dispatch_event(slint::platform::WindowEvent::ScaleFactorChanged { scale_factor: 1.0 });
    for (format, height, label) in [(0, 1920, "story"), (1, 1350, "feed")] {
        card.set_format(format);
        card.window()
            .set_size(slint::PhysicalSize::new(1080, height));
        for dark in [false, true] {
            card.set_purple(dark);
            snapshot(
                &directory.join(format!(
                    "layout-{label}-{}.png",
                    if dark { "dark" } else { "light" }
                )),
                card.window(),
            );
        }
    }
    card.set_caption("A long session title for an important creative project, with enough words to check wrapping and truncation in the card".into());
    card.set_personal_name("A deliberately long optional display name for layout review".into());
    card.set_duration("8784h 59m".into());
    card.set_tasks("\u{2713} Review a long task description that needs to wrap onto another line\n\u{2713} Prepare all release assets\n\u{2713} Test the final build\n\u{2713} Check documentation".into());
    snapshot(&directory.join("layout-feed-long.png"), card.window());
    card.set_format(0);
    card.window().set_size(slint::PhysicalSize::new(1080, 1920));
    snapshot(&directory.join("layout-story-long.png"), card.window());
}

#[cfg(windows)]
fn assert_native_opacity(window: &slint::Window, percent: u8) {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetLayeredWindowAttributes, LWA_ALPHA};
    let provider = window.window_handle();
    let handle = provider.window_handle().unwrap();
    let RawWindowHandle::Win32(handle) = handle.as_raw() else {
        panic!("Expected HWND")
    };
    let mut alpha = 0u8;
    let mut flags = 0u32;
    // SAFETY: this diagnostic runs on the UI thread with Slint's live HWND.
    assert_ne!(
        unsafe {
            GetLayeredWindowAttributes(
                handle.hwnd.get() as *mut _,
                std::ptr::null_mut(),
                &mut alpha,
                &mut flags,
            )
        },
        0
    );
    assert_eq!(flags & LWA_ALPHA, LWA_ALPHA);
    assert_eq!(alpha, (u16::from(percent) * 255 / 100) as u8);
}

fn snapshot_dialogs(main: &MainWindow, directory: &Path) {
    let directory = directory.join("dialogs");
    std::fs::create_dir(&directory).unwrap();
    let work = main.global::<WorkspaceView>();
    let saved = (
        work.get_dialog(),
        work.get_dialog_title(),
        work.get_dialog_detail(),
        work.get_notice(),
        work.get_busy(),
    );
    let palette = main.global::<Palette>();
    let language = main.global::<crate::I18n>();
    let original_dark = palette.get_dark();
    let original_language = language.get_indonesian();
    let original_size = main.window().size();
    main.window()
        .set_size(slint::LogicalSize::new(1000.0, 680.0));
    let cases = [
        (
            1,
            "stop",
            "Simpan waktu yang sudah kamu beri?",
            "Sesi dijeda. Simpan sebagai terputus, buang sesi ini, atau kembali untuk melanjutkan. Waktu istirahat tidak ikut dicatat.",
            false,
        ),
        (
            2,
            "quit",
            "Akhiri sesi dan keluar?",
            "Sesi dijeda. Simpan sebagai terputus, buang sesi ini, atau kembali untuk melanjutkan. Waktu istirahat tidak ikut dicatat.",
            false,
        ),
        (
            3,
            "delete-all",
            "Hapus seluruh riwayat?",
            "Semua sesi tersimpan dan catatannya dihapus. Preferensi tetap dipertahankan. Kartu PNG yang pernah diekspor tidak dihapus.",
            false,
        ),
        (
            4,
            "delete-entry",
            "Hapus entri ini?",
            "Entri dan catatannya akan dihapus dari riwayat lokal. Tindakan ini tidak dapat dibatalkan.",
            false,
        ),
        (
            5,
            "edit",
            "Koreksi sesi",
            "Durasi fokus boleh lebih pendek dari rentang waktu karena jeda/istirahat. Koreksi tanggal, durasi, catatan, atau status dengan sengaja.",
            false,
        ),
        (
            6,
            "save-failed",
            "Perubahan belum tersimpan",
            "Aplikasi belum dapat memastikan semua perubahan tersimpan. Coba simpan lagi, atau buang perubahan yang belum tersimpan untuk keluar. Data tidak direset.",
            false,
        ),
        (
            6,
            "saving",
            "Menyimpan dan keluar…",
            "Menunggu penulisan data lokal selesai.",
            true,
        ),
    ];
    for indonesian in [false, true] {
        language.set_indonesian(indonesian);
        for dark in [false, true] {
            palette.set_dark(dark);
            for &(kind, name, title, detail, busy) in &cases {
                work.set_dialog(kind);
                work.set_dialog_title(title.into());
                work.set_dialog_detail(detail.into());
                work.set_notice("".into());
                work.set_busy(busy);
                snapshot(
                    &directory.join(format!(
                        "{name}-{}-{}.png",
                        if indonesian { "id" } else { "en" },
                        if dark { "dark" } else { "light" }
                    )),
                    main.window(),
                );
            }
        }
        let error = crate::ErrorWindow::new().unwrap();
        crate::i18n::bind(
            error.global::<crate::I18n>(),
            if indonesian {
                crate::i18n::Language::Indonesian
            } else {
                crate::i18n::Language::English
            },
        );
        error.set_message(crate::i18n::translate(if indonesian { crate::i18n::Language::Indonesian } else { crate::i18n::Language::English }, "Data lokal rusak atau format tidak dikenali. Berkas tidak diubah; cadangkan sebelum memperbaiki.").into());
        error.show().unwrap();
        snapshot(
            &directory.join(if indonesian {
                "startup-error-id.png"
            } else {
                "startup-error-en.png"
            }),
            error.window(),
        );
        error.hide().unwrap();
    }
    work.set_dialog(saved.0);
    work.set_dialog_title(saved.1);
    work.set_dialog_detail(saved.2);
    work.set_notice(saved.3);
    work.set_busy(saved.4);
    palette.set_dark(original_dark);
    language.set_indonesian(original_language);
    main.window().set_size(original_size);
}

pub fn schedule(
    main: &MainWindow,
    floating: &FloatingTimer,
) -> Result<Option<Timer>, Box<dyn std::error::Error>> {
    if !std::env::args().any(|arg| arg == "--verify-native") {
        return Ok(None);
    }
    if main.global::<Palette>().get_dark() {
        main.global::<SessionView>().invoke_theme();
    }
    let directory = PathBuf::from("artifacts").join(format!(
        "native-{}-{}",
        std::process::id(),
        chrono::Utc::now().timestamp_millis()
    ));
    std::fs::create_dir_all(&directory)?;
    let main = main.as_weak();
    let floating = floating.as_weak();
    let step = Rc::new(Cell::new(0));
    let waits = Rc::new(Cell::new(0u32));
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_secs(1), move || {
        let main = main.upgrade().expect("main retained");
        let floating = floating.upgrade().expect("floating retained");
        let view = main.global::<SessionView>();
        let work = main.global::<WorkspaceView>();
        let n = step.get();
        step.set(n + 1);
        match n {
            0 => {
                assert!(!work.get_include_name());
                assert!(!work.get_include_tasks());
                work.invoke_add_task("Draft a paragraph".into());
                work.invoke_add_task("Read a chapter".into());
                assert_eq!(work.get_tasks().row_count(), 2);
                work.invoke_toggle_task("2".into());
                assert!(work.get_tasks().row_data(1).unwrap().completed);
                work.invoke_delete_task("2".into());
                assert_eq!(work.get_tasks().row_count(), 1);
                work.invoke_add_task("Review the draft".into());
                work.invoke_select_task("3".into());
                assert!(work.get_tasks().row_data(1).unwrap().selected);
                assert!(!main.global::<crate::I18n>().get_indonesian());
                assert_eq!(main.global::<crate::I18n>().invoke_tr("Mulai fokus".into()), "Start focus");
                snapshot(&directory.join("main-light.png"), main.window());
                work.invoke_language(true);
                assert!(floating.global::<crate::I18n>().get_indonesian());
                assert_eq!(main.global::<crate::I18n>().invoke_tr("Mulai fokus".into()), "Mulai fokus");
                snapshot(&directory.join("main-indonesian.png"), main.window());
                work.invoke_language(false);
                view.invoke_theme();
            }
            1 => {
                snapshot(&directory.join("main-dark.png"), main.window());
                view.invoke_custom_duration("0".into());
                assert_eq!(view.get_time(), "50:00");
                view.invoke_custom_duration("invalid".into());
                assert_eq!(view.get_time(), "50:00");
                view.invoke_custom_duration("10".into());
                assert_eq!(view.get_time(), "00:10");
                view.invoke_edit_session_name("Learn Rust".into());
                view.invoke_toggle();
                floating.show().unwrap();
            }
            2 => {
                assert!(floating.get_expanded());
                assert_eq!(floating.global::<SessionView>().get_session_name(), "Learn Rust");
                floating.global::<WorkspaceView>().invoke_toggle_task("1".into());
                assert!(work.get_tasks().row_data(0).unwrap().completed);
                view.invoke_edit_session_name("Wrong running name".into());
                work.invoke_new_session();
                assert_eq!(view.get_session_name(), "Learn Rust");
                assert_eq!(work.get_tasks().row_count(), 2);
                #[cfg(windows)] {
                    work.invoke_opacity(70.0);
                    assert_native_opacity(floating.window(), 70);
                    work.invoke_opacity(100.0);
                    assert_native_opacity(floating.window(), 100);
                }
                assert_eq!(view.get_time(), floating.global::<SessionView>().get_time());
                snapshot(&directory.join("floating-dark.png"), floating.window());
                floating.global::<SessionView>().invoke_toggle();
                assert_eq!(view.get_action(), "Lanjutkan");
            }
            3 => {
                assert_eq!(view.get_action(), "Lanjutkan");
                work.invoke_language(true);
                snapshot(&directory.join("floating-indonesian.png"), floating.window());
                work.invoke_language(false);
                view.invoke_theme();
                snapshot(&directory.join("floating-light.png"), floating.window());
                view.invoke_toggle();
            }
            4 => {
                view.invoke_theme();
                main.window()
                    .set_size(slint::LogicalSize::new(1000.0, 680.0));
            }
            5 => {
                snapshot(&directory.join("main-minimum-running.png"), main.window());
                assert!(view.get_remaining() > 0.0 && view.get_remaining() < 1.0);
                main.window()
                    .set_size(slint::LogicalSize::new(1120.0, 720.0));
            }
            14 => {
                assert_eq!(view.get_time(), "00:00");
                assert!(view.get_completed());
                assert_eq!(view.get_remaining(), 0.0);
                snapshot(&directory.join("floating-completed.png"), floating.window());
                work.invoke_navigate(1);
                assert_eq!(work.get_history().row_count(), 1);
            }
            15 => {
                snapshot(&directory.join("history.png"), main.window());
                work.invoke_edit("1".into());
                assert_eq!(work.get_dialog(), 5);
                snapshot(&directory.join("edit-dialog.png"), main.window());
            }
            16 => {
                work.set_display_name("Nama pilihan".into());
                work.invoke_save_settings();
                work.set_edit_seconds("8".into());
                work.set_edit_session_name("Learn Rust revised".into());
                work.invoke_save_edit();
                assert_eq!(work.get_dialog(), 0);
                work.invoke_navigate(2);
            }
            17 => {
                assert!(work.get_card_ready());
                assert_eq!(work.get_card_duration(), "8 detik");
                assert_eq!(work.get_card_caption(), "Learn Rust revised");
                assert_eq!(work.get_card_name(), "Nama pilihan");
                work.set_include_name(false);

                snapshot(&directory.join("card-paper.png"), main.window());
                main.window().set_size(slint::LogicalSize::new(1000.0, 680.0));
                work.invoke_export_card();
            }
            18 => {
                snapshot_card_layouts(&directory);
                work.set_card_template(1);
                work.set_card_format(1);
                work.set_include_name(true);
                work.set_include_tasks(true);
                assert!(work.get_card_tasks().contains("Draft a paragraph"));
                snapshot(&directory.join("card-feed.png"), main.window());
                work.invoke_export_card();
                work.invoke_language(true);
                snapshot(&directory.join("card-feed-indonesian.png"), main.window());

                work.invoke_refresh_card();
                main.window().dispatch_event(slint::platform::WindowEvent::PointerScrolled {
                    position: slint::LogicalPosition::new(380.0, 340.0), delta_x: 0.0, delta_y: -600.0,
                });
                snapshot(&directory.join("cards-scroll-bottom.png"), main.window());
                main.window().dispatch_event(slint::platform::WindowEvent::PointerScrolled {
                    position: slint::LogicalPosition::new(380.0, 340.0), delta_x: 0.0, delta_y: 1200.0,
                });
                snapshot(&directory.join("cards-scroll-top.png"), main.window());
                work.invoke_language(false);
                work.invoke_navigate(3);
            }
            19 => {
                work.set_alarm_enabled(false);
                work.set_alarm_sound(2);
                work.set_cycles("0".into());
                work.invoke_save_settings();
                assert!(work.get_notice().contains("siklus"));
                work.set_cycles("2".into());
                work.set_interval_minutes("0.017".into());
                work.set_break_minutes("0.017".into());
                work.set_notifications(false);
                work.invoke_save_settings();
                snapshot(&directory.join("preferences.png"), main.window());
                #[cfg(windows)] {
                    work.invoke_opacity(80.0);
                    assert_native_opacity(floating.window(), 80);
                    snapshot(&directory.join("preferences-opacity.png"), main.window());
                    work.invoke_opacity(50.0);
                    assert_native_opacity(floating.window(), 50);
                    work.invoke_opacity(100.0);
                    assert_native_opacity(floating.window(), 100);
                    work.invoke_opacity(95.0);
                    assert_native_opacity(floating.window(), 95);
                    work.invoke_opacity(80.0);
                }
            }
            20 => {
                work.invoke_test_alarm();
                assert!(!work.get_alarm_pending());
                assert!(work.get_alarm_status().contains("Mode pengujian"));
                work.invoke_test_notification();
                assert!(!work.get_notification_pending());
                assert!(work.get_notification_status().contains("Mode pengujian"));
                let original_language = main.global::<crate::I18n>().get_indonesian();
                for indonesian in [false, true] {
                    main.global::<crate::I18n>().set_indonesian(indonesian);
                    for (index, name) in ["general", "mini", "storage", "about"].iter().enumerate() {
                        work.set_preferences_tab(index as i32);
                        snapshot(&directory.join(format!("preferences-{name}-{}.png", if indonesian { "id" } else { "en" })), main.window());
                    }
                }
                main.global::<crate::I18n>().set_indonesian(original_language);
                view.invoke_theme();
                for (index, name) in ["general", "mini", "storage", "about"].iter().enumerate() {
                    work.set_preferences_tab(index as i32);
                    snapshot(&directory.join(format!("preferences-{name}-light.png")), main.window());
                }
                view.invoke_theme();
                work.set_preferences_tab(0);
                work.invoke_navigate(0);
                view.invoke_select_mode(1);
                snapshot(&directory.join("pomodoro-tasks.png"), main.window());
                main.global::<crate::I18n>().set_indonesian(true);
                snapshot(&directory.join("pomodoro-tasks-id.png"), main.window());
                main.global::<crate::I18n>().set_indonesian(false);
                view.invoke_toggle();
            }
            22 => {
                if view.get_mode_label() != "ISTIRAHAT" && view.get_running() && waits.get() < 3 {
                    waits.set(waits.get() + 1); step.set(22); return;
                }
                waits.set(0);
                assert_eq!(
                    view.get_mode_label(),
                    "ISTIRAHAT",
                    "time={}, status={}, active={}, running={}",
                    view.get_time(),
                    view.get_status(),
                    view.get_active(),
                    view.get_running()
                );
                assert_eq!(view.get_action(), "Mulai istirahat");
                view.invoke_toggle();
            }
            24 => {
                if view.get_action() != "Mulai fokus" && view.get_running() && waits.get() < 3 {
                    waits.set(waits.get() + 1); step.set(24); return;
                }
                waits.set(0);
                assert_eq!(view.get_action(), "Mulai fokus");
                view.invoke_toggle();
            }
            26 => {
                if !view.get_completed() && view.get_running() && waits.get() < 3 {
                    waits.set(waits.get() + 1); step.set(26); return;
                }
                waits.set(0);
                assert!(view.get_completed());
                assert_eq!(work.get_history().row_count(), 2);
                assert!(!floating.get_expanded());
                work.invoke_new_session();
                assert_eq!(work.get_dialog(), 7);
                work.invoke_dialog_save();
                assert_eq!(work.get_tasks().row_count(), 0);
                assert_eq!(view.get_session_name(), "");
                view.invoke_edit_session_name("Write a blog".into());
                work.invoke_add_task("Plan the outline".into());
                view.invoke_select_mode(2);
                view.invoke_toggle();
            }
            27 => {
                floating.global::<SessionView>().invoke_toggle();
                assert_eq!(view.get_action(), "Lanjutkan");
            }
            28 => {
                view.invoke_finish_flow();
                assert!(view.get_completed());
                assert_eq!(work.get_history().row_count(), 3);
            }
            29 => {
                view.invoke_select_mode(0);
                view.invoke_custom_duration("10".into());
                view.invoke_toggle();
            }
            30 => {
                view.invoke_stop();
                assert_eq!(work.get_dialog(), 1);
                floating.global::<SessionView>().invoke_toggle();
                assert!(!view.get_running(), "Floating controls must not bypass the stop confirmation");
                floating.global::<WorkspaceView>().invoke_toggle_task("1".into());
                assert!(!work.get_tasks().row_data(0).unwrap().completed, "Mini tasks must respect confirmation dialogs");
            }
            31 => {
                snapshot(&directory.join("stop-confirmation.png"), main.window());
                work.invoke_dialog_discard();
                assert_eq!(work.get_history().row_count(), 3);
            }
            32 => {
                view.invoke_select_mode(2);
                view.invoke_toggle();
            }
            33 => {
                view.invoke_stop();
                assert_eq!(work.get_dialog(), 1);
            }
            34 => {
                work.invoke_dialog_save();
                assert_eq!(work.get_history().row_count(), 4);
                assert!(!view.get_completed());
            }
            35 => {
                work.invoke_share_session("4".into());
                assert!(work.get_card_ready());
                assert!(work.get_card_detail().contains("Interrupted"));
                work.invoke_share_session("1".into());
                assert_eq!(work.get_card_time(), "8s");
                assert_eq!(work.get_card_caption(), "Learn Rust revised");
                snapshot(&directory.join("card-single-session.png"), main.window());
                work.invoke_select_card(1);
                assert_eq!(work.get_card_id(), "3");
                assert_eq!(work.get_card_caption(), "Write a blog");
                work.invoke_navigate(1);
                snapshot(&directory.join("history-all-modes.png"), main.window());
                snapshot_responsive(&main, &floating, &directory);
            }
            36 => {
                work.invoke_delete("4".into());
                assert_eq!(work.get_dialog(), 4);
                work.invoke_dialog_save();
                assert_eq!(work.get_history().row_count(), 3);
                work.invoke_share_session("4".into());
                assert!(!work.get_card_ready(), "Deleted sessions cannot be exported");
                work.invoke_share_session("1".into());
                assert_eq!(work.get_card_time(), "8s");
                work.invoke_navigate(1);
                main.window().set_size(slint::LogicalSize::new(1000.0, 680.0));
            }
            37 => {
                work.invoke_clear_history();
                assert_eq!(work.get_dialog(), 3);
                snapshot_dialogs(&main, &directory);
                work.invoke_dialog_cancel();
                assert_eq!(work.get_history().row_count(), 3);
            }
            38 => {
                assert!(!work.get_unsaved());
                std::fs::write(directory.join("PASSED.txt"), "All native integration assertions passed. Quit and persisted data must also be checked by the runner.\n").unwrap();
                println!("Native verification passed: {}", directory.display());
                view.invoke_quit();
            }
            39..=42 => view.invoke_quit(),
            43 => panic!("Quit did not flush data or finish"),
            _ => {}
        }
    });
    Ok(Some(timer))
}
