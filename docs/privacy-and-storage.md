# Data lokal dan kebijakan waktu

Jikanda tidak memiliki akun, server, sinkronisasi, analytics, atau telemetry.
Tidak ada pengiriman riwayat/catatan. Notifikasi hanya memakai teks umum, bukan
catatan pengguna. Tautan Slint pada atribusi hanya dibuka bila diklik pengguna.

## Lokasi

- Windows: `%LOCALAPPDATA%\Jikanda\state.json`.
- Linux: `$XDG_DATA_HOME/jikanda/state.json` jika variabel berupa path absolut;
  fallback `$HOME/.local/share/jikanda/state.json`.
- Buka lokasi melalui Preferensi → Buka folder data.
- PNG default berada di subfolder `exports`; folder dapat diubah di Preferensi.
- Flag lanjutan `--data-dir <path-absolut>` memilih folder data terpisah.
- `--verify-native` dan semua flag `--probe-*` otomatis memakai folder uji baru di
  `artifacts/state-<pid>-<timestamp>`. Tidak menulis history pribadi pengguna.

Format JSON versi 1 menyimpan preferensi dan daftar sesi: ID, mode, timestamp UTC
dalam milidetik, tanggal lokal saat mulai, durasi fokus monotonic, status selesai
atau terputus, nama sesi maksimal 120 karakter, dan salinan daftar tugas ketika
sesi dicatat. Catatan lama maksimal 160 karakter dipertahankan terpisah; tidak
dianggap otomatis sebagai nama sesi. Berkas tidak dienkripsi.
Pada Unix, file state baru dibuat mode 0600 dan folder baru mode 0700.

## Integritas dan kegagalan

Satu instance memegang `instance.lock` melalui file locking native Rust. Folder
yang sama tidak dapat dipakai dua proses Jikanda bersamaan. Satu worker I/O menulis
snapshot JSON ke file sementara, melakukan sync, lalu rename sebagai state.json.
Ini mengurangi risiko file separuh tertulis; bukan jaminan terhadap seluruh jenis
kerusakan disk, power loss, atau perilaku filesystem yang tidak standar.

Data rusak, versi tidak didukung, ID duplikat, atau format tidak valid tidak direset
diam-diam. Aplikasi menampilkan kesalahan tanpa menimpa berkas tersebut. Cadangkan
folder sebelum memperbaiki data secara manual. Saat penyimpanan gagal, perubahan
tetap ada di memori dan tersedia Coba simpan lagi. Keluar meminta pilihan eksplisit
bila masih ada perubahan yang belum tersimpan.

Batas MVP: 10000 entri dan berkas state maksimal 16 MiB. History ditampilkan 20
entri per halaman agar jumlah widget tidak tumbuh mengikuti seluruh arsip. Jika
batas tercapai, sesi baru ditolak dengan pesan yang jelas, bukan menghapus data lama.

## Waktu dan hasil sesi

- `Instant` mengukur durasi; tick UI tidak dijumlahkan sebagai waktu fokus.
- Countdown dibulatkan ke atas untuk tampilan; Flow ke bawah. Catatan durasi tetap
  milidetik aktual, bukan teks yang sudah dibulatkan.
- Interval mencatat satu sesi dengan total seluruh segmen fokus; istirahat tidak
  ditambahkan. Tiap perpindahan segmen menunggu tombol mulai.
- Deep Focus satu countdown. Flow count-up, diselesaikan lewat Selesai & simpan.
- Countdown yang selesai disimpan otomatis. Stop/keluar saat aktif menjeda terlebih
  dahulu dan menawarkan simpan sebagai terputus, buang, atau kembali.
- Suspend menjeda dan membutuhkan resume manual. Windows memakai power callback;
  Linux mencoba sinyal logind PrepareForSleep. Jika layanan tidak ada, fallback
  membandingkan jeda monotonic dan wall clock.
- Jeda >3 detik, wall clock mundur, atau perbedaan wall/monotonic >2 detik dianggap
  tidak pasti. Hanya sampel terakhir yang dipercaya. Ini dapat mengurangi hitungan
  saat event loop tersendat, termasuk build debug yang lambat. Sleep pendek tanpa
  sinyal OS dapat luput dari fallback; perilaku target harus diuji secara nyata.
- Sesi aktif tidak dipersist. Crash/restart tidak menghasilkan completion atau
  meneruskan timer. Sesi yang belum selesai dan belum dikonfirmasi dapat hilang.
- Perubahan wall clock tidak mengubah durasi fokus. Endpoint timestamp dapat
  berbalik bila jam sistem diubah; koreksi tersedia di history.
- Durasi sesi diatribusikan ke tanggal lokal saat sesi dimulai, termasuk bila sesi
  melewati tengah malam. Ringkasan mingguan berarti hari ini dan enam hari sebelumnya.
  Mengubah timezone setelahnya tidak memindahkan tanggal sesi yang sudah tersimpan.

## Mengoreksi, menghapus, dan mengekspor

