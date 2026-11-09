# Lisensi Jikanda

Copyright (C) 2026 Yu Prananda.

Aplikasi dan source yang dibuat untuk Jikanda menggunakan **GPL-3.0-only**, sesuai
keputusan maintainer 6 November 2026. Teks lengkap tanpa modifikasi ada di
[LICENSE](LICENSE). Program diberikan tanpa jaminan sesuai GPLv3. Label beta tidak
membatasi hak pengguna yang diberikan oleh lisensi.

Slint 1.18.1 digunakan melalui opsi **GPLv3**, bukan opsi Royalty-free atau komersial.
Cabang lisensi untuk dependensi Windows dicatat dalam [review lisensi](docs/license-review.md).
Teks upstream dan provenance berada di `docs/licenses`.

## Aset dan atribusi

- Plus Jakarta Sans: **SIL OFL 1.1**; teks asli ada di `assets/fonts/OFL-PlusJakartaSans.txt`.
- Mascot/wordmark: artwork yang disediakan maintainer. Penggunaan bentuk yang sama
  untuk beta GitHub dikonfirmasi 9 November 2026. Aset proyek dibagikan di bawah
  GPL-3.0-only; tidak ada klaim bahwa merek telah mendapat clearance hukum.
  Ikon raster beta berasal dari sheet asli, bukan tracing atau pengganti logo.
- Tiga WAV alarm: sinyal sintesis asli GPL-3.0-only, tanpa sampel/rekaman pihak lain.
  Generator dan notice ada di `scripts/generate-alarms.py` serta `assets/audio/README.md`.
- Ilustrasi scene historis dibuat untuk proyek; tidak ditampilkan dalam UI monokrom
  saat ini. Provenance dan prompt ada di `docs/visual-prototype.md`.

Lisensi proyek tidak menghapus copyright, notice, atau syarat lisensi upstream.
Source vendor mempertahankan header dan file upstream. Untuk beberapa crate
historis yang tidak dipakai Windows dan tidak membawa teks standalone, deklarasi
lisensi/author Cargo, source asli, dan teks standar ber-provenance dicatat terpisah;
lihat `DECLARATION.json` atau `NOTICE.txt` pada direktori notice terkait.

## Corresponding Source saat distribusi binary

Rilis Windows beta menyediakan **arsip source terpisah di rilis yang sama**, dengan
source aplikasi yang sesuai, Cargo.lock, aset asli dan generator, skrip build,
notices, serta seluruh crate Cargo yang divendor. Konfigurasi vendor di arsip
memungkinkan `cargo build --release --locked --offline` setelah toolchain dan
system build tools dipasang.

Rust/MSVC/Windows SDK serta system libraries tidak dibundel. Arsip source tidak
mengklaim hasil build identik byte-for-byte pada versi toolchain berbeda.
Jangan mengganti kewajiban penyediaan source dengan Cargo.lock atau URL umum saja.
Distribusikan source yang sesuai bersama akses binary dan pertahankan notices.

Review ini mencatat pilihan dan bahan distribusi teknis, bukan jaminan hukum atau
trademark clearance. Inventory source lintas platform tidak berarti semuanya
terhubung ke binary Windows.

Referensi:
- [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.html), terutama bagian 1 dan 4-6.
- [Opsi lisensi Slint](https://slint.dev/terms-and-conditions).
- [Kompatibilitas Apache 2.0 dan GPLv3](https://www.apache.org/licenses/GPL-compatibility).
