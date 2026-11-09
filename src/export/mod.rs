// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs::File, io::BufWriter, path::Path};

pub fn write_pixels(
    path: &Path,
    pixels: &slint::SharedPixelBuffer<slint::Rgba8Pixel>,
) -> Result<(), Box<dyn std::error::Error>> {
    encode(path, pixels.as_bytes(), pixels.width(), pixels.height()).map_err(Into::into)
}

pub fn encode(path: &Path, pixels: &[u8], width: u32, height: u32) -> Result<(), String> {
    let expected = (width as usize)
        .checked_mul(height as usize)
        .and_then(|n| n.checked_mul(4));
    if width == 0 || height == 0 || expected != Some(pixels.len()) {
        return Err("Ukuran buffer PNG tidak valid.".into());
    }
    // Create first: a collision must never delete or overwrite an existing file.
    let file = File::create_new(path).map_err(|e| e.to_string())?;
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let mut encoder = png::Encoder::new(BufWriter::new(file), width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(pixels)?;
        writer.finish()?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(path);
    }
    result.map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn path() -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "jikanda-png-{}-{}.png",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap()
        ))
    }
    #[test]
    fn png_roundtrip_and_existing_file_is_preserved() {
        let path = path();
        let pixels = [255, 0, 0, 255, 0, 255, 0, 255];
        encode(&path, &pixels, 2, 1).unwrap();
        let before = std::fs::read(&path).unwrap();
        assert!(encode(&path, &pixels, 2, 1).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), before);
        let mut decoder = png::Decoder::new(std::io::BufReader::new(File::open(&path).unwrap()))
            .read_info()
            .unwrap();
        let mut output = vec![0; decoder.output_buffer_size().unwrap()];
        let info = decoder.next_frame(&mut output).unwrap();
        assert_eq!((info.width, info.height), (2, 1));
        assert_eq!(&output[..info.buffer_size()], &pixels);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn invalid_buffer_never_creates_a_file() {
        let path = path();
        assert!(encode(&path, &[], 960, 540).is_err());
        assert!(!path.exists());
        assert!(encode(&path, &[], u32::MAX, u32::MAX).is_err());
        assert!(!path.exists());
    }
}
