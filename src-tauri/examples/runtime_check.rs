//! Developer reproduction tool; the product uses the same manager through typed IPC.
use std::{path::PathBuf, sync::atomic::AtomicBool};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let root = PathBuf::from(std::env::var_os("VTUBER_RUNTIME_DIR").expect("VTUBER_RUNTIME_DIR"));
    let manager = vtuber_core::runtime::RuntimeManager::new(
        root,
        PathBuf::from(std::env::var_os("VTUBER_PYTHON").expect("VTUBER_PYTHON")),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../workers/runtime.py"),
    )
    .unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    let cancel = AtomicBool::new(false);
    let mut progress = |p| eprintln!("progress {p}");
    let result = match args.get(1).map(String::as_str) {
        Some("install") => {
            manager.install(&id, true, &manager.consent_token(), &cancel, &mut progress)
        }
        Some("probe") => manager.probe(&id, &cancel, &mut progress),
        Some("infer") => manager
            .infer(&id, None, &cancel, &mut progress)
            .and_then(|a| manager.accept(&id, &a)),
        _ => {
            println!("{}", manager.view().unwrap());
            return;
        }
    };
    match result {
        Ok(()) => println!("{}", manager.result(&id).unwrap()),
        Err(error) => {
            eprintln!("{}", error.code);
            std::process::exit(1);
        }
    }
}
