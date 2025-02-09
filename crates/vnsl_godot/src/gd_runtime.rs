use crate::{
    action_handler::{ActionHandlerStore, VnslActionHandler},
    model::{VnslRuntimeAction, VnslRuntimeActionArg, VnslRuntimeChoice},
    service_store::ServiceStore,
    ToGodotVariant,
};
use godot::{
    builtin::{Array, GString, StringName, VariantType},
    classes::{file_access::ModeFlags, FileAccess, INode, Node},
    global::{godot_warn, print},
    meta::ToGodot,
    obj::{Base, Gd, NewGd, WithBaseField},
    prelude::{godot_api, GodotClass},
};
use thiserror::Error;
use vnsl_core::model::{VnslAction, VnslActionArg, VnslChoice};
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
pub struct BaseVnslRuntime {
    runtime: Runtime,
    #[var]
    scene_map: Gd<VnslSceneMap>,
    #[var]
    service_store: Gd<ServiceStore>,

    action_handler: ActionHandlerStore,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for BaseVnslRuntime {
    fn init(base: Base<Node>) -> Self {
        Self {
            runtime: Runtime::new(),
            scene_map: VnslSceneMap::new_gd(),
            service_store: ServiceStore::new_gd(),
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
    fn get_current_character_id(&self) -> GString {
        self.runtime
            .current_character_id()
            .cloned()
            .unwrap_or_default()
            .to_godot()
    }

    #[func]
    fn register_action_handler(&mut self, handler: Gd<VnslActionHandler>) {
        let result = self.action_handler.register_action_handler(handler);
        if let Some(existing_handler) = result {
            let name = existing_handler.bind().handle_action_name();
            godot_warn!("The action with name {} is already register", name);
        }
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
    fn set_global_val_string(&mut self, name: String, val: String) -> Gd<GdResult> {
        wrap_gd_result(|| {
            self.runtime.set_global_val(&name, val.into())?;
            Ok(())
        })
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
        wrap_gd_result(|| self._step())
    }
}

impl BaseVnslRuntime {
    fn handle_action(&self, action: VnslAction) -> bool {
        let Some(mut handler) = self.action_handler.get(&action.name) else {
            godot_warn!("Action {} has no handle", action.name);
            return true;
        };

        let mut bind = handler.bind_mut();
        let res = bind.handle(map_action(&action), self.service_store.clone());
        if res.get_type() == VariantType::BOOL {
            res.to()
        } else {
            false
        }
    }

    fn _step(&mut self) -> anyhow::Result<bool> {
        match self.runtime.step()? {
            RuntimeCommand::SetCharacterId(char_id) => {
                self.base_mut()
                    .emit_signal("set_character_id", &[char_id.to_variant()]);
                return self._step();
            }
            RuntimeCommand::ShowText(text) => {
                self.base_mut()
                    .emit_signal("show_text", &[text.to_variant()]);
            }
            RuntimeCommand::ExecuteAction(action) => {
                let should_step_next = self.handle_action(action);
                if should_step_next {
                    return self._step();
                }
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
    action.bind_mut().args = a.args.iter().map(map_action_arg).collect();

    action
}

fn map_action_arg(arg: &VnslActionArg) -> Gd<VnslRuntimeActionArg> {
    let mut gd_arg = VnslRuntimeActionArg::new_gd();
    gd_arg.bind_mut().name = arg.name.clone().unwrap_or_default().to_godot();
    gd_arg.bind_mut().data = arg.data_type.to_gd_variant();
    gd_arg
}
