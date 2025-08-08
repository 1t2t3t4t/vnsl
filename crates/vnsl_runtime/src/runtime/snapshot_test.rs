use std::{fs, path::Path};

use crate::runtime::scene_runner::SceneRunner;

const FORCE_RECORD: bool = false;
const SNAPSHOT_BASE_DIR: &str = "./snapshot";

#[test]
fn test_snapshots() {
    fs::create_dir_all(SNAPSHOT_BASE_DIR).expect("should create base snapshot dir");
    let dir = fs::read_dir(SNAPSHOT_BASE_DIR).expect("should be able to read snapshot base dir");

    for entry in dir {
        let entry = entry.expect("should be able to read dir entry");
        let is_file = entry.file_type().map(|i| i.is_file()).unwrap_or(false);
        let file_name = entry.file_name().to_str().unwrap().to_string();

        if is_file && file_name.ends_with(".vnsl") {
            let src = fs::read_to_string(entry.path()).unwrap();
            let scene = vnsl_compiler::compile(&src).unwrap();
            let scene_name = scene.name.clone();

            let mut snapshot_runner = SceneRunner::default();
            let mut result = snapshot_runner.run_scene(scene);

            result = result.replace("\r\n", "\n");
            let base_snapshot = Path::new(SNAPSHOT_BASE_DIR);
            if !base_snapshot.join("records").exists() {
                fs::create_dir_all(&base_snapshot.join("records"))
                    .expect("should create snapshot dir");
            }

            let snapshot_path = base_snapshot.join("records").join(scene_name);
            let existing_result = fs::read_to_string(&snapshot_path);
            if existing_result.is_ok() && !FORCE_RECORD {
                pretty_assertions::assert_str_eq!(existing_result.unwrap(), result);
            } else {
                fs::write(&snapshot_path, &result).expect("should write result to snapshot file");
            }
        }
    }

    assert!(!FORCE_RECORD, "Forcing record is enabled");
}
