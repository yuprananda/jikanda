// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

use std::time::{Duration, Instant, SystemTime};
pub mod audio;

#[cfg(windows)]
pub const NOTIFICATION_APP_ID: &str = "YuPrananda.Jikanda";

/// Called on the I/O worker, never on the UI thread. OS acceptance does not
/// guarantee a visible banner: user notification settings still take priority.
pub fn show_notification(body: &str) -> Result<(), String> {
    #[cfg(windows)]
    windows::register_notification_identity()?;
    let mut notification = notify_rust::Notification::new();
    notification
        .appname("Jikanda")
        .summary("Jikanda")
        .body(body);
    #[cfg(target_os = "linux")]
    notification.hint(notify_rust::Hint::SuppressSound(true));
    #[cfg(windows)]
    notification.app_id(NOTIFICATION_APP_ID);
    notification
        .show()
        .map(|_| ())
        .map_err(|error| error.to_string())
}

pub fn opacity_supported() -> bool {
    cfg!(windows)
}

pub fn set_opacity(window: &slint::Window, percent: u8) -> Result<bool, String> {
    #[cfg(windows)]
    {
        windows::set_opacity(window, percent)
    }
    #[cfg(not(windows))]
    {
        let _ = (window, percent);
        Ok(false)
    }
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(windows)]
mod windows;

/// Detects uncertain wall/monotonic gaps; wall time never contributes focus time.
pub struct SleepMonitor {
    mono: Instant,
    wall: SystemTime,
    #[cfg(windows)]
    power: windows::PowerEvents,
    #[cfg(target_os = "linux")]
    power: linux::PowerEvents,
}

impl SleepMonitor {
    pub fn new() -> Self {
        Self {
            mono: Instant::now(),
            wall: SystemTime::now(),
            #[cfg(windows)]
            power: windows::PowerEvents::new(),
            #[cfg(target_os = "linux")]
            power: linux::PowerEvents::new(),
        }
    }

    pub fn uncertain_gap(&mut self) -> bool {
        let mono = Instant::now();
        let wall = SystemTime::now();
        let elapsed = mono.duration_since(self.mono);
        let suspicious = elapsed > Duration::from_secs(3)
            || wall.duration_since(self.wall).map_or(true, |delta| {
                delta.abs_diff(elapsed) > Duration::from_secs(2)
            });
        self.mono = mono;
        self.wall = wall;
        #[cfg(any(windows, target_os = "linux"))]
        let suspicious = self.power.take() || suspicious;
        suspicious
    }

    pub fn reset(&mut self) {
        self.mono = Instant::now();
        self.wall = SystemTime::now();
        #[cfg(any(windows, target_os = "linux"))]
        self.power.take();
    }
}

pub fn open_directory(path: &std::path::Path) -> Result<(), String> {
    #[cfg(windows)]
    let mut command = {
        use std::os::windows::process::CommandExt;
        let mut command = std::process::Command::new("explorer.exe");
        command.creation_flags(0x08000000);
        command
    };
    #[cfg(not(windows))]
    let mut command = std::process::Command::new("xdg-open");
    command
        .arg(path)
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Tidak dapat membuka folder: {e}"))
}

pub fn window_note() -> &'static str {
    if cfg!(target_os = "linux") && std::env::var_os("WAYLAND_DISPLAY").is_some() {
        "Wayland: pin mengikuti dukungan compositor. Seret header atau waktu untuk memindahkan timer."
    } else {
        "Timer mini meminta posisi selalu di atas. Seret header atau waktu untuk memindahkannya."
    }
}
