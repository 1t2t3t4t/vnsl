use std::collections::HashMap;

use crate::{
    action::VnslActionHandler,
    model::{VnslRuntimeAction, VnslRuntimeChoice},
};
use godot::{
    builtin::{Array, GString, StringName},
    classes::{file_access::ModeFlags, FileAccess, INode, Node},
    global::{godot_print, print},
    meta::ToGodot,
    obj::{Base, Gd, NewGd, WithBaseField},
    prelude::{godot_api, GodotClass},
};
use thiserror::Error;
use vnsl_core::model::{VnslAction, VnslChoice};
use vnsl_runtime::{Runtime, RuntimeCommand};

use crate::{
    gd_result::{wrap_gd_result, GdResult},
    resources::{VnslSceneMap, VnslScript},
};

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Scene with name {0} doesn't exist")]
    SceneNotExist(String),
}

#[derive(GodotClass)]
#[class(base=Node)]
struct BaseVnslRuntime {
    runtime: Runtime,
    #[var]
    scene_map: Gd<VnslSceneMap>,

    action_handler: HashMap<String, Vec<Gd<VnslActionHandler>>>,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for BaseVnslRuntime {
    fn init(base: Base<Node>) -> Self {
        Self {
            runtime: Runtime::new(),
            scene_map: VnslSceneMap::new_gd(),
            action_handler: Default::default(),
            base,
        }
    }
}

#[godot_api]
impl BaseVnslRuntime {
    #[signal]
    fn set_character_id(id: String) {}

    #[signal]
    fn show_text(text: String) {}

    #[signal]
    fn change_scene(scene_name: String) {}

    #[signal]
    fn prompt_choices(choices: Array<Gd<VnslRuntimeChoice>>) {}

    #[signal]
    fn scene_end() {}

    #[func]
    fn register_action_handler(&mut self, handler: Gd<VnslActionHandler>) {
        let name = handler.bind().handle_action_name().to_string();
        if !self.action_handler.contains_key(&name) {
            self.action_handler.insert(name.clone(), vec![]);
        }
        self.action_handler.get_mut(&name).unwrap().push(handler);
    }

    #[func]
    fn construct_scene_map(&mut self, scripts_path: Array<GString>) {
        for path in scripts_path.iter_shared() {
            let res = FileAccess::open(&path, ModeFlags::READ);
            let content = res.unwrap().get_as_text().to_string();
            match vnsl_compiler::compile(&content) {
                Ok(scene) => {
                    let path = path.to_string();
                    self.scene_map
                        .bind_mut()
                        .set_scene_lazy(scene.name.into(), move || {
                            let res = FileAccess::open(&path, ModeFlags::READ);
                            let content = res.unwrap().get_as_text().to_string();
                            let mut script = VnslScript::new_gd();
                            script.bind_mut().set_content(content.to_godot());
                            script
                        });
                }
                Err(err) => {
                    print(&[format!("Compile error: {}", err).to_variant()]);
                }
            }
        }
    }

    #[func]
    fn load_scene(&mut self, scene_id: StringName) -> Gd<GdResult> {
        wrap_gd_result(move || {
            let script = self
                .scene_map
                .bind()
                .get_scene(scene_id.clone())
                .ok_or(RuntimeError::SceneNotExist(scene_id.to_string()))?;
            let script = script.bind().content.to_string();
            let scene = vnsl_compiler::compile(&script)?;
            self.runtime.load_scene(scene);
            Ok(())
        })
    }

    #[func]
    fn scene_ended(&self) -> bool {
        self.runtime.scene_ended()
    }

    #[func]
    fn step(&mut self) -> Gd<GdResult> {
        wrap_gd_result(|| {
            match self.runtime.step()? {
                RuntimeCommand::SetCharacterId(char_id) => {
                    self.base_mut()
                        .emit_signal("set_character_id", &[char_id.to_variant()]);
                }
                RuntimeCommand::ShowText(text) => {
                    self.base_mut()
                        .emit_signal("show_text", &[text.to_variant()]);
                }
                RuntimeCommand::ExecuteAction(action) => {
                    self.handle_action(action);
                }
                RuntimeCommand::PromptChoices(vnsl_choices) => {
                    let choices = vnsl_choices
                        .choices
                        .into_iter()
                        .map(map_choice)
                        .collect::<Vec<_>>();
                    self.base_mut()
                        .emit_signal("prompt_choices", &[choices.to_variant()]);
                }
                RuntimeCommand::ChangeScene(name) => {
                    self.base_mut()
                        .emit_signal("change_scene", &[name.to_variant()]);
                }
                RuntimeCommand::EndOfScene => return Ok(false),
            }
            Ok(true)
        })
    }
}

impl BaseVnslRuntime {
    fn handle_action(&mut self, action: VnslAction) {
        let Some(handlers) = self.action_handler.get_mut(&action.name) else {
            godot_print!("Action {} has no handle", action.name);
            return;
        };
        if handlers.len() == 0 {
            godot_print!("Action {} has no handle", action.name);
        }

        for handler in handlers {
            handler.bind_mut().handle(map_action(&action));
        }
    }
}

fn map_choice(c: VnslChoice) -> Gd<VnslRuntimeChoice> {
    let mut choice = VnslRuntimeChoice::new_gd();
    choice.bind_mut().id = c.id.to_godot();
    choice.bind_mut().text = c.text.to_godot();
    choice
}

fn map_action(a: &VnslAction) -> Gd<VnslRuntimeAction> {
    let mut action = VnslRuntimeAction::new_gd();
    action.bind_mut().name = a.name.to_string().into();

    action
}
