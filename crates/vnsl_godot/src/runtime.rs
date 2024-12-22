use godot::{
    classes::{INode, Node},
    meta::ToGodot,
    obj::{Base, Gd},
    prelude::{godot_api, GodotClass},
};
use vnsl_runtime::Runtime;

use crate::result::{wrap_gd_result, GdResult};

#[derive(GodotClass)]
#[class(base=Node)]
struct VnslRuntime {
    runtime: Runtime,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for VnslRuntime {
    fn init(base: Base<Node>) -> Self {
        Self {
            runtime: Runtime::new(),
            base,
        }
    }

    fn ready(&mut self) {
        godot::global::print(&["I'm born from Rust".to_variant()]);
    }

    fn process(&mut self, _delta: f64) {}
}

#[godot_api]
impl VnslRuntime {
    #[func]
    fn load_scene(&mut self, src: String) -> Gd<GdResult> {
        wrap_gd_result(move || {
            let scene = vnsl_compiler::compile(&src)?;
            self.runtime.load_scene(scene);
            Ok(())
        })
    }
}
