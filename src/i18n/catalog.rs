// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

// Indonesian source messages and English translations. Keep placeholders unchanged.
pub const MESSAGES: &[(&str, &str)] = &[
    ("Alarm selesai", "Completion alarm"),
    (
        "Bunyikan alarm saat countdown atau segmen selesai",
        "Play a sound when a countdown or segment ends",
    ),
    ("Memutar...", "Playing..."),
    ("Coba bunyi", "Preview sound"),
    (
        "Bunyi pendek sekali, tanpa pengulangan. Terpisah dari notifikasi desktop. Simpan preferensi untuk menerapkan.",
        "One short sound, with no looping. Independent of desktop notifications. Save preferences to apply.",
    ),
    (
        "Mode pengujian: alarm tidak diputar.",
        "Test mode: alarm audio was not played.",
    ),
    (
        "Bunyi uji selesai. Jika tidak terdengar, periksa volume perangkat.",
        "Sound preview finished. If you could not hear it, check your device volume.",
    ),
    (
        "Pilihan alarm tidak valid.",
        "Invalid alarm sound selection.",
    ),
    ("Worker alarm berhenti.", "The alarm worker has stopped."),
    (
        "Alarm tidak dapat diputar. Periksa perangkat dan volume audio.",
        "The alarm could not play. Check your audio device and volume.",
    ),
    (
        "Alarm tidak tersedia. Periksa audio dan pasang paplay atau aplay pada Linux.",
        "Alarm audio is unavailable. Check audio and install paplay or aplay on Linux.",
    ),
    (
        "Cache alarm tidak valid. Hapus folder sounds di folder data lalu coba lagi.",
        "The alarm cache is invalid. Remove the sounds folder from the data directory and try again.",
    ),
    ("TUGAS SELESAI", "COMPLETED TASKS"),
    ("Mengirim...", "Sending..."),
    (
        "Mengirim notifikasi uji...",
        "Sending a test notification...",
    ),
    (
        "Worker notifikasi berhenti.",
        "The notification worker has stopped.",
    ),
    (
        "Mode pengujian: notifikasi desktop tidak dikirim.",
        "Test mode: no desktop notification was sent.",
    ),
    (
        "Permintaan notifikasi diterima Windows/desktop. Jika banner tidak muncul, periksa Do Not Disturb dan izin notifikasi Jikanda pada pengaturan OS.",
        "The desktop OS accepted the notification request. If no banner appears, check Do Not Disturb and Jikanda notification permissions in your OS settings.",
    ),
    ("Nama tampilan", "Display name"),
    (
        "Nama tampilan, maksimal 60 karakter",
        "Display name, up to 60 characters",
    ),
    (
        "Nama tampilan maksimal 60 karakter.",
        "Display names may contain up to 60 characters.",
    ),
    ("Opsional, mis. Yu", "Optional, e.g. Yu"),
    (
        "Disimpan di perangkat ini. Hanya tampil di kartu jika kamu memilihnya.",
        "Stored on this device. Only appears on cards when you choose to include it.",
    ),
    ("SESI TERSIMPAN", "SAVED SESSION"),
    ("Pilih dari Riwayat", "Choose from History"),
    ("Buat kartu", "Create card"),
    ("Tampilkan nama", "Show display name"),
    ("Tampilkan tugas selesai", "Show completed tasks"),
    (
        "Atur nama tampilan di Preferences / General.",
        "Set your display name in Preferences / General.",
    ),
    (
        "Pilih sesi tersimpan untuk membuat kartu.",
        "Choose a saved session to create a card.",
    ),
    (
        "Durasi aktual satu sesi. Catatan pribadi tidak ikut diekspor.",
        "Actual time from one session. Private notes are excluded.",
    ),
    ("Terputus", "Interrupted"),
    ("Nama sesi", "Session name"),
    (
        "Nama sesi, maksimal 120 karakter",
        "Session name, up to 120 characters",
    ),
    (
        "Nama sesi maksimal 120 karakter.",
        "Session names may contain up to 120 characters.",
    ),
    ("Sesi baru", "New session"),
    ("Mis. Belajar Rust", "e.g. Learn Rust"),
    ("Sesi tanpa nama", "Untitled session"),
    (
        "Belum ada tugas untuk sesi ini.",
        "No tasks for this session yet.",
    ),
    ("{} / {} tugas selesai", "{} / {} tasks completed"),
    (
        "Catatan lama, maksimal 160 karakter",
        "Legacy note, up to 160 characters",
    ),
    (
        "Tugas saat sesi dicatat",
        "Tasks recorded with this session",
    ),
    ("Mulai sesi baru?", "Start a new session?"),
    (
        "Nama dan daftar tugas saat ini akan dikosongkan. Riwayat sesi yang sudah tersimpan tetap utuh.",
        "The current session name and task list will be cleared. Saved session history will be kept.",
    ),
    ("Sesi fokus", "Focus session"),
    ("Tugas sesi", "Session tasks"),
    ("Fokus · menit", "Focus · min"),
    ("Istirahat · menit", "Break · min"),
    (
        "Mulai tiap segmen saat siap. Istirahat tidak dihitung.",
        "Start each segment when ready. Breaks are excluded.",
    ),
    (
        "Pilih tugas untuk sesi ini.",
        "Choose a task for this session.",
    ),
    ("Tugas baru…", "New task…"),
    (
        "Privat, lokal, dan open source.",
        "Private, local, and open source.",
    ),
    ("Tentang Jikanda", "About Jikanda"),
    (
        "Pendamping fokus desktop yang tenang, ringan, dan bekerja offline.",
        "A calm, lightweight desktop focus companion that works offline.",
    ),
    ("Lisensi dan atribusi", "License and credits"),
    (
        "Aplikasi: GPL-3.0-only. Slint: opsi GPLv3.\nTanpa jaminan; detail lisensi dan source ada di LICENSE dan LICENSING.md.\nPlus Jakarta Sans: SIL OFL 1.1. Aset brand disediakan maintainer.",
        "App: GPL-3.0-only. Slint: GPLv3 option.\nNo warranty; see LICENSE and LICENSING.md for license and source details.\nPlus Jakarta Sans: SIL OFL 1.1. Brand assets supplied by the maintainer.",
    ),
    ("Umum", "General"),
    ("Penyimpanan", "Storage"),
    ("Tentang", "About"),
    ("Bahasa dan tampilan", "Language and appearance"),
    ("Tema", "Theme"),
    ("Terang", "Light"),
    ("Gelap", "Dark"),
    ("Notifikasi", "Notifications"),
    ("Penyimpanan lokal", "Local storage"),
    (
        "Gunakan path absolut. Perubahan berlaku setelah disimpan.",
        "Use an absolute path. Changes apply after saving.",
    ),
    ("Tentang sesi", "Session behavior"),
    (
        "Bahasa, tema, dan opacity langsung tersimpan.",
        "Language, theme, and opacity save automatically.",
    ),
    ("Opacity timer mini", "Mini timer opacity"),
    (
        "Opacity timer mini harus 50–100%.",
        "Mini timer opacity must be 50–100%.",
    ),
    (
        "Transparansi seluruh timer. Minimum 50% agar kontrol tetap terlihat.",
        "Whole-window transparency. A 50% minimum keeps controls visible.",
    ),
    (
        "Opacity native belum tersedia di platform ini; timer tetap opaque.",
        "Native opacity is not available on this platform yet; the timer stays opaque.",
    ),
    (
        "Opacity tidak tersedia; timer tetap dapat digunakan.",
        "Opacity is unavailable; the timer can still be used.",
    ),
    ("WAKTU FOKUS TERCATAT", "RECORDED FOCUS TIME"),
    (
        "Memberi ruang untuk hal berarti.",
        "Making space for what matters.",
    ),
    ("RENTANG WAKTU", "DATE RANGE"),
    (
        "Tanggal awal / akhir · YYYY-MM-DD",
        "Start / end date · YYYY-MM-DD",
    ),
    ("FORMAT", "FORMAT"),
    ("GAYA", "STYLE"),
    ("CAPTION OPSIONAL", "OPTIONAL CAPTION"),
    (
        "Maksimal 100 karakter. Biarkan kosong untuk kalimat bawaan.",
        "Up to 100 characters. Leave blank for the default caption.",
    ),
    ("Simpan PNG", "Save PNG"),
    (
        "Lokal dan offline. Tidak ada unggahan otomatis.",
        "Local and offline. Nothing is uploaded automatically.",
    ),
    ("PRATINJAU", "PREVIEW"),
    (
        "Hanya waktu sesi tersimpan. Catatan dan daftar tugas tidak ikut diekspor.",
        "Saved session time only. Notes and tasks are not exported.",
    ),
    (
        "Ukuran render kartu tidak sesuai format pilihan.",
        "The rendered card size does not match the selected format.",
    ),
    ("TUGAS SESI", "SESSION TASKS"),
    (
        "Tugas baru, maksimal 120 karakter",
        "New task, up to 120 characters",
    ),
    ("Tambah", "Add"),
    (
        "Satu hal kecil untuk sesi ini. Tambahkan tugas saat kamu siap.",
        "One small thing for this session. Add a task when you are ready.",
    ),
    (
        "Tandai tugas selesai atau belum selesai",
        "Mark task complete or incomplete",
    ),
    ("Hapus tugas", "Delete task"),
    (
        "Pilih judul untuk fokus sesi. Tanpa deadline.",
        "Choose a title for this session. No deadlines.",
    ),
    (
        "Tulis tugas sepanjang 1–120 karakter.",
        "Enter a task with 1–120 characters.",
    ),
    (
        "Maksimal 32 tugas. Hapus tugas yang tidak diperlukan.",
        "Up to 32 tasks. Delete tasks you no longer need.",
    ),
    (
        "Daftar tugas lokal tidak valid.",
        "The local task list is invalid.",
    ),
    (
        "25 menit fokus · 5 menit istirahat",
        "25 minutes focus · 5 minutes break",
    ),
    (
        "50 menit · satu sesi fokus",
        "50 minutes · one focus session",
    ),
    (
        "4 siklus. Mulai setiap segmen saat kamu siap. Waktu istirahat tidak dihitung sebagai fokus.",
        "4 cycles. Start each segment when you are ready. Break time is not counted as focus.",
    ),
    (
        "Tanpa jeda paksa. Kamu tetap bisa menjeda atau menghentikan sesi.",
        "No forced breaks. You can still pause or stop your session.",
    ),
    (
        "Pomodoro: fokus 25 menit, istirahat 5 menit, 4 siklus. Deep Focus: 50 menit. Flow: tanpa batas waktu.",
        "Pomodoro: 25 minutes focus, 5 minutes break, 4 cycles. Deep Focus: 50 minutes. Flow: no time limit.",
    ),
    ("Fokus", "Focus"),
    ("Timer mini", "Mini timer"),
    ("Riwayat", "History"),
    ("Kartu", "Cards"),
    ("Preferensi", "Preferences"),
    ("RUANG PRIBADIMU", "YOUR OWN SPACE"),
    ("Mode terang", "Light mode"),
    ("Mode gelap", "Dark mode"),
    ("Simpan lagi", "Save again"),
    ("Minimalkan jendela", "Minimize window"),
    (
        "Maksimalkan atau pulihkan jendela",
        "Maximize or restore window",
    ),
    ("Keluar dari Jikanda", "Quit Jikanda"),
    (
        "Waktunya untuk yang berarti.",
        "Make time for what matters.",
    ),
    (
        "Pelan-pelan, satu hal penting dalam satu waktu.",
        "One meaningful thing at a time.",
    ),
    ("SELESAI", "COMPLETE"),
    ("Waktu sesi ", "Session time "),
    ("Hentikan sesi", "Stop session"),
    (
        "CATATAN SESI · OPSIONAL · MAKS. 160",
        "SESSION NOTE · OPTIONAL · MAX. 160",
    ),
    (
        "Catatan opsional, disimpan bersama sesi, maksimal 160 karakter",
        "Optional session note, up to 160 characters",
    ),
    (
        "Apa yang ingin kamu beri waktu?",
        "What would you like to make time for?",
    ),
    ("SESI FOKUS", "FOCUS SESSION"),
    ("Ruang singkat", "A little space"),
    ("Ruang panjang", "A little longer"),
    ("5 menit · satu sesi", "5 minutes · one session"),
    (
        "25 menit · tanpa jeda paksa",
        "25 minutes · no forced breaks",
    ),
    ("50 menit · satu sesi", "50 minutes · one session"),
    ("Durasi kustom · detik", "Custom duration · seconds"),
    (
        "Durasi fokus dalam detik, 1 sampai 86400",
        "Focus duration in seconds, 1 to 86400",
    ),
    ("Atur", "Set"),
    (
        "Satu sesi, tanpa jeda paksa.",
        "One session, no forced breaks.",
    ),
    ("Ikuti alurnya.", "Find your flow."),
    (
        "Waktu bertambah tanpa batas sesi. Istirahat lewat Jeda; pilih Selesai untuk menyimpan sesi yang kamu akhiri dengan sengaja.",
        "Time counts up with no limit. Pause for a break; choose Finish to save an intentionally completed session.",
    ),
    ("Selesai & simpan", "Finish & save"),
    ("Buka timer mini", "Open mini timer"),
    ("Minimalkan timer", "Minimize timer"),
    ("Kembali ke jendela utama", "Return to main window"),
    (
        "Sesi selesai. Terima kasih!",
        "Session complete. Thank you!",
    ),
    ("Mulai fokus", "Start focus"),
    ("Mulai istirahat", "Start break"),
    ("Jeda", "Pause"),
    ("Lanjutkan", "Resume"),
    (
        "Dijeda · tidur atau jeda waktu",
        "Paused · sleep or timing gap",
    ),
    ("Waktunya beristirahat", "Time for a break"),
    ("Siap untuk fokus berikutnya", "Ready for your next focus"),
    ("Siap saat kamu siap", "Ready when you are"),
    (
        "Istirahat tidak menambah waktu fokus",
        "Breaks do not add focus time",
    ),
    (
        "Satu hal, sepenuh perhatian",
        "One thing, your full attention",
    ),
    ("Dijeda · ambil waktumu", "Paused · take your time"),
    (
        "Dihentikan · sesi terputus",
        "Stopped · interrupted session",
    ),
    (
        "Sesi selesai · terima kasih sudah hadir",
        "Session complete · thank you for showing up",
    ),
    ("ISTIRAHAT", "BREAK"),
    ("Istirahat", "Break"),
    ("Siklus {} dari {} · {}", "Cycle {} of {} · {}"),
    (
        "Satu sesi tanpa jeda paksa",
        "One session, no forced breaks",
    ),
    (
        "Count-up · akhiri saat kamu siap",
        "Count up · finish when you are ready",
    ),
    (
        "Segmen fokus selesai. Mulai istirahat saat kamu siap.",
        "Focus segment complete. Start your break when you are ready.",
    ),
    (
        "Istirahat selesai. Mulai fokus berikutnya saat kamu siap.",
        "Break complete. Start your next focus when you are ready.",
    ),
    (
        "Sesi selesai. Terima kasih sudah memberi waktu untuk hal yang kamu pilih.",
        "Session complete. Thank you for making time for what you chose.",
    ),
    (
        "Sesi terlalu singkat untuk dicatat; tidak ada durasi yang dibuat-buat.",
        "The session was too short to record; no time was added.",
    ),
    (
        "Batas riwayat tercapai. Data sesi tetap ada di memori; hapus entri sebelum mencoba simpan lagi.",
        "History limit reached. This session remains in memory; delete an entry before saving again.",
    ),
    ("Menyimpan dan keluar…", "Saving and quitting…"),
    (
        "Menunggu penulisan data lokal selesai.",
        "Waiting for local data to finish saving.",
    ),
    ("Menyimpan di perangkat…", "Saving on this device…"),
    ("Perubahan belum tersimpan", "Unsaved changes"),
    ("Tersimpan lokal.", "Saved locally."),
    (
        "Aplikasi belum dapat memastikan semua perubahan tersimpan. Coba simpan lagi, atau buang perubahan yang belum tersimpan untuk keluar. Data tidak direset.",
        "Some changes could not be saved. Try again, or discard unsaved changes to quit. Stored data has not been reset.",
    ),
    ("Waktu yang kamu beri.", "The time you made."),
    ("Rayakan dengan caramu.", "Celebrate in your own way."),
    ("Buat ruang yang nyaman.", "Make yourself comfortable."),
    (
        "Waktu sesi tercatat, bukan ukuran nilai atau hasil kerjamu.",
        "Recorded session time, not a measure of your worth or output.",
    ),
    (
        "Bagikan satu sesi tersimpan. Pilih detail yang ingin ditampilkan.",
        "Share one saved session. Choose which details to include.",
    ),
    (
        "Preferensi disimpan di perangkat ini. Tidak ada akun atau sinkronisasi.",
        "Preferences stay on this device. No account or synchronization.",
    ),
    ("HARI INI", "TODAY"),
    ("7 HARI TERAKHIR", "LAST 7 DAYS"),
    ("Riwayat sesi", "Session history"),
    ("Belum ada sesi tersimpan.", "No saved sessions yet."),
    (
        "Mulai dari sedikit waktu untuk hal yang kamu pilih.",
        "Start with a little time for something you choose.",
    ),
    ("Ubah", "Edit"),
    ("Hapus", "Delete"),
    ("Sebelumnya", "Previous"),
    ("Berikutnya", "Next"),
    ("Hapus semua", "Delete all"),
    ("Hari ini", "Today"),
    ("7 hari", "7 days"),
    ("Rentang lain", "Custom range"),
    ("Tanggal awal YYYY-MM-DD", "Start date YYYY-MM-DD"),
    ("Tanggal akhir YYYY-MM-DD", "End date YYYY-MM-DD"),
    ("Kertas putih", "White paper"),
    ("Hitam arang", "Charcoal"),
    ("Sertakan nama pilihan saya", "Include my chosen name"),
    (
        "Nama opsional pada kartu, maksimal 60 karakter",
        "Optional card name, up to 60 characters",
    ),
    (
        "Kalimat opsional pada kartu, maksimal 100 karakter",
        "Optional card caption, up to 100 characters",
    ),
    ("Perbarui pratinjau", "Refresh preview"),
    ("Ekspor PNG", "Export PNG"),
    ("Buka folder ekspor", "Open export folder"),
    (
        "Tidak ada data contoh. Kartu hanya memakai sesi tersimpan dalam rentang pilihanmu.",
        "No sample data. Cards only use saved sessions in your chosen range.",
    ),
    ("Durasi default · menit", "Default durations · minutes"),
    ("Fokus interval", "Interval focus"),
    ("Siklus", "Cycles"),
    ("Menit Deep Focus", "Deep Focus minutes"),
    ("Menit fokus interval", "Interval focus minutes"),
    ("Menit istirahat", "Break minutes"),
    ("Jumlah siklus 1 sampai 12", "Number of cycles, 1 to 12"),
    (
        "Durasi baru berlaku pada sesi berikutnya. Perpindahan interval menunggu tombol mulai.",
        "New durations apply to the next session. Each interval segment starts when you choose.",
    ),
    (
        "Notifikasi desktop ketika segmen atau sesi selesai",
        "Desktop notifications when a segment or session ends",
    ),
    (
        "Minta timer mini selalu di atas (bergantung OS)",
        "Keep mini timer on top (subject to OS support)",
    ),
    (
        "Gunakan bingkai sistem pada timer mini",
        "Use system window frame for the mini timer",
    ),
    (
        "Gerakan dekoratif tidak digunakan. Tema terang/gelap tersedia di sidebar.",
        "No decorative motion. Light and dark themes are available in the sidebar.",
    ),
    (
        "Folder ekspor · path absolut",
        "Export folder · absolute path",
    ),
    ("Folder ekspor PNG", "PNG export folder"),
    ("Simpan preferensi", "Save preferences"),
    ("Uji notifikasi", "Test notification"),
    (
        "Data hanya berada di perangkat ini",
        "Your data stays on this device",
    ),
    ("Buka folder data", "Open data folder"),
    (
        "Sesi aktif tidak dipulihkan setelah crash/restart. Waktu tidur tidak ditambahkan; sesi dijeda dan dilanjutkan secara manual.",
        "Active sessions are not restored after a crash or restart. Sleep time is excluded; resume paused sessions manually.",
    ),
    (
        "Jikanda 0.1.0 · by Yu Prananda\nAplikasi: GPL-3.0-only · Slint: opsi GPLv3.\nTanpa jaminan; lihat LICENSE dan LICENSING.md untuk lisensi dan source.\nPlus Jakarta Sans: SIL OFL 1.1. Aset brand disediakan maintainer.",
        "Jikanda 0.1.0 · by Yu Prananda\nApp: GPL-3.0-only · Slint: GPLv3 option.\nNo warranty; see LICENSE and LICENSING.md for license and source details.\nPlus Jakarta Sans: SIL OFL 1.1. Brand assets supplied by the maintainer.",
    ),
    ("Coba simpan lagi", "Try saving again"),
    (
        "Waktu lokal · YYYY-MM-DD HH:MM:SS",
        "Local time · YYYY-MM-DD HH:MM:SS",
    ),
    ("Waktu mulai lokal", "Local start time"),
    ("Waktu selesai lokal", "Local end time"),
    ("Durasi fokus · detik", "Focus duration · seconds"),
    (
        "Durasi fokus yang benar dalam detik",
        "Correct focus duration in seconds",
    ),
    (
        "Catatan sesi, maksimal 160 karakter",
        "Session note, up to 160 characters",
    ),
    (
        "Sesi selesai (matikan untuk terputus)",
        "Completed session (off for interrupted)",
    ),
    ("Kembali", "Back"),
    ("Keluar tanpa simpan", "Quit without saving"),
    ("Buang sesi", "Discard session"),
    ("Simpan perubahan", "Save changes"),
    ("Ya, hapus", "Yes, delete"),
    ("Simpan & keluar", "Save & quit"),
    ("Simpan sesi", "Save session"),
    ("Data lokal tidak diubah.", "Local data was not changed."),
    (
        "Tutup jendela ini, lalu periksa folder data atau instance Jikanda lain.",
        "Close this window, then check the data folder or another Jikanda instance.",
    ),
    (
        "Jikanda — data tidak dibuka",
        "Jikanda — data could not be opened",
    ),
    (
        "Durasi fokus 1–86400 detik, istirahat 1–7200 detik, siklus 1–12.",
        "Focus: 1–86400 seconds; break: 1–7200 seconds; cycles: 1–12.",
    ),
    (
        "Folder ekspor harus berupa path absolut.",
        "The export folder must be an absolute path.",
    ),
    ("Entri riwayat tidak valid.", "Invalid history entry."),
    (
        "Versi data belum didukung; data tidak diubah.",
        "Unsupported data version; stored data was not changed.",
    ),
    ("ID berikutnya tidak valid.", "Invalid next record ID."),
    ("ID riwayat tidak valid.", "Invalid history record ID."),
    (
        "Batas 10000 entri tercapai; ekspor/cadangkan data sebelum melanjutkan.",
        "The 10000-entry limit was reached; export or back up your data before continuing.",
    ),
    ("<1 detik", "<1 second"),
    ("{seconds} detik", "{seconds} seconds"),
    ("{} menit", "{} minutes"),
    ("{} jam {} menit", "{} hours {} minutes"),
    ("0 detik", "0 seconds"),
    ("{} sesi", "{} sessions"),
    ("{} / {} · {} sesi", "{} / {} · {} sessions"),
    (
        "{} sesi dalam 7 hari · {} selesai · {} terputus yang dikonfirmasi",
        "{} sessions in 7 days · {} completed · {} confirmed interrupted",
    ),
    ("Selesai", "Completed"),
    ("Terputus · dikonfirmasi", "Interrupted · confirmed"),
    ("Koreksi sesi", "Edit session"),
    (
        "Durasi fokus boleh lebih pendek dari rentang waktu karena jeda/istirahat. Koreksi tanggal, durasi, catatan, atau status dengan sengaja.",
        "Focus duration may be shorter than the time range because of pauses and breaks. Correct the date, duration, note, or status deliberately.",
    ),
    ("Hapus entri ini?", "Delete this entry?"),
    (
        "Entri dan catatannya akan dihapus dari riwayat lokal. Tindakan ini tidak dapat dibatalkan.",
        "This entry and its note will be deleted from local history. This cannot be undone.",
    ),
    ("Hapus seluruh riwayat?", "Delete all history?"),
    (
        "Semua sesi tersimpan dan catatannya dihapus. Preferensi tetap dipertahankan. Kartu PNG yang pernah diekspor tidak dihapus.",
        "All saved sessions and notes will be deleted. Preferences and previously exported PNGs will be kept.",
    ),
    (
        "Preferensi diperbarui. Durasi sesi aktif tidak diubah.",
        "Preferences updated. The current session duration is unchanged.",
    ),
    (
        "Durasi harus berupa angka menit.",
        "Duration must be a number of minutes.",
    ),
    (
        "Durasi fokus 1 detik–24 jam, istirahat 1 detik–120 menit.",
        "Focus: 1 second–24 hours; break: 1 second–120 minutes.",
    ),
    (
        "Jumlah siklus harus 1–12.",
        "The number of cycles must be 1–12.",
    ),
    (
        "Waktu harus YYYY-MM-DD HH:MM:SS.",
        "Time must use YYYY-MM-DD HH:MM:SS.",
    ),
    (
        "Waktu lokal ambigu/tidak ada karena perubahan zona waktu. Pilih waktu lain.",
        "This local time is ambiguous or missing due to a time-zone change. Choose another time.",
    ),
    ("Entri tidak ditemukan.", "Entry not found."),
    ("Entri sudah dihapus.", "Entry already deleted."),
    (
        "Durasi fokus harus angka detik.",
        "Focus duration must be a number of seconds.",
    ),
    ("Durasi fokus tidak valid.", "Invalid focus duration."),
    (
        "Waktu selesai harus setelah mulai, dan durasi fokus tidak boleh melebihi rentang waktu.",
        "End time must follow start time, and focus duration must fit within the time range.",
    ),
    (
        "Catatan maksimal 160 karakter.",
        "Notes may contain up to 160 characters.",
    ),
    (
        "Batas 10000 entri tercapai. Hapus entri yang tidak diperlukan sebelum memulai sesi baru.",
        "The 10000-entry limit was reached. Delete unneeded entries before starting a new session.",
    ),
    (
        "Masukkan bilangan bulat 1–86400 detik.",
        "Enter a whole number from 1 to 86400 seconds.",
    ),
    (
        "Durasi fokus {seconds} detik.",
        "Focus duration: {seconds} seconds.",
    ),
    (
        "Simpan waktu yang sudah kamu beri?",
        "Save the time you made?",
    ),
    ("Akhiri sesi dan keluar?", "End the session and quit?"),
    (
        "Sesi dijeda. Simpan sebagai terputus, buang sesi ini, atau kembali untuk melanjutkan. Waktu istirahat tidak ikut dicatat.",
        "The session is paused. Save it as interrupted, discard it, or go back to resume. Break time is excluded.",
    ),
    (
        "Coba simpan dan keluar, atau buang perubahan yang belum tersimpan. Data yang sudah tersimpan tidak dihapus.",
        "Try saving and quitting, or discard unsaved changes. Previously saved data is kept.",
    ),
    (
        "Tunggu penyimpanan selesai, lalu coba keluar lagi.",
        "Wait for saving to finish, then try quitting again.",
    ),
    (
        "Tunggu data tersimpan sebelum mengekspor kartu.",
        "Wait for data to be saved before exporting a card.",
    ),
    (
        "Tanggal awal harus YYYY-MM-DD.",
        "Start date must use YYYY-MM-DD.",
    ),
    (
        "Tanggal akhir harus YYYY-MM-DD.",
        "End date must use YYYY-MM-DD.",
    ),
    (
        "Tanggal awal harus sebelum atau sama dengan tanggal akhir.",
        "Start date must be on or before the end date.",
    ),
    (
        "Belum ada waktu fokus tersimpan pada rentang ini.",
        "No saved focus time in this range yet.",
    ),
    (
        "Nama maksimal 60 karakter dan kalimat kartu maksimal 100 karakter.",
        "Names may contain up to 60 characters; card captions up to 100.",
    ),
    ("Ruang untuk yang berarti.", "Space for what matters."),
    ("waktu fokus tercatat · ", "recorded focus time · "),
    ("Membuat PNG lokal…", "Creating a local PNG…"),
    ("PNG tersimpan: {}", "PNG saved: {}"),
    ("Ekspor gagal: {error}", "Export failed: {error}"),
    (
        "Folder ekspor akan dibuat setelah ekspor pertama.",
        "The export folder will be created with your first export.",
    ),
    ("Folder ekspor tidak valid.", "Invalid export folder."),
    ("Ukuran buffer PNG tidak valid.", "Invalid PNG buffer size."),
    (
        "Ini notifikasi uji Jikanda. Tidak ada data sesi atau catatan yang disertakan.",
        "This is a Jikanda test notification. No session data or notes are included.",
    ),
    (
        "Notifikasi desktop tidak tersedia atau diblokir OS. Status sesi tetap tampil di Jikanda.",
        "Desktop notifications are unavailable or blocked by the OS. Session status is still shown in Jikanda.",
    ),
    (
        "Worker data berhenti; data belum tersimpan.",
        "The storage worker stopped; data has not been saved.",
    ),
    ("Worker ekspor berhenti.", "The export worker stopped."),
    (
        "Data belum tersimpan: {e}. Gunakan Coba simpan lagi.",
        "Data has not been saved: {e}. Use Try saving again.",
    ),
    (
        "Tidak dapat membaca data lokal: {e}",
        "Unable to read local data: {e}",
    ),
    (
        "Tidak dapat membuat folder ekspor: {e}",
        "Unable to create the export folder: {e}",
    ),
    (
        "Tidak dapat membuka folder data: {e}",
        "Unable to open the data folder: {e}",
    ),
    (
        "Tidak dapat membuka folder: {e}",
        "Unable to open the folder: {e}",
    ),
    (
        "Tidak dapat membuka lock: {e}",
        "Unable to open the data lock: {e}",
    ),
    (
        "Tidak dapat menjalankan worker data: {e}",
        "Unable to start the storage worker: {e}",
    ),
    (
        "Tidak dapat menulis file sementara: {e}",
        "Unable to write the temporary file: {e}",
    ),
    ("Gagal membaca data: {e}", "Unable to read data: {e}"),
    (
        "Data lokal rusak atau format tidak dikenali. Berkas tidak diubah; cadangkan sebelum memperbaiki.",
        "Local data is damaged or in an unknown format. The file was not changed; back it up before repairing it.",
    ),
    (
        "Berkas data melebihi batas 16 MiB; tidak diubah.",
        "The data file exceeds the 16 MiB limit; it was not changed.",
    ),
    (
        "Folder data sedang digunakan Jikanda lain atau tidak dapat dikunci.",
        "The data folder is in use by another Jikanda instance or could not be locked.",
    ),
    (
        "LOCALAPPDATA tidak tersedia.",
        "LOCALAPPDATA is unavailable.",
    ),
    ("HOME tidak tersedia.", "HOME is unavailable."),
    (
        "Timer mini meminta posisi selalu di atas. Seret header atau waktu untuk memindahkannya.",
        "The mini timer requests always-on-top. Drag its header or time to move it.",
    ),
    (
        "Wayland: pin mengikuti dukungan compositor. Seret header atau waktu untuk memindahkan timer.",
        "Wayland: pinning depends on compositor support. Drag the header or time to move the timer.",
    ),
    ("{date} · {}", "{date} · {}"),
    ("{} — {}", "{} — {}"),
];
