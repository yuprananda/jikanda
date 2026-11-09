// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

use crate::domain::progress::Data;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub struct Store {
    pub directory: PathBuf,
    _lock: File,
}

pub fn default_directory() -> Result<PathBuf, String> {
    #[cfg(windows)]
    {
        std::env::var_os("LOCALAPPDATA")
            .map(|p| PathBuf::from(p).join("Jikanda"))
            .ok_or_else(|| "LOCALAPPDATA tidak tersedia.".into())
    }
    #[cfg(not(windows))]
    {
        if let Some(path) = std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
        {
            return Ok(path.join("jikanda"));
        }
        std::env::var_os("HOME")
            .map(|p| PathBuf::from(p).join(".local/share/jikanda"))
            .ok_or_else(|| "HOME tidak tersedia.".into())
    }
}

impl Store {
    pub fn open(directory: PathBuf) -> Result<(Self, Data), String> {
        let mut builder = fs::DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder
            .create(&directory)
            .map_err(|e| format!("Tidak dapat membuka folder data: {e}"))?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(directory.join("instance.lock"))
            .map_err(|e| format!("Tidak dapat membuka lock: {e}"))?;
        lock.try_lock().map_err(|_| {
            "Folder data sedang digunakan Jikanda lain atau tidak dapat dikunci.".to_owned()
        })?;
        let path = directory.join("state.json");
        let data = match File::open(&path) {
            Ok(file) => {
                let mut bytes = Vec::new();
                file.take(16 * 1024 * 1024 + 1)
                    .read_to_end(&mut bytes)
                    .map_err(|e| format!("Gagal membaca data: {e}"))?;
                if bytes.len() > 16 * 1024 * 1024 {
                    return Err("Berkas data melebihi batas 16 MiB; tidak diubah.".into());
                }
                let data: Data = serde_json::from_slice(&bytes).map_err(|_| "Data lokal rusak atau format tidak dikenali. Berkas tidak diubah; cadangkan sebelum memperbaiki.".to_owned())?;
                data.validate()?;
                data
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Data::default(),
            Err(e) => return Err(format!("Tidak dapat membaca data lokal: {e}")),
        };
        Ok((
            Self {
                directory,
                _lock: lock,
            },
            data,
        ))
    }

    pub fn save(&self, data: &Data, generation: u64) -> Result<(), String> {
        data.validate()?;
        let bytes = serde_json::to_vec_pretty(data).map_err(|e| e.to_string())?;
        if bytes.len() > 16 * 1024 * 1024 {
            return Err("Berkas data melebihi batas 16 MiB; tidak diubah.".into());
        }
        let temporary = self
            .directory
            .join(format!("state-{}-{generation}.tmp", std::process::id()));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options
            .open(&temporary)
            .map_err(|e| format!("Tidak dapat menulis file sementara: {e}"))?;
        let result = (|| {
            file.write_all(&bytes)?;
            file.sync_all()?;
            drop(file);
            fs::rename(&temporary, self.directory.join("state.json"))?;
            #[cfg(unix)]
            File::open(&self.directory)?.sync_all()?;
            Ok::<(), std::io::Error>(())
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result.map_err(|e| format!("Data belum tersimpan: {e}. Gunakan Coba simpan lagi."))
    }
}

pub fn valid_export_directory(path: &Path) -> Result<(), String> {
    if !path.is_absolute() {
        return Err("Folder ekspor harus berupa path absolut.".into());
    }
    fs::create_dir_all(path).map_err(|e| format!("Tidak dapat membuat folder ekspor: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn directory(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "jikanda-{name}-{}-{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap()
        ))
    }
    fn cleanup(dir: PathBuf) {
        let canonical = fs::canonicalize(&dir).unwrap();
        let temporary = fs::canonicalize(std::env::temp_dir()).unwrap();
        assert_eq!(canonical.parent(), Some(temporary.as_path()));
        assert!(
            canonical
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("jikanda-")
        );
        fs::remove_dir_all(canonical).unwrap();
    }
    #[test]
    fn atomic_roundtrip_lock_and_corruption_preservation() {
        let dir = directory("store");
        let (store, mut data) = Store::open(dir.clone()).unwrap();
        assert!(Store::open(dir.clone()).is_err());
        data.settings.dark = false;
        data.tasks.add("A local session task").unwrap();
        store.save(&data, 1).unwrap();
        store.save(&data, 2).unwrap();
        drop(store);
        let (store, loaded) = Store::open(dir.clone()).unwrap();
        assert_eq!(loaded, data);
        drop(store);
        fs::write(dir.join("state.json"), b"broken").unwrap();
        assert!(Store::open(dir.clone()).is_err());
        assert_eq!(fs::read(dir.join("state.json")).unwrap(), b"broken");
        // Delete only the explicitly constructed, isolated test directory.
        cleanup(dir);
    }
    #[test]
    fn invalid_snapshot_does_not_replace_existing_file() {
        let dir = directory("invalid");
        let (store, mut data) = Store::open(dir.clone()).unwrap();
        store.save(&data, 1).unwrap();
        let before = fs::read(dir.join("state.json")).unwrap();
        data.settings.cycles = 0;
        assert!(store.save(&data, 2).is_err());
        assert_eq!(fs::read(dir.join("state.json")).unwrap(), before);
        drop(store);
        cleanup(dir);
    }
}
