use std::collections::BTreeMap;

use vnsl_core::model::{
    VnslAction, VnslActionArg, VnslBlock, VnslChoice, VnslChoices, VnslCommand, VnslDataType,
    VnslDialogue, VnslGlobal, VnslGoTo, VnslJump, VnslLabel, VnslScene, VnslSetCharacter,
    VnslStatement,
};

use crate::result::RuntimeError;

use super::{Runtime, RuntimeCommand};

fn create_test_scene(name: &str) -> VnslScene {
    VnslScene {
        name: name.to_string(),
        main_block: VnslBlock {
            statements: vec![VnslStatement::Command(VnslCommand::Dialogue(
                VnslDialogue {
                    text: "Hello World".to_string(),
                },
            ))],
        },
        #[cfg(debug_assertions)]
        labels: BTreeMap::new(),
    }
}

fn create_test_scene_with_labels(name: &str) -> VnslScene {
    let mut labels = {
        #[cfg(debug_assertions)]
        {
            BTreeMap::new()
        }
    };

    labels.insert(
        "test_label".to_string(),
        VnslLabel {
            name: "test_label".to_string(),
            block: VnslBlock {
                statements: vec![VnslStatement::Command(VnslCommand::Dialogue(
                    VnslDialogue {
                        text: "Label reached".to_string(),
                    },
                ))],
            },
        },
    );

    VnslScene {
        name: name.to_string(),
        main_block: VnslBlock {
            statements: vec![VnslStatement::Command(VnslCommand::Jump(VnslJump {
                to_label: "test_label".to_string(),
            }))],
        },
        labels,
    }
}

fn create_test_block(text: &str) -> VnslBlock {
    VnslBlock {
        statements: vec![VnslStatement::Command(VnslCommand::Dialogue(
            VnslDialogue {
                text: text.to_string(),
            },
        ))],
    }
}

#[test]
fn test_runtime_default() {
    let runtime = Runtime::default();
    assert!(runtime.get_current_scene().is_none());
    assert_eq!(runtime.get_run_stack().len(), 0);
    assert!(runtime.scene_ended());
    assert!(runtime.current_character_id().is_none());
}

#[test]
fn test_load_scene() {
    let mut runtime = Runtime::default();
    let scene = create_test_scene("TestScene");

    runtime.load_scene(scene.clone());

    assert!(runtime.get_current_scene().is_some());
    assert_eq!(runtime.get_current_scene().unwrap().name, "TestScene");
    assert_eq!(runtime.get_run_stack().len(), 1);
    assert!(!runtime.scene_ended());
}

#[test]
fn test_scene_ended() {
    let mut runtime = Runtime::default();
    assert!(runtime.scene_ended());

    let scene = create_test_scene("TestScene");
    runtime.load_scene(scene);
    assert!(!runtime.scene_ended());
}

#[test]
fn test_step_no_scene() {
    let mut runtime = Runtime::default();
    let result = runtime.step();
    assert_eq!(result, Ok(RuntimeCommand::EndOfScene));
}

#[test]
fn test_step_with_dialogue() {
    let mut runtime = Runtime::default();
    let scene = create_test_scene("TestScene");
    runtime.load_scene(scene);

    let result = runtime.step().unwrap();
    match result {
        RuntimeCommand::ShowText(text) => assert_eq!(text, "Hello World"),
        _ => panic!("Expected ShowText command"),
    }
}

#[test]
fn test_process_current_command_no_scene() {
    let mut runtime = Runtime::default();
    let result = runtime.process_current_command();
    assert_eq!(result, Ok(RuntimeCommand::EndOfScene));
}

#[test]
fn test_set_character() {
    let mut runtime = Runtime::default();
    let scene = VnslScene {
        name: "TestScene".to_string(),
        main_block: VnslBlock {
            statements: vec![VnslStatement::Command(VnslCommand::SetCharacter(
                VnslSetCharacter {
                    id: "protagonist".to_string(),
                },
            ))],
        },
        #[cfg(debug_assertions)]
        labels: BTreeMap::new(),
    };

    runtime.load_scene(scene);
    let result = runtime.step().unwrap();

    match result {
        RuntimeCommand::SetCharacterId(id) => assert_eq!(id, "protagonist"),
        _ => panic!("Expected SetCharacterId command"),
    }

    assert_eq!(
        runtime.current_character_id(),
        Some(&"protagonist".to_string())
    );
}

