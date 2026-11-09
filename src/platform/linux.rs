// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

pub struct PowerEvents {
    changed: Arc<AtomicBool>,
}
impl PowerEvents {
    pub fn new() -> Self {
        let changed = Arc::new(AtomicBool::new(false));
        let signal = changed.clone();
        // Blocking D-Bus receive sleeps in the kernel; no periodic polling.
        let _ = std::thread::Builder::new()
            .name("jikanda-power".into())
            .spawn(move || {
                let result = (|| -> zbus::Result<()> {
                    let connection = zbus::blocking::Connection::system()?;
                    let proxy = zbus::blocking::Proxy::new(
                        &connection,
                        "org.freedesktop.login1",
                        "/org/freedesktop/login1",
                        "org.freedesktop.login1.Manager",
                    )?;
                    for _message in proxy.receive_signal("PrepareForSleep")? {
                        signal.store(true, Ordering::Release);
                    }
                    Ok(())
                })();
                if result.is_err() {
                    eprintln!("logind unavailable; using timing-gap fallback");
                }
            });
        Self { changed }
    }
    pub fn take(&self) -> bool {
        self.changed.swap(false, Ordering::AcqRel)
    }
}
