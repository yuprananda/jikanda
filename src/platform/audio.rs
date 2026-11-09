// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

use crate::domain::alarm::AlarmSound;
use std::path::Path;

fn wave(sound: AlarmSound) -> &'static [u8] {
    match sound {
        AlarmSound::SoftBell => include_bytes!("../../assets/audio/soft-bell.wav"),
        AlarmSound::DoubleChime => include_bytes!("../../assets/audio/double-chime.wav"),
        AlarmSound::BrightTone => include_bytes!("../../assets/audio/bright-tone.wav"),
    }
}

/// One short signal on the existing worker, with no idle audio thread.
pub fn play(sound: AlarmSound, directory: &Path) -> Result<(), String> {
    #[cfg(windows)]
    {
        use windows_sys::Win32::Media::Audio::{PlaySoundW, SND_MEMORY, SND_NODEFAULT, SND_SYNC};
        let _ = directory;
        // SAFETY: SND_MEMORY takes a WAVE image. Static bytes remain valid for
        // the entire synchronous call, which runs outside the UI thread.
        let played = unsafe {
            PlaySoundW(
                wave(sound).as_ptr().cast(),
                std::ptr::null_mut(),
                SND_MEMORY | SND_NODEFAULT | SND_SYNC,
            )
        };
        if played != 0 {
            Ok(())
        } else {
            Err("Alarm tidak dapat diputar. Periksa perangkat dan volume audio.".into())
        }
    }
    #[cfg(target_os = "linux")]
    {
        use std::{
            fs,
            io::Write,
            process::{Command, Stdio},
            time::{Duration, Instant},
        };
        let cache = directory.join("sounds");
        fs::create_dir_all(&cache).map_err(|error| error.to_string())?;
        let path = cache.join(format!("alarm-{}.wav", sound.index()));
        match fs::File::create_new(&path) {
            Ok(mut file) => {
                file.write_all(wave(sound))
                    .map_err(|error| error.to_string())?;
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                if fs::read(&path).map_err(|error| error.to_string())? != wave(sound) {
                    return Err("Cache alarm tidak valid. Hapus folder sounds di folder data lalu coba lagi.".into());
                }
            }
            Err(error) => return Err(error.to_string()),
        }
        for player in ["paplay", "aplay"] {
            let mut command = Command::new(player);
            if player == "aplay" {
                command.arg("-q");
            }
            let Ok(mut child) = command
                .arg(&path)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            else {
                continue;
            };
            let started = Instant::now();
            loop {
                match child.try_wait() {
                    Ok(Some(status)) if status.success() => return Ok(()),
                    Ok(Some(_)) => break,
                    Ok(None) if started.elapsed() < Duration::from_secs(4) => {
                        std::thread::sleep(Duration::from_millis(25))
                    }
                    _ => {
                        let _ = child.kill();
                        let _ = child.wait();
                        break;
                    }
                }
            }
        }
        Err("Alarm tidak tersedia. Periksa audio dan pasang paplay atau aplay pada Linux.".into())
    }
    #[cfg(not(any(windows, target_os = "linux")))]
    {
        let _ = (sound, directory);
        Err("Alarm belum tersedia pada platform ini.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn embedded_original_sounds_are_bounded_pcm_with_fades() {
        for sound in [
            AlarmSound::SoftBell,
            AlarmSound::DoubleChime,
            AlarmSound::BrightTone,
        ] {
            let bytes = wave(sound);
            assert_eq!(&bytes[..4], b"RIFF");
            assert_eq!(&bytes[8..12], b"WAVE");
            assert_eq!(u16::from_le_bytes(bytes[20..22].try_into().unwrap()), 1);
            assert_eq!(u16::from_le_bytes(bytes[22..24].try_into().unwrap()), 1);
            assert_eq!(u32::from_le_bytes(bytes[24..28].try_into().unwrap()), 44100);
            assert_eq!(u16::from_le_bytes(bytes[34..36].try_into().unwrap()), 16);
            let samples: Vec<i16> = bytes[44..]
                .chunks_exact(2)
                .map(|s| i16::from_le_bytes([s[0], s[1]]))
                .collect();
            assert!(samples.len() <= 44100 * 12 / 10);
            assert_eq!(samples[0], 0);
            assert!(samples.iter().any(|sample| sample.unsigned_abs() > 1000));
            assert!(samples.iter().all(|sample| sample.unsigned_abs() < 8000));
            assert!(samples.last().unwrap().unsigned_abs() < 20);
        }
    }
}
