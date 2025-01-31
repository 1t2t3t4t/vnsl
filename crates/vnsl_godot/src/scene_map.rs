use godot::{builtin::Dictionary, prelude::GodotClass};

#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct VnslSceneMap {
    map: Dictionary,
}

impl VnslSceneMap {}
