use std::{cell::LazyCell, collections::HashMap, ops::Deref};

use godot::{
    builtin::{Array, GString, StringName, Variant},
    meta::ToGodot,
    obj::Gd,
    prelude::{godot_api, GodotClass},
};
use vnsl_core::model::VnslBlock;

#[derive(Debug, Clone, PartialEq, GodotClass)]
#[class(base = Resource, init)]
pub struct VnslRuntimeChoice {
    #[export]
    pub id: GString,
    #[export]
    pub text: GString,
    #[export]
    block_str: GString,
    #[export]
    condition: GString,
}

impl VnslRuntimeChoice {
    pub fn get_block(&self) -> VnslBlock {
        serde_json::from_str(&self.block_str.to_string()).unwrap()
    }
}

#[derive(Debug, Clone, GodotClass)]
#[class(base = Resource, init)]
pub struct VnslRuntimeAction {
    #[export]
    pub name: GString,
    #[export]
    pub args: Array<Gd<VnslRuntimeActionArg>>,
}

#[godot_api]
impl VnslRuntimeAction {
    #[func]
    pub fn get_arg(&self, idx: u32) -> Option<Gd<VnslRuntimeActionArg>> {
        let idx = idx as usize;
        if idx >= self.args.len() {
            None
        } else {
            self.args.get(idx).clone()
        }
    }

    #[func]
    pub fn get_arg_with_name(&self, name: String) -> Option<Gd<VnslRuntimeActionArg>> {
        for arg in self.args.iter_shared() {
            if arg.bind().name == name.to_godot() {
                return Some(arg.clone());
            }
        }
        None
    }
}

#[derive(Debug, Clone, GodotClass)]
#[class(base = Resource, init)]
pub struct VnslRuntimeActionArg {
    #[export]
    pub name: GString,
    #[export]
    pub data: Variant,
}

#[godot_api]
impl VnslRuntimeActionArg {
    #[func]
    pub fn get_as_string(&self) -> String {
        self.data.to()
    }
}

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
