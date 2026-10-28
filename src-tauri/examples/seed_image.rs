//! Developer-only synthetic input fixture; never exposed as a desktop IPC command.
use std::{path::PathBuf, sync::atomic::AtomicBool};
fn main() {
    let root = PathBuf::from(std::env::var_os("VTUBER_DATA_DIR").expect("explicit fixture root"));
    let input = PathBuf::from(std::env::args_os().nth(1).expect("synthetic input path"));
    let store = vtuber_core::storage::ProjectStore::new(root.join("projects")).unwrap();
    let p = store
        .create(
            "DEV-02 high-resolution fixture".into(),
            "2d".into(),
            "vtube-studio".into(),
        )
        .unwrap();
    let p = store
        .import_image(&p.id, 1, &input, &AtomicBool::new(false))
        .unwrap();
    println!("{}", serde_json::to_string(&p).unwrap());
}
