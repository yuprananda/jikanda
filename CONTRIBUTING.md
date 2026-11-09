# Berkontribusi pada Jikanda

Baca AGENTS.md sebelum mengubah kode. Jikanda adalah desktop companion lokal,
bukan layanan cloud, task manager, atau platform tim. Perubahan scope, nama,
lisensi, aset brand, dan dependensi perlu keputusan maintainer.

Gunakan Rust stable ≥1.92, Slint 1.18.1, Cargo.lock, dan software renderer baseline.
Pemisahan modul: domain (tanpa UI), application, persistence, platform, presentation,
export, dan worker I/O. Domain menerima timestamp monotonic yang dapat diuji;
jangan mengukur waktu dengan jumlah tick render.

Sebelum mengirim perubahan, jalankan fmt, Clippy, test, dan release build sesuai
docs/release-readiness.md. Untuk perubahan UI, periksa snapshot pada ukuran minimum
dan tema terang/gelap. Sebutkan OS/compositor/DPI serta apa yang belum diuji.
Jangan memakai history atau catatan pribadi dalam fixture, log, screenshot, issue,
atau paket. Gunakan --data-dir absolut terpisah atau mode verifikasi terisolasi.

Kode dan kontribusi Jikanda menggunakan GPL-3.0-only; lihat LICENSE dan LICENSING.md.
Sertakan SPDX-License-Identifier yang sesuai pada file source baru. Jangan mengirim
aset yang tidak memiliki izin yang jelas. Sertakan sumber dan lisensi font/artwork.
Jangan mengganti mascot dengan ikon generik atau men-trace sheet secara otomatis.

Untuk laporan bug, sertakan langkah reproduksi, versi/build, OS, display server,
scaling, dan pesan kesalahan yang sudah dibersihkan dari informasi pribadi.
