# Kesiapan rilis Jikanda 0.1.0

Status: **build internal untuk review**, bukan rilis publik yang telah disetujui.
Permintaan maintainer mencakup implementasi Phase 1–5; kemampuan yang dapat diuji
di mesin Windows ini diimplementasikan dan diverifikasi. Daftar di bawah membedakan
kode yang tersedia dari pengujian/keputusan yang belum dapat ditutup.

## Cakupan implementasi

- Phase 1: tokens, komponen custom, light/dark, font berlisensi, main/floating;
  mascot/wordmark tetap crop asli dari sheet, belum SVG master produksi.
- Phase 2: Interval dengan konfigurasi fokus/istirahat/siklus, Deep Focus, Flow;
  start/pause/resume/stop, completion eksplisit, sleep policy, notifikasi OS dan
  fallback visual. Antarsegmen menunggu pengguna; tidak ada kredit break.
- Phase 3: JSON lokal berversi, atomic write, single-instance lock, history,
  koreksi/hapus, ringkasan harian dan tujuh hari, data path discoverable.
- Phase 4: dua template PNG per sesi tersimpan, nama tampilan opt-in,
  kalimat opsional, preview dari komponen yang sama, encoding pada worker.
- Phase 5: pemeriksaan otomatis, skrip paket Windows/Linux internal, checksum,
  inventory lisensi, instalasi shortcut Windows, CI dua OS, dokumentasi instalasi,
  kontribusi, privasi, changelog, serta daftar validasi manual.

## Gerbang sebelum rilis publik

1. **Lisensi aplikasi diputuskan:** GPL-3.0-only, disetujui maintainer 6 November
   2026. LICENSE, SPDX dan metadata Cargo sudah ditetapkan; Slint memakai opsi GPLv3.
   Gerbang yang masih terbuka adalah kelengkapan Corresponding Source dan audit
   distribusi, bukan lagi pemilihan lisensi. Lihat LICENSING.md.
2. **Brand:** crop raster masih provisional. SVG master, geometri final, wordmark,
   palette dan micro-icon 16/24/32/48/64 px memerlukan review sesuai AGENTS.md §6A.
   Jangan mengklaim nama sudah lolos trademark atau artwork sudah legal-cleared.
3. **Linux:** belum ada distro WSL pada mesin ini. Build, D-Bus/logind, notifikasi,
   X11 dan Wayland belum dijalankan nyata. CI tersedia tetapi belum dijalankan remote.
4. **OS nyata:** sleep/hibernate, drag fisik, multi-monitor/hotplug, keyboard nyata,
   pembaca layar, DPI 100/150/200% dan startup cold/first-paint masih perlu uji.
5. **Notifikasi Windows:** identitas AppUserModelID YuPrananda.Jikanda kini
   diregistrasi per-user saat mengirim, termasuk executable portable. Probe nyata
   9 November 2026 diterima OS dan maintainer mengonfirmasi banner muncul.
   Izin OS/Do Not Disturb tetap dihormati. Tombol uji menampilkan status penerimaan
   atau kegagalan. Aktivasi klik toast belum membawa jendela ke depan.
6. **Audit distribusi:** tinjau pilihan Slint yang tepat, seluruh lisensi transitif,
   teks/atribusi, font, dan aset. Inventory otomatis adalah bahan review, bukan
   pengganti audit atau pernyataan kompatibilitas hukum final.

## Perilaku platform dan fallback

### Arah visual terbaru

Preferences memiliki empat tab: General (bahasa, tema, notifikasi), Mini timer
(topmost, bingkai, opacity), Storage (ekspor dan lokasi data), serta About
(identitas aplikasi, lisensi, atribusi Slint). Footer simpan berada di luar scroll;
bahasa/tema/opacity tetap tersimpan langsung, perubahan lain memakai tombol simpan.

Timer mini kini 228×112 logical px; logo sekitar 74×20 px. Preferences menyediakan
opacity 50–100% pada Windows melalui WS_EX_LAYERED/LWA_ALPHA. Ini alpha seluruh
jendela, bukan blur atau click-through. Pemilihan 100% mengembalikan alpha 255.
Handle native diterapkan setelah window ditampilkan; kegagalan mempertahankan
fallback opaque. Linux belum mengimplementasikan opacity: slider dinonaktifkan
dengan penjelasan, tanpa mengklaim dukungan X11/Wayland. Pengujian Windows membaca
kembali alpha native melalui GetLayeredWindowAttributes dan mengecek persistensi.

