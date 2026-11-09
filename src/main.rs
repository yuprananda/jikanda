// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod app;
mod domain;
mod export;
mod i18n;
mod persistence;
mod platform;
mod verification;
mod worker;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|arg| arg == "--probe-notification") {
        platform::show_notification("Jikanda notification test. No session data is included.")?;
        println!(
            "Notification request accepted by the desktop OS; banner visibility is controlled by OS settings."
        );
        return Ok(());
    }
    if let Some(index) =
        std::env::args().find_map(|arg| arg.strip_prefix("--probe-alarm=").map(str::to_owned))
    {
        let sound = domain::alarm::AlarmSound::from_index(index.parse()?)?;
        let directory = std::env::temp_dir().join("jikanda-alarm-probe");
        platform::audio::play(sound, &directory)?;
        println!("Alarm playback finished.");
        return Ok(());
    }
    app::run()
}
