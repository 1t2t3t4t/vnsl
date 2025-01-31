use std::ops::Deref;

use godot::{
    builtin::{Array, GString},
    classes::{file_access::ModeFlags, FileAccess, INode, Node, Resource},
    global::print,
    meta::ToGodot,
    obj::{Base, Gd, NewGd},
    prelude::{godot_api, GodotClass},
};
use vnsl_runtime::Runtime;

use crate::{
    resources::{VnslSceneMap, VnslScript},
    result::{wrap_gd_result, GdResult},
};

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
            let scene = vnsl_compiler::compile(&content.to_string()).unwrap();
            let mut script = VnslScript::new_gd();
            script.bind_mut().set_content(content);

            self.scene_map
                .bind_mut()
                .set_scene(scene.name.into(), script);
        }
    }

    #[func]
    fn load_scene(&mut self, src: String) -> Gd<GdResult> {
        wrap_gd_result(move || {
            let scene = vnsl_compiler::compile(&src)?;
            self.runtime.load_scene(scene);
            Ok(())
        })
    }
}
