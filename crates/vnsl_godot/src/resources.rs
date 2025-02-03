use std::{cell::LazyCell, collections::HashMap, ops::Deref};

use godot::{
    builtin::{GString, StringName},
    obj::Gd,
    prelude::{godot_api, GodotClass},
};

#[derive(Debug)]
pub struct LazyResource<R: GodotClass> {
    lazy: LazyCell<Gd<R>, Box<dyn FnOnce() -> Gd<R>>>,
}

impl<R: GodotClass> LazyResource<R> {
    pub fn new<F>(f: F) -> Self
    where
        F: 'static + FnOnce() -> Gd<R>,
    {
        Self {
            lazy: LazyCell::new(Box::new(f)),
        }
    }

    pub fn clone_inner(&self) -> Gd<R> {
        self.lazy.clone()
    }
}

impl<R: GodotClass> Deref for LazyResource<R> {
    type Target = Gd<R>;

    fn deref(&self) -> &Self::Target {
        &self.lazy
    }
}

#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct VnslSceneMap {
    map: HashMap<StringName, LazyResource<VnslScript>>,
}

#[godot_api]
impl VnslSceneMap {
    #[func]
    pub fn get_scene(&self, scene_id: StringName) -> Option<Gd<VnslScript>> {
        self.map.get(&scene_id).map(|l| l.clone_inner())
    }

    #[func]
    pub fn set_scene(&mut self, scene_id: StringName, scene: Gd<VnslScript>) -> bool {
        self.map
            .insert(scene_id, LazyResource::new(|| scene))
            .is_none()
    }
}

impl VnslSceneMap {
    pub fn set_scene_lazy<F>(&mut self, scene_id: StringName, f: F) -> bool
    where
        F: 'static + FnOnce() -> Gd<VnslScript>,
    {
        self.map.insert(scene_id, LazyResource::new(f)).is_none()
    }
}

#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct VnslScript {
    #[export]
    pub content: GString,
}
