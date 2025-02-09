use std::collections::HashMap;

use godot::{
    classes::Object,
    obj::Gd,
    prelude::{godot_api, GodotClass},
};

#[derive(GodotClass, Debug)]
#[class(base=RefCounted, init)]
pub struct ServiceStore {
    storage: HashMap<String, Gd<Object>>,
}

#[godot_api]
impl ServiceStore {
    #[func]
    pub fn register_service(&mut self, name: String, obj: Gd<Object>) {
        self.storage.insert(name.to_string(), obj);
    }

    #[func]
    pub fn get_service(&self, name: String) -> Option<Gd<Object>> {
        self.storage.get(&name).cloned()
    }
}
