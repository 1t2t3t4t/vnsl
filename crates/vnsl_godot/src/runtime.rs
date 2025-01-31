use godot::{
    builtin::{Array, GString, StringName},
    classes::{file_access::ModeFlags, FileAccess, INode, Node},
    global::print,
    meta::ToGodot,
    obj::{Base, Gd, NewGd},
    prelude::{godot_api, GodotClass},
};
use thiserror::Error;
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
}
