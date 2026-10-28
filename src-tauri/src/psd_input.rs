//! PSD v1 reader, byte ranges refer to the preserved original, never external paths.
use crate::image_input::{check_cancel, dimensions, invalid, Layer, Raster, MAX_DECODE, MAX_ICC};
use crate::{AppError, Result};
use std::{io::Read, sync::atomic::AtomicBool};

struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
    base: usize,
}
impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8], base: usize) -> Self {
        Self { bytes, at: 0, base }
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let end = self.at.checked_add(n).ok_or_else(invalid)?;
        let result = self.bytes.get(self.at..end).ok_or_else(invalid)?;
        self.at = end;
        Ok(result)
    }
    fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }
    fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_be_bytes(self.take(2)?.try_into().unwrap()))
    }
    fn i16(&mut self) -> Result<i16> {
        Ok(self.u16()? as i16)
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn i32(&mut self) -> Result<i32> {
        Ok(self.u32()? as i32)
    }
    fn section(&mut self) -> Result<Self> {
        let n = self.u32()? as usize;
        let base = self.base + self.at;
        Ok(Self::new(self.take(n)?, base))
    }
    fn remaining(&self) -> usize {
        self.bytes.len() - self.at
    }
    fn pascal(&mut self, pad: usize) -> Result<String> {
        let n = self.u8()? as usize;
        let s = String::from_utf8_lossy(self.take(n)?).into_owned();
        let padding = (pad - (n + 1) % pad) % pad;
        self.take(padding)?;
        Ok(s)
    }
}
pub(crate) fn read(bytes: &[u8], cancel: &AtomicBool) -> Result<Raster> {
    let mut r = Cursor::new(bytes, 0);
    if r.take(4)? != b"8BPS" {
        return Err(invalid());
    }
    if r.u16()? != 1 {
        return Err(AppError::new("unsupported_psb"));
    }
    if r.take(6)? != [0; 6] {
        return Err(invalid());
    }
    let channels = r.u16()? as usize;
    let height = r.u32()?;
    let width = r.u32()?;
    let depth = r.u16()?;
    let mode = r.u16()?;
    if ![8, 16].contains(&depth) || ![1, 3].contains(&mode) {
        return Err(AppError::new("unsupported_color"));
    }
    let base_channels = if mode == 3 { 3 } else { 1 };
    if channels < base_channels || channels > 16 {
        return Err(AppError::new("image_too_large"));
    }
    dimensions(width, height, 0)?;
    if width as usize * height as usize * 8 > crate::image_input::MAX_WORKING {
        return Err(AppError::new("image_too_large"));
    }
    let color_data = r.section()?;
    if color_data.remaining() != 0 {
        return Err(AppError::new("unsupported_color"));
    }
    let mut resources = r.section()?;
    let mut icc = vec![];
    let mut metadata = vec![];
    let mut resource_count = 0;
    while resources.remaining() > 0 {
        check_cancel(cancel)?;
        resource_count += 1;
        if resource_count > 4096 {
            return Err(AppError::new("image_too_large"));
        }
        if resources.take(4)? != b"8BIM" {
            return Err(invalid());
        }
        let id = resources.u16()?;
        resources.pascal(2)?;
        let size = resources.u32()? as usize;
        let payload = resources.take(size)?;
        if !size.is_multiple_of(2) {
            resources.take(1)?;
        }
        if id == 1039 {
            if size > MAX_ICC || !icc.is_empty() {
                return Err(AppError::new("invalid_icc"));
            }
            icc = payload.to_vec();
        }
        metadata.push(format!("resource:{id}:{size}"));
    }
    let mut layer_mask = r.section()?;
    let mut layers = vec![];
    let mut warnings = vec!["psd_cached_composite_only".into()];
    let mut merged_alpha = false;
    if layer_mask.remaining() > 0 {
        (layers, merged_alpha) = parse_layers(layer_mask.section()?, depth, cancel)?;
        if layer_mask.remaining() >= 4 {
            let global = layer_mask.section()?;
            if global.remaining() > 0 {
                warnings.push("psd_global_data_uninterpreted".into());
            }
        }
        while layer_mask.remaining() >= 12 {
            check_cancel(cancel)?;
            if layer_mask.take(4)? != b"8BIM" {
                return Err(invalid());
            }
            let key = layer_mask.take(4)?;
            let payload = layer_mask.section()?;
            let size = payload.remaining();
            if key == b"Lr16" && depth == 16 {
                if !layers.is_empty() {
                    return Err(invalid());
                }
                (layers, merged_alpha) = parse_layers(payload, depth, cancel)?;
            } else {
                warnings.push("psd_global_data_uninterpreted".into());
            }
            if !size.is_multiple_of(4) {
                layer_mask.take(4 - size % 4)?;
            }
        }
        // Remaining global mask/additional blocks are preserved, not interpreted.
        if layer_mask.remaining() > 0 {
            warnings.push("psd_global_data_uninterpreted".into());
        }
    }
    let bpc = depth as usize / 8;
    let plane_size = width as usize * height as usize * bpc;
    if plane_size
        .checked_mul(channels)
        .is_none_or(|n| n > MAX_DECODE)
    {
        return Err(AppError::new("image_too_large"));
    }
    let compression = r.u16()?;
    let mut rgba = vec![[0, 0, 0, 65535]; width as usize * height as usize];
    if compression > 3 {
        return Err(AppError::new("unsupported_compression"));
    }
    let mut lengths = vec![];
    if compression == 1 {
        for _ in 0..height as usize * channels {
            lengths.push(r.u16()? as usize);
        }
    }
    let zipped = if compression >= 2 {
        let mut decoded = inflate(r.take(r.remaining())?, plane_size * channels, cancel)?;
        if compression == 3 {
            prediction(&mut decoded, width, depth, cancel)?;
        }
        Some(decoded)
    } else {
        None
    };
    for channel in 0..channels {
        check_cancel(cancel)?;
        for y in 0..height as usize {
            check_cancel(cancel)?;
            let row = if let Some(ref data) = zipped {
                let offset = (channel * height as usize + y) * width as usize * bpc;
                data[offset..offset + width as usize * bpc].to_vec()
            } else if compression == 0 {
                r.take(width as usize * bpc)?.to_vec()
            } else {
                let n = lengths[channel * height as usize + y];
                unpack(r.take(n)?, width as usize * bpc)?
            };
            for x in 0..width as usize {
                let value = if depth == 16 {
                    u16::from_be_bytes([row[x * 2], row[x * 2 + 1]])
                } else {
                    u16::from(row[x]) * 257
                };
                let p = &mut rgba[y * width as usize + x];
                if mode == 1 && channel == 0 {
                    p[..3].fill(value);
                } else if channel < base_channels {
                    p[channel] = value;
                } else if channel == base_channels && merged_alpha {
                    p[3] = value;
                }
            }
        }
    }
    if r.remaining() != 0 {
        return Err(invalid());
    }
    // PSD merged transparency stores color against white. Undo that matte in
    // the encoded source space before ICC conversion; layer channels stay intact.
    if merged_alpha && channels > base_channels {
        for row in rgba.chunks_mut(width as usize) {
            check_cancel(cancel)?;
            for p in row {
                let alpha = u64::from(p[3]);
                for c in &mut p[..3] {
                    *c = ((65535 - u64::from(*c)) * 65535)
                        .checked_div(alpha)
                        .map(|v| 65535u64.saturating_sub(v).min(65535) as u16)
                        .unwrap_or(0);
                }
            }
        }
    }
    if layers
        .iter()
        .any(|l| l.mask || l.clipping || l.blend != "norm" || !l.additional.is_empty())
    {
        warnings.push("psd_layer_render_unverified".into());
    }
    if depth == 16 && layers.is_empty() {
        warnings.push("psd_16bit_layer_blocks_uninterpreted".into());
    }
    Ok(Raster {
        width,
        height,
        depth: depth as u8,
        mode: if mode == 3 { "RGB" } else { "Grayscale" }.into(),
        rgba,
        icc,
        metadata,
        layers,
        warnings,
        origin: "psd_embedded_merged_image".into(),
    })
}
fn decode(data: &[u8], w: u32, h: u32, depth: u16, cancel: &AtomicBool) -> Result<()> {
    let mut r = Cursor::new(data, 0);
    let compression = r.u16()?;
    let row_size = w as usize * (depth as usize / 8);
    match compression {
        0 => {
            r.take(row_size * h as usize)?;
        }
        1 => {
            let mut lengths = vec![];
            for _ in 0..h {
                lengths.push(r.u16()? as usize);
            }
            for n in lengths {
                check_cancel(cancel)?;
                unpack(r.take(n)?, row_size)?;
            }
        }
        2 | 3 => {
            let mut raw = inflate(r.take(r.remaining())?, row_size * h as usize, cancel)?;
            if compression == 3 {
                prediction(&mut raw, w, depth, cancel)?;
            }
        }
        _ => return Err(invalid()),
    }
    if r.remaining() != 0 {
        return Err(invalid());
    }
    Ok(())
}
fn inflate(data: &[u8], expected: usize, cancel: &AtomicBool) -> Result<Vec<u8>> {
    if expected > MAX_DECODE {
        return Err(AppError::new("image_too_large"));
    }
    let mut decoder = flate2::read::ZlibDecoder::new(data);
    let mut raw = Vec::with_capacity(expected);
    let mut block = [0; 65536];
    loop {
        check_cancel(cancel)?;
        let n = decoder.read(&mut block).map_err(|_| invalid())?;
        if n == 0 {
            break;
        }
        if raw.len() + n > expected {
            return Err(invalid());
        }
        raw.extend_from_slice(&block[..n]);
    }
    if raw.len() != expected || decoder.total_in() != data.len() as u64 {
        return Err(invalid());
    }
    Ok(raw)
}
fn prediction(raw: &mut [u8], w: u32, depth: u16, cancel: &AtomicBool) -> Result<()> {
    for row in raw.chunks_exact_mut(w as usize * (depth as usize / 8)) {
        check_cancel(cancel)?;
        if depth == 8 {
            for i in 1..row.len() {
                row[i] = row[i].wrapping_add(row[i - 1]);
            }
        } else {
            let mut previous = 0u16;
            for sample in row.as_chunks_mut::<2>().0 {
                previous = previous.wrapping_add(u16::from_be_bytes([sample[0], sample[1]]));
                sample.copy_from_slice(&previous.to_be_bytes());
            }
        }
    }
    Ok(())
}
fn unpack(data: &[u8], expected: usize) -> Result<Vec<u8>> {
    let mut r = Cursor::new(data, 0);
    let mut row = Vec::with_capacity(expected);
    while r.remaining() > 0 {
        let n = r.u8()? as i8;
        if n >= 0 {
            let size = n as usize + 1;
            if row.len() + size > expected {
                return Err(invalid());
            }
            row.extend_from_slice(r.take(size)?);
        } else if n != -128 {
            let count = (1 - i16::from(n)) as usize;
            let value = r.u8()?;
            if row.len() + count > expected {
                return Err(invalid());
            }
            row.resize(row.len() + count, value);
        }
    }
    if row.len() != expected {
        return Err(invalid());
    }
    Ok(row)
}