#[test]
fn test_global_variable() {
    let mut runtime = Runtime::default();
    let scene = VnslScene {
        name: "TestScene".to_string(),
        main_block: VnslBlock {
            statements: vec![VnslStatement::Command(VnslCommand::Global(VnslGlobal {
                name: "test_var".to_string(),
                value: VnslDataType::String("test_value".to_string()),
            }))],
        },
        #[cfg(debug_assertions)]
        labels: BTreeMap::new(),
    };

    runtime.load_scene(scene);
    let _result = runtime.step().unwrap();

    // The global should be set in the lua runtime
    // We can't directly test this without access to the lua runtime,
    // but we can test the set_global_val method
    let set_result = runtime.set_global_val("another_var", VnslDataType::Number(42.0));
    assert!(set_result.is_ok());
}

#[test]
fn test_action_command() {
    let mut runtime = Runtime::default();
    let action = VnslAction {
        name: "test_action".to_string(),
        args: vec![VnslActionArg {
            name: Some("param1".to_string()),
            data_type: VnslDataType::String("value1".to_string()),
        }],
    };

    let scene = VnslScene {
        name: "TestScene".to_string(),
        main_block: VnslBlock {
            statements: vec![VnslStatement::Command(VnslCommand::Action(action.clone()))],
        },
        #[cfg(debug_assertions)]
        labels: BTreeMap::new(),
    };

    runtime.load_scene(scene);
    let result = runtime.step().unwrap();

    match result {
        RuntimeCommand::ExecuteAction(returned_action) => {
            assert_eq!(returned_action.name, "test_action");
            assert_eq!(returned_action.args.len(), 1);
            assert_eq!(returned_action.args[0].name, Some("param1".to_string()));
        }
        _ => panic!("Expected ExecuteAction command"),
    }
}

#[test]
fn test_choices_command() {
    let mut runtime = Runtime::default();
    let choices = VnslChoices {
        choices: vec![
            VnslChoice {
                id: "choice1".to_string(),
                text: "Option 1".to_string(),
                block: create_test_block("Choice 1 selected"),
            },
            VnslChoice {
                id: "choice2".to_string(),
                text: "Option 2".to_string(),
                block: create_test_block("Choice 2 selected"),
            },
        ],
    };

    let scene = VnslScene {
        name: "TestScene".to_string(),
        main_block: VnslBlock {
            statements: vec![VnslStatement::Choices(choices.clone())],
        },
        #[cfg(debug_assertions)]
        labels: BTreeMap::new(),
    };

    runtime.load_scene(scene);
    let result = runtime.step().unwrap();

    match result {
        RuntimeCommand::PromptChoices(returned_choices) => {
            assert_eq!(returned_choices.choices.len(), 2);
            assert_eq!(returned_choices.choices[0].text, "Option 1");
            assert_eq!(returned_choices.choices[1].text, "Option 2");
        }
        _ => panic!("Expected PromptChoices command"),
    }
}

#[test]
fn test_select_choice() {
    let mut runtime = Runtime::default();
    let choice = VnslChoice {
        id: "test_choice".to_string(),
        text: "Test Choice".to_string(),
        block: create_test_block("Choice executed"),
    };

    let scene = create_test_scene("TestScene");
    runtime.load_scene(scene);

    // Select the choice
    runtime.select_choice(&choice);

    // The choice block should be on the stack
    assert_eq!(runtime.get_run_stack().len(), 2);
}

#[test]
fn test_change_scene() {
    let mut runtime = Runtime::default();
    let scene = VnslScene {
        name: "TestScene".to_string(),
        main_block: VnslBlock {
            statements: vec![VnslStatement::Command(VnslCommand::GoTo(VnslGoTo {
                scene_id: "NextScene".to_string(),
            }))],
        },
        #[cfg(debug_assertions)]
        labels: BTreeMap::new(),
    };

    runtime.load_scene(scene);
    let result = runtime.step().unwrap();

    match result {
        RuntimeCommand::ChangeScene(scene_id) => assert_eq!(scene_id, "NextScene"),
        _ => panic!("Expected ChangeScene command"),
    }

    // Stack should be cleared after scene change
    assert_eq!(runtime.get_run_stack().len(), 0);
}

