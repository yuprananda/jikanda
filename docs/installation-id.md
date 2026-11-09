# Instalasi Jikanda beta - Windows

Paket beta ini diuji pada **Windows 11 x64**. Windows 10, ARM64, dan Linux belum
divalidasi. Unduh dari [GitHub Releases resmi](https://github.com/yuprananda/jikanda/releases).

## Memasang

Unduh `Jikanda-0.1.0-beta.1-windows-x64-setup.exe`, lalu jalankan Setup.
Pemasangan hanya untuk pengguna saat ini, tanpa administrator, dengan shortcut
Start menu dan pilihan shortcut desktop. Lokasi default: `%LOCALAPPDATA%/Programs/Jikanda`.
Setup memeriksa ketersediaan runtime Microsoft Visual C++ x64.

### Alternatif portable

1. Unduh ZIP Windows, bukan arsip source.
2. Ekstrak seluruh ZIP ke folder permanen, misalnya `%LOCALAPPDATA%/Programs/Jikanda`.
3. Jalankan `jikanda.exe`, bukan dari dalam ZIP. Tidak perlu akun atau administrator.
4. Opsional: buka PowerShell di folder aplikasi untuk membuat shortcut Start menu:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File ./install-windows.ps1
```

Script hanya membuat shortcut pengguna saat ini dengan ikon dari executable.
Tidak ada autostart atau layanan background. Bila folder dipindah, hapus shortcut
lama lalu buat ulang. Paket beta belum ditandatangani dengan sertifikat publisher;
Windows mungkin memberi peringatan reputasi. Periksa asal unduhan dan checksum.
Jangan jalankan berkas yang asalnya tidak dapat kamu pastikan.

## Memeriksa checksum

Unduh `SHA256SUMS.txt` dari rilis yang sama dan bandingkan nilai ZIP:

```powershell
Get-FileHash ./Jikanda-0.1.0-beta.1-windows-x64.zip -Algorithm SHA256
```

## Penggunaan awal

- Bahasa default Inggris; ubah melalui Preferences > General > Language.
- Beri nama sesi, pilih mode, dan tambahkan tugas bila diperlukan.
- Pomodoro 25/5 menit, empat siklus; Deep Focus 50 menit; Flow menghitung naik.
- Mini timer dibuka dari sidebar. Close pada mini mengembalikan jendela utama.
- Alarm: Preferences > General > Completion alarm. Pilih bunyi atau matikan, lalu
  **Save preferences**. Preview sound merupakan pemutaran manual.
- Notifikasi desktop terpisah dari alarm. Periksa izin Windows/Do Not Disturb bila
  banner tidak terlihat; Test notification memberi hasil permintaan.
- Ekspor PNG satu sesi melalui History > Create card.

## Memperbarui

Untuk versi installer, tutup Jikanda lalu jalankan Setup versi baru. Untuk portable,
tutup Jikanda dahulu; Windows mengunci exe yang sedang terbuka. Ekstrak versi baru
lalu ganti berkas aplikasi. Data tetap di `%LOCALAPPDATA%/Jikanda`, terpisah dari
pemasangan. Cadangkan `state.json` sebelum mencoba beta. Bila aplikasi dipindahkan,
hapus shortcut lama lalu buat ulang.

## Menghapus

Versi installer: tutup aplikasi lalu buka Settings Windows > Apps > Jikanda > Uninstall.
Riwayat, pengaturan, dan ekspor tetap disimpan. Tidak ada autostart atau layanan background.

Versi portable: tutup aplikasi, hapus folder aplikasi dan shortcut Start menu. Riwayat tetap ada.
Jika ingin menghapus data juga, hapus `%LOCALAPPDATA%/Jikanda` secara sengaja setelah
mencadangkannya bila perlu. Identitas notifikasi aplikasi ada di registry
`HKCU/Software/Classes/AppUserModelId/YuPrananda.Jikanda` dan dapat dihapus saat uninstall.

## Bantuan

- Audio: periksa volume aplikasi/perangkat Windows dan coba Preview sound.
- Runtime MSVC hilang: gunakan Visual C++ Redistributable x64 resmi Microsoft;
  jangan mengunduh DLL dari situs acak. Tautan ada di panduan Inggris.
- Data rusak tidak direset otomatis. Cadangkan folder dan sertakan pesan error
  tanpa mengunggah riwayat pribadi.
- Minimum jendela utama 1000x680 logical px. Laporkan skala layar jika teks meluap.

[Laporkan masalah](https://github.com/yuprananda/jikanda/issues) |
[Kebijakan data dan waktu](privacy-and-storage.md)
