// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

fn main() {
    slint_build::compile("ui/app.slint").expect("Slint UI compilation failed");
    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=assets/brand/exports/jikanda.ico");
        winresource::WindowsResource::new()
            .set_icon("assets/brand/exports/jikanda.ico")
            .set("ProductName", "Jikanda")
            .set("FileDescription", "Jikanda Desktop Focus Companion")
            .set("CompanyName", "Yu Prananda")
            .set(
                "LegalCopyright",
                "Copyright (C) 2026 Yu Prananda. GPL-3.0-only.",
            )
            .compile()
            .expect("Windows icon and version resources failed");
    }
}
