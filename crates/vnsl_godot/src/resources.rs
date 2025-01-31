use godot::{
    builtin::{Dictionary, GString, StringName},
    obj::Gd,
    prelude::{godot_api, GodotClass},
};

#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct VnslSceneMap {
    #[export]
    map: Dictionary,
}

#[godot_api]
impl VnslSceneMap {
    #[func]
    pub fn get_scene(&self, scene_id: StringName) -> Option<Gd<VnslScript>> {
        self.map.get(scene_id).map(|v| v.to())
    }

    #[func]
    pub fn set_scene(&mut self, scene_id: StringName, scene: Gd<VnslScript>) -> bool {
        self.map.insert(scene_id, scene).is_none()
    }
}

#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct VnslScript {
    #[export]
    pub content: GString,
}
