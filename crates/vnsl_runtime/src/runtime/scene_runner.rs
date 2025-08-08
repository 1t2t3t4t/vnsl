use vnsl_core::model::VnslScene;

use crate::RuntimeCommand;

use super::Runtime;

#[derive(Debug, Default)]
pub struct SceneRunner {
    force_choice_pick: Vec<usize>,
}

impl SceneRunner {
    pub fn run_scene(&mut self, scene: VnslScene) -> String {
        let mut runtime = Runtime::default();
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
                    result.push_str("Prompt choice\n");
                    for choice in &vnsl_choices {
                        result.push_str(&format!("\tChoice {}\n", choice.text));
                    }
                    #[cfg(test)]
                    assert!(
                        self.force_choice_pick.len() > 0,
                        "No choice selection provided"
                    );

                    let selection = if self.force_choice_pick.len() > 0 {
                        self.force_choice_pick.remove(0)
                    } else {
                        rand::random_range(0..vnsl_choices.len())
                    };

                    let choice = vnsl_choices.get(selection).unwrap();
                    result.push_str(&format!("Select choice {}\n", choice.text));
                    runtime.select_choice(choice);
                }
                RuntimeCommand::Batch(batch) => {
                    result.push_str("==Start Batch==\n");
                    for cmd in batch {
                        result.push_str(&format!("{:#?}\n", cmd));
                    }
                    result.push_str("==End Batch==\n");
                }
                _ => result.push_str(&format!("{:#?}\n", cmd)),
            }
        }

        result
    }
}
