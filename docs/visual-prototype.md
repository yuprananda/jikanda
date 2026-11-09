# Prototipe visual — ruang fokus saat senja

Catatan historis perubahan visual. Fitur history/persistence/kartu nyata sudah
ditambahkan sesudah prototipe ini; lihat `docs/release-readiness.md` untuk status MVP.

Arahan maintainer: menyesuaikan referensi UI yang dikirim, mempertahankan mascot
yang sama dengan brand sheet, serta memotong sendiri aset yang masih menyatu.
Ini persetujuan untuk prototipe visual, bukan finalisasi SVG logo atau penambahan
fitur di luar scope. Tidak ada task manager, statistik palsu, atau riwayat semu.

## Implementasi

- Main window 1120×720 logical px, minimum 1000×680. Sidebar navigasi fungsional,
  ilustrasi statis, panel sesi, timer melingkar, kontrol mulai/jeda/stop.
- Floating 332×204 logical px, kontrol mulai/jeda/stop, buka utama, bingkai, keluar.
- Deep Focus 5/25/50 menit dan durasi kustom 1–86400 detik. Semua preset tetap satu
  countdown tanpa interval istirahat otomatis. Interval dan Flow belum ditambahkan.
- Catatan niat sesi hanya berada di memori selama aplikasi terbuka. Tidak ikut
  dalam kartu contoh dan tidak masuk log. Ini bukan daftar tugas atau history.
- Dark default, tema light tetap tersedia. Warna, komposisi, dan aset provisional.
- Rust tetap satu-satunya sumber status timer. Rasio sisa durasi menggerakkan ring
  dan bar; update satu detik hanya ketika running. Tidak ada animasi kontinu.
- Software renderer tetap dipakai. Tidak ditambahkan renderer GPU atau crate baru.
- Panel transparan mencampur warna dengan ilustrasi aplikasi sendiri. Tidak ada
  live blur desktop, efek kaca native, shadow software palsu, atau layer-shell.
- Jendela native opaque dan tepi luar persegi adalah fallback saat ini. Radius
  panel di dalam aplikasi tetap diterapkan; sudut luar/shadow native belum disamakan
  dengan mockup. Bingkai sistem tersedia untuk kedua jendela.

## Brand sheet dan proses crop

Sumber asli: `Jikanda Brand Identity Sheet.png`, 1536×1024. File asli tidak diubah.
Script reproducible: `scripts/extract-brand.ps1` (System.Drawing pada Windows).

- Mascot: crop `(635, 50, 315, 335)` →
  `assets/brand/exports/mascot-concept-mask.png`.
- Wordmark: crop `(1035, 159, 450, 145)` →
  `assets/brand/exports/wordmark-concept-mask.png`.
- Warna ungu pada latar near-white diubah menjadi alpha mask; UI memberi warna
  monokrom melalui Image.colorize. Gestur dan bentuk sumber dipertahankan.
- Rasio tidak diubah; tidak ada tracing atau auto-vectorization. Tidak dilakukan
  upscale karena ukuran crop cukup untuk penampilan logo di UI saat ini.
- Aset raster ini hanya untuk prototipe. Belum ada SVG master produksi, micro-icon
  tray, atau pengujian logo 16/24/32/48/64 px. Persetujuan final tetap diperlukan.

## Font

Plus Jakarta Sans variable upright dibundel, 176288 byte, tanpa modifikasi.
Sumber: Google Fonts, `ofl/plusjakartasans/PlusJakartaSans[wght].ttf`.
Lisensi SIL OFL 1.1 disertakan di `assets/fonts/OFL-PlusJakartaSans.txt`.
Wordmark memakai potongan sheet, bukan mengganti bentuknya dengan font ini.

- https://github.com/google/fonts/tree/main/ofl/plusjakartasans
- https://github.com/tokotype/PlusJakartaSans

## Ilustrasi dan prompt

