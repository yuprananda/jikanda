# Phase 0 — Technical Feasibility

Catatan historis baseline. Implementasi MVP berikutnya, pengujian, dan batas rilis
terbaru ada di `docs/release-readiness.md`; angka/fitur di bawah bukan status terbaru.

## Batas prototipe

Satu Cargo package dengan pemisahan domain, aplikasi, platform, presentasi Slint,
dan ekspor. Domain tidak bergantung pada Slint atau jam sistem. Belum ada database,
penyimpanan sesi aktif, akun, jaringan, atau telemetry. Tidak ada perubahan logo.
Peralihan ke Phase 1 atau fase berikutnya membutuhkan persetujuan maintainer.

`src/domain/session.rs` menerima offset monotonic; `src/app.rs` memasok offset
`Instant` dan mengoordinasikan kedua jendela. UI mengirim perintah dan membaca
properti, bukan memiliki timer sendiri. Satu timer event-loop memperbarui tampilan
setiap detik ketika running; timer tersebut dihentikan saat ready/paused/terminal.
Tidak ada animasi kontinu atau thread polling aplikasi.

## Kebijakan waktu

- Durasi dihitung dari monotonic elapsed, bukan jumlah callback.
- Pause mengakumulasi satu segmen; resume memulai segmen baru. Countdown tidak negatif.
- Nol ditolak. Completion dan interruption adalah status berbeda.
- Suspend menghasilkan pause dan membutuhkan resume eksplisit. Tidak ada kredit waktu tidur.
- Windows mendaftarkan callback PowerRegisterSuspendResumeNotification. Callback
  hanya menandai atomic; UI thread mengonsumsi sinyal pada pemeriksaan berikutnya.
- Fallback membandingkan jeda monotonic dan perubahan wall clock. Jeda monotonic
  >3 detik, wall clock mundur, atau selisih wall/monotonic >2 detik dianggap tidak
  pasti. Perubahan jam tidak menambah durasi; ia dapat menyebabkan pause konservatif.
- Pada jeda tidak pasti, hanya sampel terakhir yang dipercaya. Ini dapat membuang
  hingga satu interval sampel normal, atau lebih bila event loop tersendat.
- Linux belum memiliki langganan logind; fallback bukan detektor suspend universal.
  Sleep singkat yang tidak melewati ambang mungkin tidak terdeteksi. Integrasi
  `PrepareForSleep` perlu spike tersendiri pada mesin Linux sebelum kebijakan ini
  dinyatakan tervalidasi di Linux.
- Restart selalu mulai tanpa sesi aktif. Crash tidak membuat catatan completed.
  Persistensi sesi aktif ditunda sampai kebijakan pemulihan dirancang.

## Jendela dan visual

Winit + software renderer dipilih eksplisit. Floating timer memakai WindowMoveArea
dan permintaan always-on-top. Tidak ada click-through, posisi absolut, translucent
window, atau layer-shell khusus compositor. Tombol Bingkai memberi fallback
dekorasi native; tombol Buka utama dan Keluar menjaga akses navigasi.

Windows dan X11 tetap membutuhkan pengujian kebijakan window manager. Pada Wayland,
Winit tidak mendukung window level topmost dan posisi absolut. Floating menjadi
jendela biasa; pengguna dapat memakai pin/rules compositor bila tersedia.
Pesan Wayland berdasarkan lingkungan WAYLAND_DISPLAY adalah indikasi konservatif,
bukan deteksi pasti backend X11 versus Wayland.

Palet menggunakan arah brand provisional. Custom button mempunyai label aksesibel,
keyboard activation, tab focus, hover/pressed/disabled/focus state. Tidak ada gerakan
dekoratif sehingga tidak dibutuhkan toggle reduced-motion di spike ini. Accessibility
dan DPI tetap perlu uji pembaca layar serta skala desktop nyata.

Baseline awal menggunakan font sistem. Prototipe visual berikutnya memakai Plus
Jakarta Sans dengan lisensi OFL terlampir, serta crop mascot/wordmark dari sheet
sesuai arahan maintainer; lihat `docs/visual-prototype.md`. Future SVG harus berasal dari master
editable yang disetujui, termasuk pengujian 16/24/32/48/64 px. Tidak ada SVG mascot
baru yang dibuat. Renderer GPU hanya akan dibandingkan bila bukti kualitas/performa
menunjukkan kebutuhan; footprint lebih kecil belum terbukti dengan asumsi saja.

## Ekspor PNG

Spike memakai komponen Slint SampleCard dan Window::take_snapshot untuk memperoleh
RGBA, lalu `png` untuk encoding. Ukuran yang diminta 960×540; ukuran aktual diperiksa.
Kartu selalu diberi label SAMPLE DATA. Ekspor tidak menggunakan browser atau jaringan.
Nama file eksisting ditolak untuk menghindari overwrite diam-diam.

Snapshot/encoding berjalan sinkron sebagai spike yang dipicu pengguna; biaya ini
harus diukur sebelum ekspor production. Pemisahan kerja encoding ke worker hanya
layak ditambahkan bila latensinya mengganggu. Desain template dan data pencapaian
riil tetap Phase 4.

## Verifikasi Windows — 6 November 2026

Lingkungan: Windows 11 Home Single Language 10.0.26200 x64, Intel Core i5-13420H
(12 logical processors), RAM 16 GB, Rust/Cargo 1.95.0, renderer software,
accessibility diaktifkan. Snapshot native menunjukkan skala desktop 125%.