fn parse_layers(mut li: Cursor<'_>, depth: u16, cancel: &AtomicBool) -> Result<(Vec<Layer>, bool)> {
    let mut layers = vec![];
    let mut merged_alpha = false;
    if li.remaining() > 0 {
        let signed = li.i16()?;
        merged_alpha = signed < 0;
        let count = signed.unsigned_abs() as usize;
        if count > 1024 {
            return Err(AppError::new("image_too_large"));
        }
        let mut channel_lengths = vec![];
        let mut budget = 0usize;
        for index in 0..count {
            check_cancel(cancel)?;
            let start = li.base + li.at;
            let bounds = [li.i32()?, li.i32()?, li.i32()?, li.i32()?];
            let h = i64::from(bounds[2]) - i64::from(bounds[0]);
            let w = i64::from(bounds[3]) - i64::from(bounds[1]);
            if w < 0 || h < 0 || w > 30000 || h > 30000 {
                return Err(invalid());
            }
            let nc = li.u16()? as usize;
            if nc > 16 {
                return Err(AppError::new("image_too_large"));
            }
            let mut ids = vec![];
            let mut lengths = vec![];
            for _ in 0..nc {
                let id = li.i16()?;
                if ids.contains(&id) {
                    return Err(invalid());
                }
                ids.push(id);
                lengths.push(li.u32()? as usize);
            }
            budget = budget
                .checked_add(w as usize * h as usize * nc * (depth as usize / 8))
                .ok_or_else(invalid)?;
            if budget > MAX_DECODE {
                return Err(AppError::new("image_too_large"));
            }
            if li.take(4)? != b"8BIM" {
                return Err(invalid());
            }
            let blend = String::from_utf8_lossy(li.take(4)?).into_owned();
            let opacity = li.u8()?;
            let clipping = match li.u8()? {
                0 => false,
                1 => true,
                _ => return Err(invalid()),
            };
            let flags = li.u8()?;
            li.take(1)?;
            let mut extra = li.section()?;
            let mask_data = extra.section()?;
            let mask = mask_data.remaining() > 0;
            // Mask payload remains addressable in record_range; do not guess complex masks.
            extra.section()?;
            let mut name = extra.pascal(4)?;
            let mut group = 0;
            let mut additional = vec![];
            while extra.remaining() >= 12 {
                if additional.len() >= 256 {
                    return Err(AppError::new("image_too_large"));
                }
                if extra.take(4)? != b"8BIM" {
                    return Err(invalid());
                }
                let key = String::from_utf8_lossy(extra.take(4)?).into_owned();
                let mut payload = extra.section()?;
                let n = payload.remaining();
                if key == "luni" {
                    let units = payload.u32()? as usize;
                    if units > 4096 {
                        return Err(AppError::new("image_too_large"));
                    }
                    let mut utf = vec![];
                    for _ in 0..units {
                        utf.push(payload.u16()?);
                    }
                    name = String::from_utf16(&utf).map_err(|_| invalid())?;
                }
                if key == "lsct" || key == "lsdk" {
                    group = payload.u32()?;
                    if group > 3 {
                        return Err(invalid());
                    }
                }
                additional.push(key);
                if n % 2 != 0 {
                    extra.take(1)?;
                }
            }
            if extra.remaining() > 0 && extra.take(extra.remaining())?.iter().any(|v| *v != 0) {
                return Err(invalid());
            }
            channel_lengths.push((lengths, w as u32, h as u32));
            layers.push(Layer {
                index,
                name,
                bounds,
                visible: flags & 2 == 0,
                opacity,
                blend,
                clipping,
                group,
                channels: ids,
                mask,
                additional,
                record_range: [start, li.base + li.at],
                channel_range: [0, 0],
            });
        }
        let mut group_depth = 0usize;
        for (layer, (lengths, w, h)) in layers.iter_mut().zip(channel_lengths) {
            check_cancel(cancel)?;
            let start = li.base + li.at;
            for (id, n) in layer.channels.iter().zip(lengths) {
                let data = li.take(n)?;
                // Mask channels may have different dimensions. Preserve without falsely validating pixels.
                if *id >= -1 && w > 0 && h > 0 {
                    decode(data, w, h, depth, cancel)?;
                }
            }
            layer.channel_range = [start, li.base + li.at];
            // PSD records are stored bottom-to-top: the bounding divider precedes its folder.
            if layer.group == 3 {
                group_depth += 1;
            } else if layer.group == 1 || layer.group == 2 {
                group_depth = group_depth.checked_sub(1).ok_or_else(invalid)?;
            }
            if group_depth > 64 {
                return Err(AppError::new("image_too_large"));
            }
        }
        if group_depth != 0 {
            return Err(invalid());
        }
        if li.remaining() > 3 || li.take(li.remaining())?.iter().any(|v| *v != 0) {
            return Err(invalid());
        }
    }

    Ok((layers, merged_alpha))
}
