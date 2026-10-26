use std::path::PathBuf;
use vtuber_core::storage::ProjectStore;
fn main() {
    let root = std::env::var_os("VTUBER_DATA_DIR").expect("Explicit test data root required");
    let store = ProjectStore::new(PathBuf::from(root).join("projects")).unwrap();
    let p = store
        .create(
            "DEV-00 synthetic native test".into(),
            "2d".into(),
            "vbridger-vts".into(),
        )
        .unwrap();
    let p = store
        .import_png(
            &p.id,
            p.revision,
            &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../samples/diagnostic.png"),
        )
        .unwrap();
    println!("{}", serde_json::to_string(&p).unwrap());
}