Halaman Cards terbaru memisahkan pengaturan dari pratinjau yang selalu muat utuh.
Area scroll menyisakan gutter kanan 20 logical px untuk scrollbar overlay Slint;
lebar konten mengikuti visible-width dan scroll horizontal dimatikan. Pemeriksaan
native mencakup scroll pengaturan, pemilihan sesi, dan penolakan sesi yang sudah dihapus.
Format ekspor sekarang Story 1080×1920 dan Feed 1080×1350, dengan dua gaya monokrom.
Komponen komposisi yang sama dipakai pada preview dan PNG. Layout Story/Feed
memakai posisi dan ukuran tipografi terpisah: judul 72/60 px pada kanvas 1080 px,
metadata 34-38 px, dan tugas 44/38 px dengan panel khusus. Teks panjang memakai
wrap dan elipsis; galeri native mencakup kedua tema, kedua format, dan kasus panjang. Durasi memakai bentuk
ringkas tanpa pembulatan naik. Nama sesi menjadi judul, mode dan status tetap jujur.
Nama tampilan di Preferences / General tersimpan lokal; kartu hanya menyertakannya
jika dipilih. Tugas selesai dapat disertakan secara opt-in dari snapshot sesi.
Ilustrasi pada referensi tidak disalin atau digunakan sebagai logo baru. Mask
brand yang sudah ada dipakai kembali. Hasil PNG 960×540 di bagian historis dokumen
merupakan format build sebelumnya.

Pilihan sesi sesuai keputusan terbaru: Pomodoro 25/5 menit, 4 siklus; Deep Focus
50 menit; Flow count-up. Tidak ada preset/input waktu kustom di UI. Field durasi
legacy tetap dibaca tetapi dinormalkan ke durasi tetap ketika aplikasi dibuka.
Nama serialisasi `Interval` dipertahankan agar riwayat lama kompatibel; label UI
adalah Pomodoro. Koreksi durasi riwayat tetap tersedia untuk akurasi catatan.
Durasi pendek pada verifikasi native hanya aktif dalam mode diagnostik terisolasi.

Panel kanan juga memuat to-do sesi lokal tanpa deadline: tambah, centang, pilih
tugas aktif, dan hapus. Maksimal 32 tugas; judul sampai 120 karakter. Nama sesi
maksimal 120 karakter disimpan bersama daftar tugas sebagai konteks; riwayat
menyimpan salinan konteks, terpisah dari draft sesi berikutnya. Tidak ada penambahan otomatis
ke kartu ekspor; tugas selesai hanya muncul jika sakelar berbagi diaktifkan. Daftar kosong menjadi default untuk file data lama. Logo di
sidebar kini dipusatkan terhadap tinggi header 44 logical px. Dialog konfirmasi
mengikuti tinggi isi, sementara editor riwayat memakai area scroll dan footer tetap.
Verifikasi native menghasilkan galeri jenis dialog dalam kedua bahasa/tema pada
ukuran jendela minimum, termasuk tampilan kegagalan/sedang menyimpan dan error awal.

Antarmuka menyediakan English (default) dan Bahasa Indonesia lewat Preferences →
Language. Pilihan tersimpan lokal dan diterapkan tanpa restart, termasuk timer
mini, notifikasi, dan kartu PNG. Data lama tanpa field `language` memakai English.
Catatan sesi, nama pilihan, caption pribadi, serta path tidak diterjemahkan.
Katalog terjemahan ada di `src/i18n/catalog.rs`; tidak ada layanan terjemahan online.

Palet antarmuka dan kartu ekspor memakai hitam, putih, dan abu-abu sesuai arahan
maintainer. Tema terang/gelap tetap tersedia. Ilustrasi latar berwarna dan toggle
ilustrasi tidak ditampilkan; aset sumber tetap disimpan. Logo tidak digambar ulang.
Snapshot dan benchmark lama yang memakai palet ungu adalah bukti build historis.

Main window dan floating memakai Winit software renderer, native executable tanpa
web runtime. Release Windows memakai GUI subsystem sehingga tidak membuka konsol.
WindowMoveArea menjalankan native drag; bingkai timer mini dapat diaktifkan melalui
Preferensi. Tombol bingkai dan Buka utama tidak ditampilkan; tombol tutup timer mini
mengembalikan ke jendela utama. Timer
meminta topmost jika preferensi aktif. Wayland tidak menjanjikan topmost atau posisi
absolut; compositor memutuskan. Window opaque dengan sudut luar persegi adalah
fallback saat ini. Tidak ada desktop blur, click-through, atau translucency native.

