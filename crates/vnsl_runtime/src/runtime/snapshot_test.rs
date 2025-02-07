use std::{
    fs::{self},
    path::Path,
};

use vnsl_core::model::VnslScene;

use crate::RuntimeCommand;

use super::Runtime;

const FORCE_RECORD: bool = false;
const SNAPSHOT_BASE_DIR: &str = "./snapshot";

#[derive(Debug, Default)]
struct SceneSnapshotRunner {
    force_choice_pick: Vec<usize>,
}

impl SceneSnapshotRunner {
    fn run_scene(&mut self, scene: VnslScene, record: bool) {
        let mut runtime = Runtime::new();
        let name = scene.name.clone();
        let mut result = String::new();
        runtime.load_scene(scene);
        println!("Testing {name}");

        while !runtime.scene_ended() {
            let cmd = runtime.step().unwrap();
            match cmd {
                RuntimeCommand::ExecuteAction(action)
                    if action.name == "forceChoice".to_string() =>
                {
                    let choices = action
                        .args
                        .into_iter()
                        .map(|a| a.data_type.get_number() as usize);
                    self.force_choice_pick = choices.collect();
                }
                RuntimeCommand::PromptChoices(vnsl_choices) => {
                    assert!(
                        self.force_choice_pick.len() > 0,
                        "No choice selection provided"
                    );
                    let selection = self.force_choice_pick.remove(0);
                    let choice = vnsl_choices.choices.get(selection).unwrap();
                    result.push_str(&format!("Select choice {}\n", choice.text));
                    runtime.select_choice(choice);
                }
                _ => result.push_str(&format!("{:#?}\n", cmd)),
            }
        }

        result = result.replace("\r\n", "\n");
        let base_snapshot = Path::new(SNAPSHOT_BASE_DIR);

        let snapshot_path = base_snapshot.join("records").join(name.clone());
        let existing_result = fs::read_to_string(&snapshot_path);
        if existing_result.is_ok() && !record {
            pretty_assertions::assert_str_eq!(existing_result.unwrap(), result);
        } else {
            fs::write(&snapshot_path, &result).expect("should write result to snapshot file");
        }
    }
}

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
            let mut snapshot_runner = SceneSnapshotRunner::default();
            snapshot_runner.run_scene(scene, FORCE_RECORD);
        }
    }

    assert!(!FORCE_RECORD, "Forcing record is enabled");
}