- `cargo fmt --check`: lulus.
- `cargo clippy --all-targets -- -D warnings`: lulus.
- `cargo test`: 10 pengujian lulus, termasuk pause/resume, zero, delayed callback,
  completion, interruption/restart, gap konservatif, dan sinyal callback daya Windows.
  Regresi pada klik Pause tepat saat deadline atau setelah gap juga diuji: perintah
  tersebut tidak boleh diam-diam memulai sesi baru atau melanjutkan sesi.
- `cargo build --release`: lulus.
- `--verify-native`: lulus pada build terakhir. Memeriksa input durasi invalid/valid,
  sinkronisasi dua jendela, pause dari floating, resume dari main, dan completion.
- Snapshot light/dark kedua jendela diperiksa secara visual. Kontrol floating sempat
  terpotong pada tinggi 170 logical px; diperbaiki menjadi 190 dan diperiksa ulang.
- Ekspor lokal 960×540 RGBA PNG berhasil dan diperiksa secara visual. Kartu tidak
  memunculkan jendela baru. Ukuran diinisialisasi eksplisit sebelum snapshot untuk
  menghindari ukuran nol pada window yang belum ditampilkan.
- `scripts/verify-windows.ps1`: lulus; dua native window ditemukan, WS_EX_TOPMOST
  terpasang, menutup floating memulihkan main, dan menutup main mengakhiri proses.
  Ini memeriksa flag topmost, bukan jaminan stacking di atas fullscreen/secure desktop.

Snapshot lokal build terakhir: `artifacts/native-5676/`. File tersebut diabaikan oleh .gitignore.
Tidak ada pengujian drag fisik, keyboard/screen reader nyata, suspend/hibernate nyata,
multi-monitor, atau skala selain 125%. Hanya target Rust Windows MSVC yang terpasang;
build dan perilaku Linux X11/Wayland belum diuji. Linux masih memakai fallback gap,
bukan langganan logind. Phase 0 belum membuktikan semua target lintas platform.

### Pengukuran release

Sampel terakhir: 6 November 2026, 10:51 WITA; warm-up 3 detik dan sampel sekitar
10 detik per kondisi. File executable SHA-256:
`3182DDEE3F10DA32DE1D584D4DAA961D58CA99EF72CE81466A943D9B542608EE`.

| Jendela | Status | Working set MiB | Private bytes MiB | CPU % satu inti | Proxy startup ms |
| --- | --- | ---: | ---: | ---: | ---: |
| Main | Idle | 22.23 | 5.29 | 0.000 | 245.6 |
| Main | Running | 22.29 | 5.30 | 0.156 | 114.3 |
| Floating | Idle | 20.77 | 3.86 | 0.000 | 113.1 |
| Floating | Running | 20.75 | 3.82 | 0.312 | 116.1 |
| Keduanya | Idle | 22.72 | 5.71 | 0.000 | 113.0 |
| Keduanya | Running | 22.70 | 5.65 | 0.468 | 84.1 |

CPU dihitung sebagai delta waktu CPU proses dibagi waktu wall sampel; 100% berarti
satu inti penuh, bukan seluruh 12 logical processors. Angka nol berarti tidak ada
tambahan waktu CPU yang terukur dalam sampel pendek, bukan jaminan CPU selalu nol.
Pada putaran pendahuluan, floating idle sempat mencatat 0.780% satu inti; pada
pengulangan hasilnya nol. Variasi ini belum dianalisis dan membutuhkan pengukuran
lebih panjang sebelum klaim performa umum. Working set adalah memori resident;
private bytes adalah metrik terpisah dan bukan penggantinya.

Proxy startup adalah handle native pertama yang terlihat, dengan polling 20 ms;
ini bukan benchmark cold start atau first paint. Hasil RAM mendukung kelayakan
baseline software renderer pada mesin ini, tetapi bukan bukti target semua OS.
Data baseline ini: `artifacts/windows-performance-20261106-105152.json`.
`artifacts/windows-performance.json` memuat hasil paling baru dan dapat berbeda
setelah perubahan UI. Pengukuran prototipe visual berikutnya dicatat terpisah di
`docs/visual-prototype.md`.

Checklist manual lintas platform:

- Drag dari area teks; klik kontrol tidak menyeret; keyboard Tab/Space/Enter bekerja.
- Topmost saat aplikasi lain aktif; dekorasi fallback dan reopen main tetap berguna.
- DPI 100/125/150/200%, perpindahan monitor, disconnect monitor, ukuran minimum.
- Sleep/hibernate ketika running dan paused, resume manual, perubahan jam maju/mundur.
- Completion, stop sebelum selesai, tutup floating, tutup main, dan keluar saat running.
- Tampilan light/dark, kontras, screen reader, dan tidak ada redraw kontinu saat idle.
- Ulangi benchmark release main/floating/both pada Windows, X11, dan Wayland.

Benchmark Windows memakai working set dan private bytes proses, delta CPU selama
sampel, serta kemunculan MainWindowHandle sebagai proxy startup (polling 20 ms,
bukan first paint; satu warm run per kondisi). Catat
OS, CPU, RAM, build, renderer, durasi sampel, dan kondisi jendela dalam JSON.
Hasil satu mesin bukan jaminan lintas platform.

## Referensi resmi

- https://docs.slint.dev/latest/docs/slint/reference/window/windowmovearea/
- https://docs.slint.dev/latest/docs/slint/reference/window/window/
- https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backends_and_renderers/
- https://docs.rs/winit/latest/winit/window/enum.WindowLevel.html
- https://docs.rs/winit/latest/winit/window/struct.Window.html
- https://doc.rust-lang.org/std/time/struct.Instant.html
- https://learn.microsoft.com/en-us/windows/win32/api/powerbase/nf-powerbase-powerregistersuspendresumenotification
- https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.login1.html