`notify-rust` memanggil WinRT di Windows dan D-Bus di Linux. Dependensi Windows
bernama `tauri-winrt-notification` hanyalah binding notifikasi, bukan framework
atau runtime aplikasi Tauri. Notifikasi dapat ditolak/ditunda OS atau Focus Assist;
status sesi selalu tersedia di UI. Aktivasi klik toast belum diintegrasikan untuk
membawa jendela ke depan. Tidak ada polling notifikasi saat idle.

Satu worker menunggu pekerjaan I/O; Linux memiliki listener sinyal logind yang
menunggu D-Bus. Timer tampilan berhenti saat tidak running; ringkasan dijadwalkan
ulang pada tengah malam lokal. Tidak ada animasi dekoratif kontinu. Profil RAM/CPU
tetap perlu diukur untuk build ini, bukan mengutip hasil prototype lama.

## Perintah verifikasi

Hasil lokal terakhir, 6 November 2026, Windows 11 Home Single Language 10.0.26200
x64, i5-13420H (12 logical processors), RAM 16 GB, Rust 1.95.0, DPI 125%:

- Formatting, Clippy `--locked --all-targets -- -D warnings`, 21 unit test dan
  release build: **lulus**, tanpa warning pada pemeriksaan final.
- `scripts/verify-native.ps1`: **lulus**. Tiga mode, validasi preferensi,
  pause/resume, batas konfirmasi yang tidak bisa dilewati floating, koreksi/hapus,
  dua PNG data nyata, isi JSON setelah quit dan dimensi PNG diverifikasi.
- `scripts/verify-windows.ps1`: **lulus** untuk dua jendela, flag topmost dan lifecycle.
- Snapshot native diperiksa untuk tema light/dark, ukuran minimum 1000×680,
  history, modal stop, preferensi dan hasil kedua template. Data uji terisolasi:
  `artifacts/native-23396-1793938938976/` dan `artifacts/state-23396-1793938938958/`.
  Editor koreksi juga diperiksa: isi dapat digulir dan tombol tetap berada dalam modal.
- Installer shortcut diuji pada `artifacts/jikanda-notification-test.lnk`, bukan
  Start menu pengguna. Pembacaan kembali System.AppUserModel.ID menghasilkan
  `YuPrananda.Jikanda` yang benar. Popup OS belum divalidasi setelah instalasi.
- Sintaks script Linux diperiksa dengan `bash -n` melalui Git Bash; ini **bukan**
  pengujian build atau runtime Linux.
- Inventory lisensi Windows berhasil dihasilkan; lisensi aplikasi kemudian ditetapkan
  GPL-3.0-only oleh maintainer. Audit distribusi tetap perlu diselesaikan.

Pada uji awal, assertion perpindahan interval berlomba dengan callback render/timer
yang jatuh pada batas detik yang sama. Skenario native sekarang menunggu kondisi
yang diharapkan secara terbatas (maksimal tiga callback tambahan), dan tetap gagal
jika engine paused/tidak maju. Pengukuran durasi engine tidak diubah untuk meluluskan
tes. Kegagalan assertion diagnostik ditulis ke artifacts dan keluar dengan status
gagal, sehingga GUI test tidak menggantung tanpa pesan.

### Pengukuran release terakhir

Angka di bawah berasal dari build MVP sebelum pembaruan metadata/atribusi GPL.
Hash ini dipertahankan sebagai identitas build yang benar-benar diukur; pembaruan
lisensi tidak diklaim sudah menjalani benchmark ulang.

SHA-256 executable:
`25CC5C1B43BA13A9907D706C296B3F23E85A9AC7F7516D1FD27B73BC320C6D07`.
Software renderer, accessibility aktif, dataset kosong terisolasi; warm-up 3 detik,
satu sampel sekitar 10 detik per kondisi, 6 November 2026 12:24 WITA.

| Jendela | Status | Working set MiB | Private bytes MiB | CPU % satu inti | Proxy startup ms |
| --- | --- | ---: | ---: | ---: | ---: |
| Main | Idle | 34.37 | 14.73 | 0.000 | 124.4 |
| Main | Running | 34.46 | 14.71 | 0.000 | 52.7 |
| Floating | Idle | 29.13 | 9.73 | 0.000 | 47.2 |
| Floating | Running | 29.24 | 10.30 | 0.312 | 72.5 |
| Keduanya | Idle | 34.91 | 15.11 | 0.000 | 46.2 |
| Keduanya | Running | 34.99 | 15.12 | 0.000 | 51.3 |