History → Ubah mengizinkan koreksi mode, tanggal/waktu lokal, durasi, status, dan
catatan. Waktu lokal yang ambigu/tidak ada pada transisi DST ditolak. Durasi tidak
boleh melebihi rentang endpoint setelah koreksi (toleransi pembulatan satu detik).
Menghapus entri atau semua history selalu meminta konfirmasi. Preferensi tetap ada.

Hapus seluruh history mengosongkan entri pada state.json; PNG lama tidak dihapus.
Jika proses pernah crash saat menulis, file `state-*.tmp` dapat tertinggal berisi
snapshot lama. Untuk penghapusan seluruh data, tutup semua instance, lalu hapus
folder data secara manual, termasuk file sementara dan ekspor jika diinginkan.
Tidak ada auto-backup atau salinan tersembunyi yang dibuat aplikasi.

Kartu hanya memakai satu record tersimpan yang dipilih. Ekspor ditolak
jika perubahan masih menunggu penyimpanan. Nama opsional nonaktif secara default;
catatan sesi tidak dimasukkan otomatis. Nama/kalimat kartu hanya di memori, tidak
dipersist. Render memakai Slint lokal, encoding PNG di worker, tanpa browser.
File baru memakai timestamp; file yang sudah ada tidak ditimpa. Kartu merepresentasikan
waktu sesi tercatat, bukan produktivitas atau konsentrasi yang terverifikasi.
# Tugas sesi

Daftar tugas disimpan bersama data lokal, maksimal 32 tugas dengan judul sampai
120 karakter. Tugas tidak mempunyai deadline dan tidak dikirim ke layanan lain.
Nama sesi dan daftar tugas menjadi satu draft lokal. Nama disimpan setelah jeda
mengetik singkat; nama tidak dapat diganti saat timer berjalan/dijeda. Riwayat
menyimpan salinan nama/tugas, sehingga perubahan draft berikutnya tidak mengubah
riwayat. Sesi baru meminta konfirmasi sebelum mengosongkan draft. Menyelesaikan
timer tidak otomatis mencentang tugas. Nama/tugas tidak dimasukkan otomatis ke PNG.


## Kartu per sesi dan nama tampilan

Nama tampilan opsional (maksimal 60 karakter) disimpan pada settings.display_name.
Data lama tanpa field ini tetap terbaca dengan nama kosong. Input tersedia di
Preferences / General, bukan di editor kartu. Nama dan daftar tugas selesai secara
bawaan tidak disertakan; pengguna mengaktifkannya pada Cards. Nama sesi, tanggal,
mode, status, dan durasi aktual satu sesi menjadi isi kartu. Catatan lama tidak
pernah ikut. Kartu memakai snapshot tugas pada riwayat, bukan draft tugas sekarang.
PNG tetap dibuat lokal/offline. Tidak ada rekap rentang tanggal pada alur Cards.

## Notifikasi Windows portable

Identitas YuPrananda.Jikanda didaftarkan di registry per-user
HKCU\Software\Classes\AppUserModelId\YuPrananda.Jikanda saat mengirim notifikasi.
Hanya DisplayName aplikasi yang ditulis, tanpa admin, izin notifikasi, autostart,
atau perubahan Do Not Disturb. Tidak ada polling tambahan. Pengiriman tetap pada
worker. Tombol uji memberi hasil pengiriman; penerimaan OS tidak membuktikan
banner terlihat. Pengaturan OS dapat menyembunyikan atau menahan banner.
Flag manual --probe-notification mengirim satu pesan umum tanpa membaca data sesi.
Tes native biasa tidak mengirim notifikasi nyata.

Referensi: https://github.com/tauri-apps/winrt-notification/blob/dev/examples/unpackaged_app.rs

## Alarm lokal

Preferences / General menyediakan alarm on/off serta Soft Bell, Double Chime,
dan Bright Tone. Default aktif dengan Soft Bell; data lama tanpa field alarm tetap
terbaca. Simpan preferensi untuk menerapkan. Pilihan dan mute disimpan lokal.
Bunyi pendek sekali pada countdown Deep Focus, akhir segmen fokus/istirahat
Pomodoro, dan akhir Pomodoro. Pause, stop, sleep, dan Flow yang diakhiri manual
bukan alarm countdown. Satu kejadian tidak diputar ulang saat refresh atau unmute.
Preview adalah tindakan eksplisit dan tetap bisa dicoba meskipun alarm dimatikan.

WAV asli disematkan pada executable. Windows memakai PlaySound di worker; Linux
memakai paplay (PulseAudio/PipeWire) atau aplay (ALSA) jika tersedia, dengan batas
waktu empat detik per player dan cache WAV di subfolder sounds pada folder data.
Tidak ada thread audio/polling saat idle. Worker dapat sibuk maksimal 1.15 detik
selama playback Windows; UI/timer tetap responsif. Pengaturan notifikasi desktop
terpisah, dan Jikanda meminta notifikasi tanpa bunyi agar tidak berbunyi dua kali.
Volume/mute perangkat dan kebijakan notifikasi OS tetap berlaku. Linux audio belum
divalidasi pada mesin ini. --probe-alarm=0/1/2 memainkan satu sampel secara manual;
verifikasi otomatis biasa selalu tanpa bunyi.
