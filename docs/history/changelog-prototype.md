# Changelog

## 0.1.0 — Unreleased / internal review

- Tambah alarm countdown yang bisa dimatikan, tiga sinyal orisinal GPL-3.0-only,
  preview bunyi dan pilihan tersimpan di Preferences / General. Tidak berulang,
  tidak berbunyi saat stop/pause/Flow manual, dan terpisah dari notifikasi desktop.

- Perbaiki responsivitas isi timer: ukuran angka mengikuti diameter, status berada
  dalam area khusus, dan waktu panjang tetap muat pada mini. Tombol membatasi label,
  toggle membungkus teks, dan panel sesi dapat digulir pada jendela minimum.

- Komposisi kartu menyesuaikan Story dan Feed: durasi, judul, metadata, dan tugas
  diperbesar; tugas memiliki blok khusus agar tidak tenggelam dalam ruang kosong.

- Perbaiki notifikasi Windows portable dengan registrasi identitas per-user sebelum
  mengirim. Tombol uji menampilkan status dan mencegah klik berulang selama proses;
  penerimaan OS tetap dibedakan dari banner yang terlihat. Pengaturan OS tidak diubah.

- Cards beralih ke satu sesi tersimpan dengan tombol dari Riwayat atau hasil sesi.
  Nama tampilan opsional pindah ke Preferences / General dan tersimpan lokal.
  Nama sesi menjadi judul kartu, dilengkapi durasi aktual, mode, tanggal, dan status;
  nama pengguna serta tugas selesai hanya tampil atas pilihan pengguna.

- Footer sidebar memiliki batas tinggi, inset teks, dan margin bawah yang tetap;
  tombol tema, tagline, dan kredit lebih tertata pada ukuran jendela minimum.

- Session name menggantikan input session note, ditempatkan bersama mode dan to-do
  di panel kanan. Riwayat menyimpan nama dan snapshot tugas; catatan lama tetap utuh.
- Mini menampilkan nama/checklist tersinkron hanya selama sesi berjalan/dijeda,
  dan kembali ringkas saat idle/selesai. Sesi baru mengosongkan draft dengan konfirmasi.

- Panel Focus session memperjelas hierarki durasi, siklus, dan petunjuk; Pomodoro
  menampilkan metrik fokus/istirahat terpisah. Judul tugas lebih terbaca, checkbox
  lebih proporsional, dan field tugas baru memiliki placeholder.

- About lebih ringkas dengan identitas aplikasi serta lisensi/atribusi sejajar.
  Hapus status simpan rutin di header/footer; kegagalan simpan dan retry tetap terlihat.

- Preferences dikelompokkan ke tab General, Mini timer, Storage, dan About.
  Kontrol tema tersedia bersama bahasa; footer simpan tetap terlihat dan informasi
  lisensi dipisahkan dari pengaturan harian.

- Timer mini 228×112 px dengan logo lebih kecil dan kontrol yang dipadatkan.
  Opacity native Windows 50–100% tersimpan lokal; Linux memakai fallback opaque.

- Area scroll menyediakan jalur scrollbar sendiri, lebar konten terkunci ke viewport,
  dan scroll horizontal dimatikan agar kontrol tidak tertimpa scrollbar.

- Cards memakai panel pengaturan dan pratinjau utuh terpisah. Kartu monokrom
  memiliki durasi ringkas, caption, tanggal, dan logo yang sudah ada; ekspor lokal
  Story 1080×1920 atau Feed 1080×1350, dengan gaya terang/gelap.

- Dialog konfirmasi mengikuti tinggi isi, tanpa spacer kosong; tombol lebih ringkas.
  Header utama dipadatkan menjadi 44 px dengan logo dan kontrol di tengah vertikal.

- To-do sesi lokal tanpa deadline: tambah, centang, pilih tugas aktif, hapus.
  Catat judul tugas aktif pada riwayat jika catatan sesi kosong. Logo utama rata
  tengah vertikal terhadap header.

- Tiga pilihan sesi: Pomodoro 25/5 menit (4 siklus), Deep Focus 50 menit, dan Flow
  count-up. Hapus preset/input durasi kustom dan tombol mini duplikat di panel kanan.
  Riwayat lama dengan mode tersimpan `Interval` tetap dibaca dan ditampilkan sebagai Pomodoro.

- English sebagai default, dengan pilihan Bahasa Indonesia di Preferensi.
  Pilihan tersimpan lokal, berlaku tanpa restart pada main/mini, notifikasi, dan kartu.

- Palet hitam-putih/abu-abu untuk tema terang, gelap, timer mini, dan kartu PNG.
  Ilustrasi berwarna tidak ditampilkan pada arah visual monokrom ini.

- Hapus tombol Keluar dari timer mini; kontrol × tetap kembali ke jendela utama.

- Timer mini dipadatkan menjadi 280 × 140 px; warna kontrol jendela diseragamkan
  dengan tema yang dipakai timer mini.

- Kontrol jendela lebih ringkas: tombol 28 px, ikon 16 px, latar muncul saat hover.

- Hapus tombol bingkai sistem dari sidebar/timer mini dan tombol Buka utama.
  Tombol tutup timer mini kembali ke utama; fallback bingkai mini ada di Preferensi.

- Maintainer menetapkan lisensi aplikasi GPL-3.0-only; Slint memakai opsi GPLv3.

- Tiga mode: Interval, Deep Focus, Flow; pengukuran monotonic dan pause pada sleep/gap.
- Main/floating custom Slint dengan light/dark, font Plus Jakarta Sans, ilustrasi
  senja statis, dan mascot/wordmark crop dari brand sheet sesuai arahan maintainer.
- History lokal, koreksi mode/waktu/durasi/status/catatan, hapus dengan konfirmasi,
  serta ringkasan harian dan tujuh hari.
- Preferensi lokal, single-instance lock, penulisan JSON atomik, fallback jika data
  rusak atau penyimpanan gagal; tidak ada pemulihan sesi aktif setelah crash.
- Kartu PNG offline dengan dua template, rentang tanggal dan nama opt-in.
- Notifikasi WinRT/D-Bus dan listener sleep Windows/logind dengan fallback.
- Skrip verifikasi native, paket internal/checksum, inventory lisensi, dan CI.

Belum disetujui untuk rilis publik. Lihat docs/release-readiness.md.