CPU nol berarti tidak ada tambahan waktu CPU terukur dalam sampel pendek, bukan
jaminan matematis nol. 100% berarti satu inti penuh. Startup memakai kemunculan
MainWindowHandle dengan polling 20 ms, bukan first paint/cold start. Tidak ada
klaim target seluruh OS atau seluruh ukuran arsip history telah tercapai.
Data mentah: `artifacts/windows-performance-20261106-122432.json` dan
`artifacts/windows-performance.json` (hasil terbaru dapat menimpa nama generik).

```text
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

Windows (policy hanya pada proses pengujian, tidak mengubah konfigurasi global):

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify-native.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify-windows.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/measure-windows.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/package-windows.ps1 -Internal
```

`verify-native` menjalankan skenario sekitar 40 detik dengan data terisolasi dan
notifikasi otomatis dimatikan: ketiga mode, pause/resume dua jendela, stop save/
discard, koreksi/hapus history, preference validation, dua ekspor data nyata, lalu
flush saat quit. Runner memeriksa JSON di disk dan dimensi kedua PNG. Snapshot
adalah hasil native, bukan bukti input fisik atau pengujian compositor lain.

Linux: `bash scripts/package-linux.sh --internal`. Skrip tersedia tetapi belum
dijalankan pada Linux. Paket Linux menyertakan metadata dependensi untuk audit;
teks lisensi lengkap masih harus disusun sebelum distribusi publik.

## Paket internal yang diserahkan

`artifacts/packages/Jikanda-0.1.0-windows-x64-internal-20261106-122704.zip`
(sekitar 10.6 MB), dengan checksum pada file `.zip.sha256` di sebelahnya.
SHA-256 ZIP: `E4CCB64A8008E6CF6852442AB96C9F58CC346505492403ED1A540942B26D4AFC`.

Arsip diperiksa tidak memuat state.json, instance.lock, atau data uji. Setelah
ekstraksi ke `artifacts/package-smoke-final-20261106`, executable dengan hash yang
sama menjalani ulang seluruh skenario native dan pemeriksaan data/PNG: **lulus**.
Hasil uji paket ada di subfolder artifacts folder ekstraksi; data yang dihasilkan
setelah ekstraksi tersebut tidak berada di dalam ZIP yang diserahkan.

Paket tersebut adalah build historis sebelum penetapan GPL. Paket berikutnya
menyertakan LICENSE dan source aplikasi. Kesiapan teknis Windows tidak menutup audit distribusi,
brand produksi, pengujian Linux, atau validasi OS manual yang disebutkan di atas.

## Sumber resmi

- https://docs.slint.dev/latest/docs/slint/reference/window/windowmovearea/
- https://docs.rs/winit/latest/winit/window/enum.WindowLevel.html
- https://doc.rust-lang.org/std/fs/struct.File.html#method.try_lock
- https://slint.dev/terms-and-conditions
- https://learn.microsoft.com/en-us/windows/win32/shell/enable-desktop-toast-with-appusermodelid
- https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.login1.html

## Pemeriksaan responsivitas

Ukuran minimum jendela utama tetap 1000 x 680 logical px. Isi ring memakai proporsi
terhadap diameter, termasuk waktu berjam-jam dan status dua baris. Label tombol
memiliki inset dan elipsis, toggle membungkus label, input serta judul tugas tidak
meluap, dan panel kanan memakai scroll saat konten tidak muat. Galeri diagnostik
responsive mencakup tiga mode, English/Indonesia, ukuran 1000 x 680 dan 1280 x 800,
semua halaman/tab pada ukuran minimum, serta waktu panjang pada mini. Nama dan tugas
panjang hanya fixture diagnostik; data pengguna tidak berubah.

## Alarm countdown

Tiga WAV orisinal (Soft Bell, Double Chime, Bright Tone) beserta generator matematis
berlisensi GPL-3.0-only ditambahkan tanpa crate audio baru. Settings menyimpan mute
serta pilihan bunyi. AlarmEvents mengonsumsi tiap akhir countdown/segmen sekali,
termasuk saat dimatikan, agar unmute tidak mengulang kejadian lama. Stop/pause/sleep
serta Flow manual tidak membunyikan alarm. Test otomatis tanpa suara; probe manual
Windows pada 9 November 2026 mengembalikan sukses untuk ketiga pilihan. Ini bukan
bukti volume perangkat terdengar oleh pengguna. Galeri native memeriksa pengaturan
alarm pada ukuran minimum dan script memastikan mute/BrightTone tetap tersimpan.
38 unit tests, Clippy, build release, dan integrasi Windows lulus. Audio Linux
menggunakan paplay/aplay opsional dan belum diuji secara runtime di mesin ini.
