// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    ffi::c_void,
    ptr,
    sync::atomic::{AtomicBool, Ordering},
};
use windows_sys::Win32::{
    System::Power::{
        DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS, PowerRegisterSuspendResumeNotification,
        PowerUnregisterSuspendResumeNotification,
    },
    UI::WindowsAndMessaging::{
        DEVICE_NOTIFY_CALLBACK, PBT_APMRESUMEAUTOMATIC, PBT_APMRESUMESUSPEND, PBT_APMSUSPEND,
    },
};

pub fn register_notification_identity() -> Result<(), String> {
    register_display_name(&format!(
        "Software\\Classes\\AppUserModelId\\{}",
        super::NOTIFICATION_APP_ID
    ))
}

fn register_display_name(path: &str) -> Result<(), String> {
    use windows_sys::Win32::System::Registry::{
        HKEY_CURRENT_USER, KEY_SET_VALUE, REG_OPTION_NON_VOLATILE, REG_SZ, RegCloseKey,
        RegCreateKeyExW, RegSetValueExW,
    };
    let path: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
    let name: Vec<u16> = "DisplayName".encode_utf16().chain(Some(0)).collect();
    let value: Vec<u16> = "Jikanda".encode_utf16().chain(Some(0)).collect();
    let mut key = ptr::null_mut();
    // SAFETY: all strings are terminated UTF-16. Only this app's per-user
    // identity is written; notification permissions and system policies are untouched.
    unsafe {
        let error = RegCreateKeyExW(
            HKEY_CURRENT_USER,
            path.as_ptr(),
            0,
            ptr::null(),
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE,
            ptr::null(),
            &mut key,
            ptr::null_mut(),
        );
        if error != 0 {
            return Err(format!(
                "Registrasi identitas notifikasi gagal (Windows {error})."
            ));
        }
        let error = RegSetValueExW(
            key,
            name.as_ptr(),
            0,
            REG_SZ,
            value.as_ptr().cast(),
            (value.len() * size_of::<u16>()) as u32,
        );
        RegCloseKey(key);
        if error != 0 {
            return Err(format!(
                "Registrasi identitas notifikasi gagal (Windows {error})."
            ));
        }
    }
    Ok(())
}

pub fn set_opacity(window: &slint::Window, percent: u8) -> Result<bool, String> {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GWL_EXSTYLE, GetWindowLongPtrW, LWA_ALPHA, SetLayeredWindowAttributes, SetWindowLongPtrW,
        WS_EX_LAYERED,
    };
    let provider = window.window_handle();
    let Ok(handle) = provider.window_handle() else {
        return Ok(false);
    };
    let RawWindowHandle::Win32(handle) = handle.as_raw() else {
        return Ok(false);
    };
    let hwnd = handle.hwnd.get() as *mut c_void;
    // SAFETY: Slint owns this live HWND; called only on its UI thread. Preserve
    // all existing flags and use constant alpha, with no click-through behavior.
    unsafe {
        let flags = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        if flags & WS_EX_LAYERED as isize == 0 {
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, flags | WS_EX_LAYERED as isize);
        }
        let alpha = (u16::from(percent.clamp(50, 100)) * 255 / 100) as u8;
        if SetLayeredWindowAttributes(hwnd, 0, alpha, LWA_ALPHA) == 0 {
            if flags & WS_EX_LAYERED as isize == 0 {
                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, flags);
            }
            return Err("Opacity tidak tersedia; timer tetap dapat digunakan.".into());
        }
    }
    Ok(true)
}

pub struct PowerEvents {
    handle: *mut c_void,
    changed: Box<AtomicBool>,
}

impl PowerEvents {
    pub fn new() -> Self {
        let changed = Box::new(AtomicBool::new(false));
        let mut parameters = DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
            Callback: Some(on_power),
            Context: (&*changed as *const AtomicBool).cast_mut().cast(),
        };
        let mut handle = ptr::null_mut();
        // SAFETY: parameters is valid during registration; the boxed context remains
        // allocated until after synchronous unregistration in Drop.
        let error = unsafe {
            PowerRegisterSuspendResumeNotification(
                DEVICE_NOTIFY_CALLBACK,
                (&mut parameters as *mut DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS).cast(),
                &mut handle,
            )
        };
        if error != 0 {
            eprintln!(
                "Power notification registration failed ({error}); using timing-gap fallback"
            );
        }
        Self { handle, changed }
    }

    pub fn take(&self) -> bool {
        self.changed.swap(false, Ordering::AcqRel)
    }
}

unsafe extern "system" fn on_power(context: *const c_void, event: u32, _: *const c_void) -> u32 {
    if matches!(
        event,
        PBT_APMSUSPEND | PBT_APMRESUMEAUTOMATIC | PBT_APMRESUMESUSPEND
    ) {
        // SAFETY: PowerEvents owns the boxed AtomicBool until registration ends.
        unsafe { &*context.cast::<AtomicBool>() }.store(true, Ordering::Release);
    }
    0
}

impl Drop for PowerEvents {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            // SAFETY: handle was returned by successful registration and is used once.
            let error = unsafe { PowerUnregisterSuspendResumeNotification(self.handle as isize) };
            if error != 0 {
                // Keep the callback context alive if Windows could still reference it.
                let context =
                    std::mem::replace(&mut self.changed, Box::new(AtomicBool::new(false)));
                Box::leak(context);
                eprintln!("Power notification unregistration failed ({error})");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_identity_registration_is_idempotent() {
        use windows_sys::Win32::System::Registry::{
            HKEY_CURRENT_USER, RRF_RT_REG_SZ, RegDeleteKeyW, RegGetValueW,
        };
        let path = format!(
            "Software\\Jikanda\\Tests\\Notification-{}",
            std::process::id()
        );
        register_display_name(&path).unwrap();
        register_display_name(&path).unwrap();
        let path: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
        let name: Vec<u16> = "DisplayName".encode_utf16().chain(Some(0)).collect();
        let mut value = [0u16; 32];
        let mut bytes = size_of_val(&value) as u32;
        // SAFETY: buffers are live and their byte lengths are supplied correctly.
        // Only the isolated test key is read and removed, never OS preferences.
        unsafe {
            assert_eq!(
                RegGetValueW(
                    HKEY_CURRENT_USER,
                    path.as_ptr(),
                    name.as_ptr(),
                    RRF_RT_REG_SZ,
                    ptr::null_mut(),
                    value.as_mut_ptr().cast(),
                    &mut bytes
                ),
                0
            );
            assert_eq!(RegDeleteKeyW(HKEY_CURRENT_USER, path.as_ptr()), 0);
        }
        let count = value.iter().position(|value| *value == 0).unwrap();
        assert_eq!(String::from_utf16(&value[..count]).unwrap(), "Jikanda");
    }

    #[test]
    fn power_callback_marks_suspend_and_resume_only() {
        let flag = AtomicBool::new(false);
        let context = (&flag as *const AtomicBool).cast();
        unsafe {
            on_power(context, PBT_APMSUSPEND, ptr::null());
        }
        assert!(flag.swap(false, Ordering::AcqRel));
        unsafe {
            on_power(context, PBT_APMRESUMEAUTOMATIC, ptr::null());
        }
        assert!(flag.swap(false, Ordering::AcqRel));
        unsafe {
            on_power(context, 0, ptr::null());
        }
        assert!(!flag.load(Ordering::Acquire));
    }
}