Aset: `assets/scenes/dusk-study.png`, 1536×1024. Dibuat menggunakan built-in
imagegen untuk proyek ini; bukan diambil dari situs stok, bukan crop UI mockup,
dan tidak memuat mascot, logo, atau teks. Tidak ada data sesi pengguna yang
dikirim untuk pembuatan aset. Aplikasi tidak mengakses layanan imagegen saat runtime.

Prompt final yang digunakan:

> Use case: stylized-concept. Asset type: static desktop focus companion background
> illustration, landscape 1536x1024. Create an original refined painterly digital
> illustration of a quiet study nook at dusk. View from inside toward a large window;
> distant rooftops, gentle hills and soft lavender-pink sunset sky. Left edge has a
> trailing leafy houseplant and a few books; foreground lower edge a warm wooden desk
> with plain ceramic mug and closed notebook. Mature premium editorial illustration,
> beautifully composed, soft natural light, restrained plum, dusty lavender and peach,
> subtle depth and precise drawing. Large central 60 percent must be quiet open window
> and soft sky with very little visual detail so timer UI can sit over it; objects
> concentrated along left and bottom edges. No people, no text, no letters, no logos,
> no mascot, no clocks, no UI, no frame, no watermark. Landscape image, full bleed,
> static texture suitable for offline native app. Do not use anime/manga style or neon.
> This is just the scenic background, never an image of an app interface.

## Verifikasi

Pada Windows 11 10.0.26200 x64, i5-13420H, RAM 16 GB, skala desktop 125%:

- `cargo fmt --check`, Clippy `--all-targets -- -D warnings`, 10 tes, dan build
  release: lulus. Tidak ada crate atau renderer tambahan.
- `--verify-native`: lulus. Snapshot tema terang/gelap main dan floating diperiksa;
  snapshot main minimum 1000×680 saat running juga diperiksa, termasuk arc progress.
  Kontrol dan catatan platform tetap berada dalam panel setelah spacing minimum
  diperbaiki. Completion mengosongkan bar tanpa mengubah status menjadi interruption.
- `scripts/verify-windows.ps1`: lulus untuk dua native window, WS_EX_TOPMOST,
  menutup floating memulihkan main, dan menutup main mengakhiri proses.
- PNG contoh tetap berhasil diekspor lokal pada 960×540.
- Snapshot build terakhir: `artifacts/native-23368/` (diabaikan Git).

Pengukuran release terakhir, 6 November 2026 11:20 WITA, warm-up 3 detik dan
sampel sekitar 10 detik per kondisi; software renderer dan accessibility aktif:

| Jendela | Status | Working set MiB | Private bytes MiB | CPU % satu inti | Proxy startup ms |
| --- | --- | ---: | ---: | ---: | ---: |
| Main | Idle | 33.62 | 14.69 | 0.000 | 132.4 |
| Main | Running | 33.73 | 14.70 | 1.406 | 95.9 |
| Floating | Idle | 28.55 | 9.71 | 0.000 | 110.3 |
| Floating | Running | 28.63 | 9.71 | 0.000 | 113.0 |
| Keduanya | Idle | 34.09 | 15.08 | 0.000 | 95.7 |
| Keduanya | Running | 34.19 | 15.07 | 0.468 | 114.1 |

SHA-256 executable: `3AC22231CB91169C2F2E418491EEC3F86F3B4F89F3607B0247A14765130503EC`.
Data mentah terbaru: `artifacts/windows-performance.json`. Skrip juga menyimpan
salinan JSON bertimestamp. CPU nol berarti tidak ada tambahan waktu CPU yang
terukur dalam sampel pendek, bukan jaminan absolut. Proxy startup adalah handle
jendela pertama, bukan first paint atau cold start. Variasi running membutuhkan
sampel lebih panjang untuk kesimpulan umum; hasil ini bukan jaminan lintas mesin.

Pengujian drag fisik, keyboard/screen reader nyata, suspend nyata, serta Linux
X11/Wayland tetap belum dilakukan. Batas always-on-top Wayland tidak berubah.
Sudut luar native persegi dan tidak adanya desktop blur tetap merupakan batas
visual yang disengaja, bukan klaim setara pixel dengan mockup.