#[test]
fn test_jump_to_label() {
    let mut runtime = Runtime::default();
    let scene = create_test_scene_with_labels("TestScene");

    runtime.load_scene(scene);
    let result = runtime.step().unwrap();

    // Should jump to label and execute its dialogue
    match result {
        RuntimeCommand::ShowText(text) => assert_eq!(text, "Label reached"),
        _ => panic!("Expected ShowText command from label"),
    }
}

#[test]
fn test_jump_to_invalid_label() {
    let mut runtime = Runtime::default();
    let scene = VnslScene {
        name: "TestScene".to_string(),
        main_block: VnslBlock {
            statements: vec![VnslStatement::Command(VnslCommand::Jump(VnslJump {
                to_label: "nonexistent_label".to_string(),
            }))],
        },
        #[cfg(debug_assertions)]
        labels: BTreeMap::new(),
    };

    runtime.load_scene(scene);
    let result = runtime.step();

    assert!(result.is_err());
    match result.unwrap_err() {
        RuntimeError::InvalidLabelJump(label) => assert_eq!(label, "nonexistent_label"),
        _ => panic!("Expected InvalidLabelJump error"),
    }
}

#[test]
fn test_snapshot_creation() {
    let mut runtime = Runtime::default();
    let scene = create_test_scene("TestScene");
    runtime.load_scene(scene);

    let snapshot = runtime.snapshot();
    assert!(snapshot.current_scene.is_some());
    assert_eq!(snapshot.current_scene.as_ref().unwrap().name, "TestScene");
    assert_eq!(snapshot.run_stack.len(), 1);
}

#[test]
fn test_runtime_from_snapshot() {
    let mut original_runtime = Runtime::default();
    let scene = create_test_scene("TestScene");
    original_runtime.load_scene(scene);

    // Set a global variable
    let _ = original_runtime.set_global_val("test_var", VnslDataType::Number(123.0));

    let snapshot = original_runtime.snapshot();
    let restored_runtime = Runtime::from(snapshot);

    assert!(restored_runtime.get_current_scene().is_some());
    assert_eq!(
        restored_runtime.get_current_scene().unwrap().name,
        "TestScene"
    );
    assert_eq!(restored_runtime.get_run_stack().len(), 1);
}

#[test]
fn test_multiple_steps_until_scene_end() {
    let mut runtime = Runtime::default();
    let scene = VnslScene {
        name: "TestScene".to_string(),
        main_block: VnslBlock {
            statements: vec![
                VnslStatement::Command(VnslCommand::Dialogue(VnslDialogue {
                    text: "First line".to_string(),
                })),
                VnslStatement::Command(VnslCommand::Dialogue(VnslDialogue {
                    text: "Second line".to_string(),
                })),
            ],
        },
        #[cfg(debug_assertions)]
        labels: BTreeMap::new(),
    };

    runtime.load_scene(scene);

    // First step
    let result1 = runtime.step().unwrap();
    match result1 {
        RuntimeCommand::ShowText(text) => assert_eq!(text, "First line"),
        _ => panic!("Expected first ShowText command"),
    }
    assert!(!runtime.scene_ended());

    // Second step
    let result2 = runtime.step().unwrap();
    match result2 {
        RuntimeCommand::ShowText(text) => assert_eq!(text, "Second line"),
        _ => panic!("Expected second ShowText command"),
    }
    assert!(runtime.scene_ended());

    // Third step should end the scene
    let result3 = runtime.step().unwrap();
    match result3 {
        RuntimeCommand::EndOfScene => {}
        _ => panic!("Expected EndOfScene command"),
    }
    assert!(runtime.scene_ended());
}

#[cfg(debug_assertions)]
#[test]
fn test_run_stack_string() {
    let mut runtime = Runtime::default();
    let scene = create_test_scene("TestScene");
    runtime.load_scene(scene);

    let stack_string = runtime.run_stack_string();
    assert!(!stack_string.is_empty());
    assert!(stack_string.contains("BlockRunner"));
}

#[test]
fn test_error_conditions() {
    let mut runtime = Runtime::default();

    // Test with no scene loaded
    let result = runtime.process_current_command();
    assert_eq!(result, Ok(RuntimeCommand::EndOfScene));

    // Test step with no scene
    let result = runtime.step();
    assert_eq!(result, Ok(RuntimeCommand::EndOfScene));
}
