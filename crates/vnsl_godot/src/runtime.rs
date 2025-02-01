mod model {
    use godot::{builtin::GString, prelude::GodotClass};

    #[derive(Debug, Clone, PartialEq, Eq, GodotClass)]
    #[class(base = RefCounted, init)]
    pub struct VnslRuntimeChoice {
        #[var]
        pub id: GString,
        #[var]
        pub text: GString,
    }
}

use godot::{
    builtin::{Array, GString, StringName},
    classes::{file_access::ModeFlags, FileAccess, INode, Node},
    global::print,
    meta::ToGodot,
    obj::{Base, Gd, NewGd, WithBaseField},
    prelude::{godot_api, GodotClass},
};
use model::VnslRuntimeChoice;
use thiserror::Error;
use vnsl_core::model::VnslChoice;
use vnsl_runtime::Runtime;

use crate::{
    resources::{VnslSceneMap, VnslScript},
    result::{wrap_gd_result, GdResult},
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

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for BaseVnslRuntime {
    fn init(base: Base<Node>) -> Self {
        Self {
            runtime: Runtime::new(),
            scene_map: VnslSceneMap::new_gd(),
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
    fn construct_scene_map(&mut self, scripts_path: Array<GString>) {
        for path in scripts_path.iter_shared() {
            let res = FileAccess::open(&path, ModeFlags::READ);
            let content = res.unwrap().get_as_text();
            match vnsl_compiler::compile(&content.to_string()) {
                Ok(scene) => {
                    let mut script = VnslScript::new_gd();
                    script.bind_mut().set_content(content);

                    self.scene_map
                        .bind_mut()
                        .set_scene(scene.name.into(), script);
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
    fn step(&mut self) -> Gd<GdResult> {
        wrap_gd_result(|| {
            match self.runtime.step()? {
                vnsl_runtime::RuntimeCommand::SetCharacterId(char_id) => {
                    self.base_mut()
                        .emit_signal("set_character_id", &[char_id.to_variant()]);
                }
                vnsl_runtime::RuntimeCommand::ShowText(text) => {
                    self.base_mut()
                        .emit_signal("show_text", &[text.to_variant()]);
                }
                vnsl_runtime::RuntimeCommand::ExecuteAction(_) => todo!(),
                vnsl_runtime::RuntimeCommand::PromptChoices(vnsl_choices) => {
                    let choices = vnsl_choices
                        .choices
                        .into_iter()
                        .map(map_choice)
                        .collect::<Vec<_>>();
                    self.base_mut()
                        .emit_signal("prompt_choices", &[choices.to_variant()]);
                }
                vnsl_runtime::RuntimeCommand::ChangeScene(name) => {
                    self.base_mut()
                        .emit_signal("change_scene", &[name.to_variant()]);
                }
                vnsl_runtime::RuntimeCommand::EndOfScene => {
                    self.base_mut().emit_signal("scene_end", &[]);
                }
            }
            Ok(())
        })
    }
}

fn map_choice(c: VnslChoice) -> Gd<VnslRuntimeChoice> {
    let mut choice = VnslRuntimeChoice::new_gd();
    choice.bind_mut().id = c.id.to_godot();
    choice.bind_mut().text = c.text.to_godot();
    choice
}
