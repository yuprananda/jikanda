# Dependensi dan lisensi — MVP internal

Maintainer memilih **GPL-3.0-only** pada 6 November 2026. Slint digunakan melalui
opsi GPLv3, bukan Royalty-free atau lisensi komersial. Prototipe visual membundel Plus Jakarta
Sans (SIL OFL 1.1, file lisensi disertakan), crop brand sheet yang disediakan
maintainer, serta ilustrasi latar hasil built-in imagegen. Provenance dan prompt
dicatat di `docs/visual-prototype.md`; aset brand belum menjadi master produksi.

- `slint = 1.18.1`: UI, event loop, Winit, software renderer, accessibility.
  Default features dimatikan; Qt, tray, dan renderer GPU tidak dipilih.
- `slint-build = 1.18.1`: kompilasi `.slint` saat build, bukan interpreter runtime.
- `png 0.18`: encoding RGBA hasil render menjadi PNG lokal; MIT OR Apache-2.0.
- `windows-sys 0.61` (hanya Windows): binding notifikasi suspend/resume, opacity, dan registry identitas notifikasi per-user;
  MIT OR Apache-2.0. Tidak membutuhkan runtime async atau thread polling aplikasi.
- `serde 1` dan `serde_json 1`: serialisasi JSON berversi; MIT OR Apache-2.0.
- `chrono 0.4`: timestamp, tanggal lokal dan rentang kalender; MIT OR Apache-2.0.
- `notify-rust 4.18`: notifikasi desktop native; MIT OR Apache-2.0. Binding Windows
  transitif `tauri-winrt-notification` bukan framework/runtime aplikasi Tauri.
- `zbus 5.19` (Linux): listener logind lewat D-Bus; MIT. Runtime D-Bus hanya dipakai
  untuk integrasi OS lokal, tidak untuk backend/cloud atau traffic history.

Cargo.lock dikomitkan untuk reproduksibilitas bila repositori Git dibuat.
Lockfile dapat memuat dependensi opsional dan platform lain; itu bukan daftar
semua pustaka yang berjalan atau terhubung ke executable Windows.

Opsi GPLv3 Slint sudah dipilih. Lisensi upstream lain tetap dipertahankan; keputusan
ini bukan audit lengkap dependensi transitif atau clearance aset. Sebelum distribusi,
siapkan Corresponding Source yang lengkap dan pemenuhan notice, bukan sekadar link umum.

Paket Windows internal menyertakan inventory hasil cargo metadata terfilter target
serta teks lisensi upstream yang tersedia dalam paket Cargo. Inventory mencakup
build dependencies dan tidak otomatis memilih salah satu opsi lisensi majemuk.
Audit final serta kelengkapan teks/atribusi masih diperlukan sebelum rilis publik.

Sumber resmi:
- https://slint.dev/terms-and-conditions
- https://docs.rs/slint/1.18.1/slint/docs/cargo_features/index.html
- https://docs.rs/png/0.18.1/png/
- https://github.com/microsoft/windows-rs
# Akses handle untuk opacity

`raw-window-handle` 0.6 digunakan langsung hanya pada Windows, bersama fitur stabil
Slint `raw-window-handle-06`, untuk memperoleh HWND milik timer mini. Crate ini
sudah ada sebagai dependensi transitif; tidak menambahkan renderer atau thread.
Win32 opacity memakai `windows-sys` yang sudah dipakai proyek. Linux tetap opaque.


Alarm memakai WAV hasil sintesis asli (GPL-3.0-only) yang disematkan. Tidak ada crate
atau pustaka audio baru. Windows memakai fitur Win32_Media_Audio pada windows-sys;
Linux mencoba utilitas paplay/aplay lokal jika tersedia, tanpa instalasi otomatis.
