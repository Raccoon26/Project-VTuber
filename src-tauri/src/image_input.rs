//! Bounded source inspection. Original bytes never pass through an encoder.
use crate::{storage::hash, AppError, Result};
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};

pub const MAX_FILE: usize = 64 * 1024 * 1024;
pub const MAX_DECODE: usize = 160 * 1024 * 1024;
pub const MAX_WORKING: usize = 320 * 1024 * 1024;
pub const MAX_ICC: usize = 4 * 1024 * 1024;
pub fn invalid() -> AppError {
    AppError::new("invalid_image")
}
pub fn check_cancel(cancel: &AtomicBool) -> Result<()> {
    if cancel.load(Ordering::Relaxed) {
        Err(AppError::new("import_cancelled"))
    } else {
        Ok(())
    }
}
pub fn dimensions(w: u32, h: u32, bytes: usize) -> Result<()> {
    if w == 0 || h == 0 || u64::from(w) * u64::from(h) > 40_000_000 || bytes > MAX_DECODE {
        Err(AppError::new("image_too_large"))
    } else {
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct Layer {
    pub index: usize,
    pub name: String,
    pub bounds: [i32; 4],
    pub visible: bool,
    pub opacity: u8,
    pub blend: String,
    pub clipping: bool,
    pub group: u32,
    pub channels: Vec<i16>,
    pub mask: bool,
    pub additional: Vec<String>,
    pub record_range: [usize; 2],
    pub channel_range: [usize; 2],
}
#[derive(Debug, Serialize)]
pub struct Inspection {
    pub version: u32,
    pub parser: String,
    pub source_hash: String,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_mode: String,
    pub icc_hash: Option<String>,
    pub icc_bytes: usize,
    pub color_status: String,
    pub metadata: Vec<String>,
    pub layers: Vec<Layer>,
    pub warnings: Vec<String>,
    pub preview_width: u32,
    pub preview_height: u32,
    pub preview_png: Vec<u8>,
    pub preview_origin: String,
    pub precision: String,
    pub decode_ms: u64,
    pub preview_ms: u64,
}
pub(crate) struct Raster {
    pub width: u32,
    pub height: u32,
    pub depth: u8,
    pub mode: String,
    pub rgba: Vec<[u16; 4]>,
    pub icc: Vec<u8>,
    pub metadata: Vec<String>,
    pub layers: Vec<Layer>,
    pub warnings: Vec<String>,
    pub origin: String,
}
pub fn inspect(bytes: &[u8], cancel: &AtomicBool) -> Result<Inspection> {
    let started = std::time::Instant::now();
    check_cancel(cancel)?;
    if bytes.len() > MAX_FILE {
        return Err(AppError::new("image_too_large"));
    }
    let (format, mut raster) = if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        ("png", read_png(bytes, cancel)?)
    } else if bytes.starts_with(b"8BPS") {
        ("psd", crate::psd_input::read(bytes, cancel)?)
    } else {
        return Err(invalid());
    };
    let decode_ms = started.elapsed().as_millis() as u64;
    let preview_started = std::time::Instant::now();
    check_cancel(cancel)?;
    let color_status = if raster.icc.is_empty() {
        if raster.metadata.iter().any(|x| x == "sRGB") {
            "srgb_declared"
        } else {
            raster.warnings.push("color_assumed".into());
            "srgb_assumed"
        }
    } else {
        if raster.icc.len() > MAX_ICC {
            return Err(AppError::new("image_too_large"));
        }
        let input =
            lcms2::Profile::new_icc(&raster.icc).map_err(|_| AppError::new("invalid_icc"))?;
        if input.color_space() != lcms2::ColorSpaceSignature::RgbData {
            return Err(AppError::new("unsupported_color"));
        }
        let transform = lcms2::Transform::<[u16; 3], [u16; 3]>::new(
            &input,
            lcms2::PixelFormat::RGB_16,
            &lcms2::Profile::new_srgb(),
            lcms2::PixelFormat::RGB_16,
            lcms2::Intent::RelativeColorimetric,
        )
        .map_err(|_| AppError::new("invalid_icc"))?;
        // Row-sized RGB buffer: alpha is straight and never transformed/premultiplied.
        for row in raster.rgba.chunks_mut(raster.width as usize) {
            check_cancel(cancel)?;
            let mut rgb: Vec<[u16; 3]> = row.iter().map(|p| [p[0], p[1], p[2]]).collect();
            transform.transform_in_place(&mut rgb);
            for (p, c) in row.iter_mut().zip(rgb) {
                p[..3].copy_from_slice(&c);
            }
        }
        "icc_to_srgb"
    }
    .to_string();
    let scale = raster.width.max(raster.height).div_ceil(1024).max(1);
    let pw = raster.width.div_ceil(scale);
    let ph = raster.height.div_ceil(scale);
    let mut preview = Vec::with_capacity((pw * ph * 4) as usize);
    for y in 0..ph {
        check_cancel(cancel)?;
        for x in 0..pw {
            let p = raster.rgba[((y * scale) * raster.width + x * scale) as usize];
            preview.extend(p.map(|v| ((u32::from(v) + 128) / 257) as u8));
        }
    }
    let mut encoded = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut encoded, pw, ph);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_srgb(png::SrgbRenderingIntent::RelativeColorimetric);
        encoder
            .write_header()
            .map_err(|_| invalid())?
            .write_image_data(&preview)
            .map_err(|_| invalid())?;
    }
    Ok(Inspection {
        version: 1,
        parser: "bounded-rust-1/png-0.17.16/lcms2-6.2.0".into(),
        source_hash: hash(bytes),
        format: format.into(),
        width: raster.width,
        height: raster.height,
        bit_depth: raster.depth,
        color_mode: raster.mode,
        icc_hash: (!raster.icc.is_empty()).then(|| hash(&raster.icc)),
        icc_bytes: raster.icc.len(),
        color_status,
        metadata: raster.metadata,
        layers: raster.layers,
        warnings: raster.warnings,
        preview_width: pw,
        preview_height: ph,
        preview_png: encoded,
        preview_origin: raster.origin,
        precision: "derived_srgb_rgba8_nearest_original_unchanged".into(),
        decode_ms,
        preview_ms: preview_started.elapsed().as_millis() as u64,
    })
}
fn read_png(bytes: &[u8], cancel: &AtomicBool) -> Result<Raster> {
    let mut decoder = png::Decoder::new(bytes);
    decoder.set_limits(png::Limits {
        bytes: 192 * 1024 * 1024,
    });
    decoder.set_transformations(png::Transformations::EXPAND);
    let mut reader = decoder.read_info().map_err(|_| invalid())?;
    let info = reader.info();
    let (width, height, depth, mode) = (
        info.width,
        info.height,
        info.bit_depth as u8,
        format!("{:?}", info.color_type),
    );
    dimensions(width, height, reader.output_buffer_size())?;
    // RGBA16 work buffer is separately bounded, even for tiny compressed indexed inputs.
    dimensions(width, height, 0)?;
    if width as usize * height as usize * 8 > MAX_WORKING {
        return Err(AppError::new("image_too_large"));
    }
    if info.animation_control.is_some() {
        return Err(AppError::new("unsupported_image"));
    }
    let icc = info.icc_profile.as_deref().unwrap_or_default().to_vec();
    if icc.len() > MAX_ICC {
        return Err(AppError::new("image_too_large"));
    }
    let mut metadata = Vec::new();
    if info.srgb.is_some() {
        metadata.push("sRGB".into());
    }
    if let Some(g) = info.source_gamma {
        metadata.push(format!("gAMA:{}", g.into_scaled()));
    }
    if info.source_chromaticities.is_some() {
        metadata.push("cHRM".into());
    }
    let mut raw = vec![0; reader.output_buffer_size()];
    let output = reader.next_frame(&mut raw).map_err(|_| invalid())?;
    reader.finish().map_err(|_| invalid())?;
    let channels = output.color_type.samples();
    let step = if output.bit_depth == png::BitDepth::Sixteen {
        2
    } else {
        1
    };
    let mut rgba = Vec::with_capacity(width as usize * height as usize);
    for (i, pixel) in raw[..output.buffer_size()]
        .chunks_exact(channels * step)
        .enumerate()
    {
        if i % width as usize == 0 {
            check_cancel(cancel)?;
        }
        let mut values = [65535; 4];
        for c in 0..channels {
            values[c] = if step == 2 {
                u16::from_be_bytes([pixel[c * 2], pixel[c * 2 + 1]])
            } else {
                u16::from(pixel[c]) * 257
            };
        }
        rgba.push(match output.color_type {
            png::ColorType::Grayscale => [values[0], values[0], values[0], 65535],
            png::ColorType::GrayscaleAlpha => [values[0], values[0], values[0], values[1]],
            png::ColorType::Rgb => [values[0], values[1], values[2], 65535],
            png::ColorType::Rgba => values,
            _ => return Err(invalid()),
        });
    }
    let warnings = if metadata
        .iter()
        .any(|x| x.starts_with("gAMA") || x == "cHRM")
        && icc.is_empty()
        && !metadata.iter().any(|x| x == "sRGB")
    {
        vec!["gamma_chromaticities_preview_unverified".into()]
    } else {
        vec![]
    };
    Ok(Raster {
        width,
        height,
        depth,
        mode,
        rgba,
        icc,
        metadata,
        layers: vec![],
        warnings,
        origin: "png_decoded".into(),
    })
}
