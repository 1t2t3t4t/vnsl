use std::{fs, path::Path};

use vnsl_core::model::VnslScene;

use crate::RuntimeCommand;

use super::Runtime;

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

        while let Ok(cmd) = runtime.step() {
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
                RuntimeCommand::EndOfScene => {
                    result.push_str(&format!("{:#?}\n", cmd));
                    break;
                }
                _ => result.push_str(&format!("{:#?}\n", cmd)),
            }
        }

        let base_snapshot = Path::new("./snapshot");
        fs::create_dir_all(base_snapshot).expect("should create base snapshot dir");

        let snapshot_path = base_snapshot.join(name);
        let existing_result = fs::read_to_string(&snapshot_path);
        if existing_result.is_ok() && !record {
            pretty_assertions::assert_str_eq!(existing_result.unwrap(), result);
        } else {
            fs::write(&snapshot_path, &result).expect("should write result to snapshot file");
        }
    }
}

const FORCE_RECORD: bool = false;

#[test]
fn test_basic_flow() {
    let src = r#"
scene Basic

character Boss

[forceChoice 0]
[action 123]

"Hello"
"World"

choice:
    "A":
        "A"
"#;
    let scene = vnsl_compiler::compile(&src).unwrap();
    let mut snapshot_runner = SceneSnapshotRunner::default();
    snapshot_runner.run_scene(scene, FORCE_RECORD);
}
