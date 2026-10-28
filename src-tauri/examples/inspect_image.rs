use std::{io::Read, sync::atomic::AtomicBool, time::Instant};
fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("input path");
    let mut bytes = vec![];
    std::fs::File::open(path)
        .unwrap()
        .take((vtuber_core::image_input::MAX_FILE + 1) as u64)
        .read_to_end(&mut bytes)
        .unwrap();
    let start = Instant::now();
    match vtuber_core::image_input::inspect(&bytes, &AtomicBool::new(false)) {
        Ok(mut report) => {
            let elapsed = start.elapsed().as_millis();
            if let Some(preview) = args.next() {
                std::fs::write(preview, &report.preview_png).unwrap();
            }
            report.preview_png.clear();
            let mut value = serde_json::to_value(report).unwrap();
            value["elapsed_ms"] = serde_json::json!(elapsed);
            value["input_bytes"] = bytes.len().into();
            println!("{}", value);
        }
        Err(error) => {
            println!("{}", serde_json::to_string(&error).unwrap());
            std::process::exit(1);
        }
    }
}
