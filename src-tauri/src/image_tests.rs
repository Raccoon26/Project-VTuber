use crate::{
    image_input::inspect,
    storage::{hash, ProjectStore},
};
use std::{fs, sync::atomic::AtomicBool};
fn off() -> AtomicBool {
    AtomicBool::new(false)
}
fn png_fixture(color: png::ColorType, depth: png::BitDepth, raw: &[u8], icc: bool) -> Vec<u8> {
    let mut output = vec![];
    let mut info = png::Info::with_size(2, 1);
    info.color_type = color;
    info.bit_depth = depth;
    if color == png::ColorType::Indexed {
        info.palette = Some(vec![255, 0, 0, 0, 255, 0].into());
        info.trns = Some(vec![0, 128].into());
    }
    if icc {
        info.icc_profile = Some(lcms2::Profile::new_srgb().icc().unwrap().into());
    }
    let encoder = png::Encoder::with_info(&mut output, info).unwrap();
    encoder
        .write_header()
        .unwrap()
        .write_image_data(raw)
        .unwrap();
    output
}
fn decoded(bytes: &[u8]) -> Vec<u8> {
    let mut reader = png::Decoder::new(bytes).read_info().unwrap();
    let mut raw = vec![0; reader.output_buffer_size()];
    let frame = reader.next_frame(&mut raw).unwrap();
    if frame.color_type == png::ColorType::Rgb {
        raw.as_chunks::<3>()
            .0
            .iter()
            .flat_map(|c| [c[0], c[1], c[2], 255])
            .collect()
    } else {
        raw
    }
}
#[test]
fn png_modes_precision_transparency_and_icc() {
    let cases = [
        (
            png::ColorType::Rgba,
            png::BitDepth::Eight,
            vec![255, 0, 100, 0, 10, 20, 30, 128],
            vec![255, 0, 100, 0, 10, 20, 30, 128],
        ),
        (
            png::ColorType::Rgb,
            png::BitDepth::Eight,
            vec![1, 2, 3, 4, 5, 6],
            vec![1, 2, 3, 255, 4, 5, 6, 255],
        ),
        (
            png::ColorType::Grayscale,
            png::BitDepth::Eight,
            vec![0, 255],
            vec![0, 0, 0, 255, 255, 255, 255, 255],
        ),
        (
            png::ColorType::GrayscaleAlpha,
            png::BitDepth::Eight,
            vec![30, 0, 100, 128],
            vec![30, 30, 30, 0, 100, 100, 100, 128],
        ),
        (
            png::ColorType::Indexed,
            png::BitDepth::One,
            vec![0b01000000],
            vec![255, 0, 0, 0, 0, 255, 0, 128],
        ),
        (
            png::ColorType::Rgb,
            png::BitDepth::Sixteen,
            vec![0, 0, 128, 0, 255, 255, 255, 255, 0, 0, 0, 0],
            vec![0, 128, 255, 255, 255, 0, 0, 255],
        ),
    ];
    for (color, depth, raw, expected) in cases {
        let bytes = png_fixture(color, depth, &raw, false);
        let before = hash(&bytes);
        let result = inspect(&bytes, &off()).unwrap();
        assert_eq!(result.source_hash, before);
        assert_eq!(decoded(&result.preview_png), expected);
        assert_eq!(hash(&bytes), before);
    }
    let bytes = png_fixture(
        png::ColorType::Rgb,
        png::BitDepth::Sixteen,
        &[0, 0, 128, 0, 255, 255, 255, 255, 0, 0, 0, 0],
        true,
    );
    let result = inspect(&bytes, &off()).unwrap();
    assert_eq!(result.color_status, "icc_to_srgb");
    assert!(result.icc_hash.is_some());
    assert_eq!(result.bit_depth, 16);
}
fn u16v(v: u16) -> Vec<u8> {
    v.to_be_bytes().to_vec()
}
fn u32v(v: u32) -> Vec<u8> {
    v.to_be_bytes().to_vec()
}
fn section(data: Vec<u8>) -> Vec<u8> {
    [u32v(data.len() as u32), data].concat()
}
pub fn psd_fixture(depth: u16, layered: bool) -> Vec<u8> {
    let mut result = [
        b"8BPS".to_vec(),
        u16v(1),
        vec![0; 6],
        u16v(3),
        u32v(1),
        u32v(2),
        u16v(depth),
        u16v(3),
        u32v(0),
        u32v(0),
    ]
    .concat();
    let mut info = vec![];
    if layered {
        info.extend(u16v(2));
        for (i, name) in ["目・눈", "Hidden"].iter().enumerate() {
            info.extend(
                [
                    0i32.to_be_bytes(),
                    0i32.to_be_bytes(),
                    1i32.to_be_bytes(),
                    2i32.to_be_bytes(),
                ]
                .concat(),
            );
            info.extend(u16v(1));
            info.extend(u16v(0));
            info.extend(u32v(2 + 2 * (depth as u32 / 8)));
            info.extend(b"8BIMnorm");
            info.extend([128, i as u8, if i == 1 { 2 } else { 0 }, 0]);
            let unicode = name.encode_utf16().collect::<Vec<_>>();
            let mut payload = u32v(unicode.len() as u32);
            for c in unicode {
                payload.extend(u16v(c));
            }
            let extra = [
                u32v(0),
                u32v(0),
                vec![1, b'L', 0, 0],
                b"8BIMluni".to_vec(),
                section(payload),
            ]
            .concat();
            info.extend(section(extra));
        }
        for _ in 0..2 {
            info.extend(u16v(0));
            info.extend(vec![100; 2 * (depth as usize / 8)]);
        }
    }
    result.extend(if layered {
        section([section(info), u32v(0)].concat())
    } else {
        u32v(0)
    });
    result.extend(u16v(0));
    for color in [255u8, 128, 0] {
        for _ in 0..2 {
            result.extend(vec![color; depth as usize / 8]);
        }
    }
    result
}
#[test]
fn psd_source_layers_16bit_and_v1_history_are_preserved() {
    for depth in [8, 16] {
        let bytes = psd_fixture(depth, true);
        let result = inspect(&bytes, &off()).unwrap();
        assert_eq!(result.layers.len(), 2);
        assert_eq!(result.layers[0].name, "目・눈");
        assert!(!result.layers[1].visible);
        assert!(result.layers[1].clipping);
        assert_eq!(result.bit_depth, depth as u8);
        assert_eq!(
            decoded(&result.preview_png),
            [255, 128, 0, 255, 255, 128, 0, 255]
        );
        let temp = tempfile::tempdir().unwrap();
        let store = ProjectStore::new(temp.path().join("projects")).unwrap();
        let p = store
            .create("日本・한국".into(), "2d".into(), "vbridger-vts".into())
            .unwrap();
        let before = fs::read(
            temp.path()
                .join("projects")
                .join(&p.id)
                .join("00000001.json"),
        )
        .unwrap();
        let path = temp.path().join("原画.psd");
        fs::write(&path, &bytes).unwrap();
        let saved = store.import_image(&p.id, 1, &path, &off()).unwrap();
        assert_eq!(saved.schema_version, 2);
        assert_eq!(saved.sources[0].format, "psd");
        assert_eq!(store.open(&p.id).unwrap().revision, 2);
        assert_eq!(store.list_v2().unwrap().healthy.len(), 1);
        let base = temp.path().join("projects").join(&p.id);
        assert_eq!(fs::read(base.join("00000001.json")).unwrap(), before);
        assert_eq!(
            fs::read(base.join("sources").join(format!("{}.psd", hash(&bytes)))).unwrap(),
            bytes
        );
        let preview = store.preview_recovery(&p.id, 2).unwrap();
        assert_eq!(store.recover(&p.id, 2, &preview.token).unwrap().revision, 3);
        assert_eq!(fs::read(path).unwrap(), bytes);
    }
}
#[test]
fn malformed_truncated_oversized_cancelled_inputs_do_not_panic_or_commit() {
    let psd = psd_fixture(8, true);
    for length in 0..psd.len() {
        assert!(
            inspect(&psd[..length], &off()).is_err(),
            "truncated prefix {length}"
        );
    }
    let mut huge = psd.clone();
    huge[14..18].copy_from_slice(&u32::MAX.to_be_bytes());
    assert!(inspect(&huge, &off()).is_err());
    let mut psb = psd.clone();
    psb[5] = 2;
    assert_eq!(inspect(&psb, &off()).unwrap_err().code, "unsupported_psb");
    assert_eq!(
        inspect(&psd, &AtomicBool::new(true)).unwrap_err().code,
        "import_cancelled"
    );
    let temp = tempfile::tempdir().unwrap();
    let store = ProjectStore::new(temp.path().join("projects")).unwrap();
    let p = store
        .create("Safe".into(), "2d".into(), "vtube-studio".into())
        .unwrap();
    let path = temp.path().join("bad.png");
    fs::write(&path, b"fake").unwrap();
    assert!(store.import_image(&p.id, 1, &path, &off()).is_err());
    assert_eq!(store.open(&p.id).unwrap().revision, 1);
}
#[test]
fn independently_encoded_groups_and_codecs_match_reference_pixels() {
    let group = inspect(
        include_bytes!("../../tests/fixtures/source/groups.psd"),
        &off(),
    )
    .unwrap();
    assert_eq!(group.layers.len(), 4);
    assert_eq!(group.layers[0].group, 3);
    assert_eq!(group.layers[2].group, 1);
    assert!(group.layers[1].mask);
    assert!(!group.layers[3].visible);
    assert_eq!(
        decoded(&group.preview_png),
        decoded(include_bytes!(
            "../../tests/fixtures/source/groups-merged.png"
        ))
    );
    for bytes in [
        include_bytes!("../../tests/fixtures/source/rgb8-codec1.psd").as_slice(),
        include_bytes!("../../tests/fixtures/source/rgb8-codec2.psd").as_slice(),
        include_bytes!("../../tests/fixtures/source/rgb8-codec3.psd").as_slice(),
        include_bytes!("../../tests/fixtures/source/rgb16-codec1.psd").as_slice(),
        include_bytes!("../../tests/fixtures/source/rgb16-codec2.psd").as_slice(),
        include_bytes!("../../tests/fixtures/source/rgb16-codec3.psd").as_slice(),
    ] {
        let report = inspect(bytes, &off()).unwrap();
        assert_eq!(
            decoded(&report.preview_png),
            [255, 0, 128, 255, 255, 0, 128, 255]
        );
    }
}
#[test]
fn sixteen_bit_lr16_extension_retains_layer_references() {
    let original = psd_fixture(16, true);
    let block_len = u32::from_be_bytes(original[34..38].try_into().unwrap()) as usize;
    let info_len = u32::from_be_bytes(original[38..42].try_into().unwrap()) as usize;
    let info = original[42..42 + info_len].to_vec();
    let mut global = [
        u32v(0),
        u32v(0),
        b"8BIMLr16".to_vec(),
        section(info.clone()),
    ]
    .concat();
    global.extend(vec![0; (4 - info.len() % 4) % 4]);
    let file = [
        original[..34].to_vec(),
        section(global),
        original[38 + block_len..].to_vec(),
    ]
    .concat();
    let report = inspect(&file, &off()).unwrap();
    assert_eq!(report.layers.len(), 2);
    assert_eq!(report.layers[0].name, "目・눈");
    assert_eq!(report.bit_depth, 16);
}
#[test]
fn psd_merged_white_matte_does_not_leave_light_or_dark_halos() {
    let mut bytes = psd_fixture(8, true);
    bytes[12..14].copy_from_slice(&4u16.to_be_bytes());
    bytes[42..44].copy_from_slice(&(-2i16).to_be_bytes());
    let n = bytes.len();
    bytes[n - 6..].copy_from_slice(&[255, 255, 127, 127, 127, 127]);
    bytes.extend([128, 128]);
    let report = inspect(&bytes, &off()).unwrap();
    assert_eq!(
        decoded(&report.preview_png),
        [255, 0, 0, 128, 255, 0, 0, 128]
    );
    // Straight red over black/white has no additional white/black edge component.
    for bg in [0u16, 255] {
        let p = decoded(&report.preview_png);
        let alpha = u16::from(p[3]);
        let green = (u16::from(p[1]) * alpha + bg * (255 - alpha) + 127) / 255;
        assert_eq!(green, (bg * 127 + 127) / 255);
    }
}
#[test]
fn psd_icc_and_structural_mutations_are_bounded() {
    let psd = psd_fixture(16, true);
    let profile = lcms2::Profile::new_srgb().icc().unwrap();
    let mut resource = [
        b"8BIM".to_vec(),
        u16v(1039),
        vec![0, 0],
        section(profile.clone()),
    ]
    .concat();
    if !profile.len().is_multiple_of(2) {
        resource.push(0);
    }
    let with_icc = [psd[..30].to_vec(), section(resource), psd[34..].to_vec()].concat();
    let report = inspect(&with_icc, &off()).unwrap();
    assert_eq!(report.icc_hash, Some(hash(&profile)));
    assert_eq!(report.color_status, "icc_to_srgb");
    for position in 0..psd.len() {
        let mut mutated = psd.clone();
        mutated[position] ^= 255;
        assert!(
            std::panic::catch_unwind(|| inspect(&mutated, &off())).is_ok(),
            "panic at offset {position}"
        );
    }
}
#[test]
fn unsupported_blend_is_preserved_without_fake_normal_recomposition() {
    let mut bytes = psd_fixture(8, true);
    let at = bytes.windows(8).position(|w| w == b"8BIMnorm").unwrap();
    bytes[at + 4..at + 8].copy_from_slice(b"mul ");
    let report = inspect(&bytes, &off()).unwrap();
    assert_eq!(report.layers[0].blend, "mul ");
    assert!(report
        .warnings
        .contains(&"psd_layer_render_unverified".into()));
    assert_eq!(report.preview_origin, "psd_embedded_merged_image");
}
#[test]
fn cancel_and_stale_input_never_publish_a_revision() {
    let temp = tempfile::tempdir().unwrap();
    let store = ProjectStore::new(temp.path().join("projects")).unwrap();
    let p = store
        .create("Cancel".into(), "3d".into(), "vbridger-vmc".into())
        .unwrap();
    let bytes = psd_fixture(16, true);
    let file = temp.path().join("日本・한국.psd");
    fs::write(&file, &bytes).unwrap();
    assert_eq!(
        store
            .import_image(&p.id, 1, &file, &AtomicBool::new(true))
            .unwrap_err()
            .code,
        "import_cancelled"
    );
    assert_eq!(store.open(&p.id).unwrap().revision, 1);
    store.rename(&p.id, 1, "Updated".into()).unwrap();
    assert_eq!(
        store
            .import_image(&p.id, 1, &file, &off())
            .unwrap_err()
            .code,
        "revision_conflict"
    );
    assert_eq!(store.open(&p.id).unwrap().revision, 2);
    assert_eq!(fs::read(&file).unwrap(), bytes);
    let blob = temp
        .path()
        .join("projects")
        .join(&p.id)
        .join("sources")
        .join(format!("{}.psd", hash(&bytes)));
    fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&blob)
        .unwrap()
        .set_len(65 * 1024 * 1024)
        .unwrap();
    assert_eq!(
        store
            .import_image(&p.id, 2, &file, &off())
            .unwrap_err()
            .code,
        "source_changed"
    );
    assert_eq!(store.open(&p.id).unwrap().revision, 2);
}
#[test]
fn cancellation_during_high_resolution_decode_releases_the_operation() {
    use std::sync::{atomic::Ordering, Arc, Barrier};
    let mut bytes = vec![];
    {
        let mut encoder = png::Encoder::new(&mut bytes, 4000, 4000);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder
            .write_header()
            .unwrap()
            .write_image_data(&vec![128; 4000 * 4000 * 3])
            .unwrap();
    }
    let cancel = Arc::new(off());
    let barrier = Arc::new(Barrier::new(2));
    let flag = cancel.clone();
    let ready = barrier.clone();
    let worker = std::thread::spawn(move || {
        ready.wait();
        inspect(&bytes, &flag)
    });
    barrier.wait();
    std::thread::sleep(std::time::Duration::from_millis(5));
    cancel.store(true, Ordering::SeqCst);
    assert_eq!(worker.join().unwrap().unwrap_err().code, "import_cancelled");
    // A subsequent independent operation still succeeds; no global poisoned state.
    assert!(inspect(&psd_fixture(8, false), &off()).is_ok());
}
#[test]
fn png_icc_import_preserves_v1_and_original_sixteen_bit_bytes() {
    let temp = tempfile::tempdir().unwrap();
    let store = ProjectStore::new(temp.path().join("projects")).unwrap();
    let p = store
        .create("PNG16".into(), "2d".into(), "vtube-studio".into())
        .unwrap();
    let bytes = png_fixture(
        png::ColorType::Rgb,
        png::BitDepth::Sixteen,
        &[0, 1, 128, 1, 255, 255, 255, 254, 0, 1, 0, 0],
        true,
    );
    let path = temp.path().join("原画.png");
    fs::write(&path, &bytes).unwrap();
    let saved = store.import_image(&p.id, 1, &path, &off()).unwrap();
    assert_eq!(saved.schema_version, 1);
    let blob = temp
        .path()
        .join("projects")
        .join(&p.id)
        .join("sources")
        .join(format!("{}.png", hash(&bytes)));
    assert_eq!(fs::read(blob).unwrap(), bytes);
    assert_eq!(store.inspect_source(&p.id, &off()).unwrap().bit_depth, 16);
    assert_eq!(
        store
            .rename(&p.id, 2, "Renamed".into())
            .unwrap()
            .schema_version,
        1
    );
}
#[test]
fn display_p3_matrix_profile_is_converted_in_the_derivative_only() {
    use lcms2::{CIExyY, CIExyYTRIPLE, Profile, ToneCurve};
    let xy = |x, y| CIExyY { x, y, Y: 1.0 };
    let curve =
        ToneCurve::new_parametric(4, &[2.4, 1.0 / 1.055, 0.055 / 1.055, 1.0 / 12.92, 0.04045])
            .unwrap();
    let profile = Profile::new_rgb(
        &xy(0.3127, 0.3290),
        &CIExyYTRIPLE {
            Red: xy(0.68, 0.32),
            Green: xy(0.265, 0.69),
            Blue: xy(0.15, 0.06),
        },
        &[&curve, &curve, &curve],
    )
    .unwrap()
    .icc()
    .unwrap();
    let mut bytes = vec![];
    let mut info = png::Info::with_size(1, 1);
    info.color_type = png::ColorType::Rgb;
    info.icc_profile = Some(profile.into());
    png::Encoder::with_info(&mut bytes, info)
        .unwrap()
        .write_header()
        .unwrap()
        .write_image_data(&[255, 128, 0])
        .unwrap();
    let source_hash = hash(&bytes);
    let report = inspect(&bytes, &off()).unwrap();
    let pixel = decoded(&report.preview_png);
    assert_eq!(report.color_status, "icc_to_srgb");
    assert_eq!(report.source_hash, source_hash);
    // Independent D65 P3 -> XYZ -> sRGB matrix calculation gives clipped [255,119,0].
    for (actual, expected) in pixel[..3].iter().zip([255u8, 119, 0]) {
        assert!(actual.abs_diff(expected) <= 2, "{pixel:?}");
    }
    assert_ne!(pixel[..3], [255, 128, 0]);
    assert_eq!(hash(&bytes), source_hash);
}
