use godot::{
    builtin::{Array, GString, Variant},
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
