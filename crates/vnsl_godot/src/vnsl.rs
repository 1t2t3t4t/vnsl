use std::path::Path;

use godot::{
    classes::{file_access::ModeFlags, DirAccess, FileAccess, INode, Node},
    meta::ToGodot,
    obj::{Base, Gd},
    prelude::{godot_api, GodotClass},
};

#[derive(GodotClass)]
#[class(base=Node)]
struct Vnsl {
    base: Base<Node>,
}

#[godot_api]
impl INode for Vnsl {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        godot::global::print(&["I'm born from Rust".to_variant()]);
    }

    fn process(&mut self, _delta: f64) {}
}

#[godot_api]
impl Vnsl {
    #[func]
    fn compile(&self, src: String, output_path: String) {
        let result = vnsl_compiler::compile(&src).unwrap();
        let result_str = serde_json::to_string_pretty(&result).unwrap();
        let err = DirAccess::make_dir_recursive_absolute(&output_path);
        if err != godot::global::Error::OK {
            panic!("Error")
        }
        let out_file = Path::new(&output_path).join(format!("{}.txt", result.name));
        let out_file = out_file.to_str().unwrap();
        let mut file = FileAccess::open(out_file, ModeFlags::WRITE).unwrap();
        file.store_string(&result_str);
        file.flush();
    }

    #[func]
    fn take_control(&self, vnsl: Gd<Vnsl>) {}
}
