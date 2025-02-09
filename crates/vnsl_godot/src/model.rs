use godot::{
    builtin::{Array, GString, Variant},
    meta::ToGodot,
    obj::Gd,
    prelude::{godot_api, GodotClass},
};

#[derive(Debug, Clone, PartialEq, Eq, GodotClass)]
#[class(base = RefCounted, init)]
pub struct VnslRuntimeChoice {
    #[var]
    pub id: GString,
    #[var]
    pub text: GString,
}

#[derive(Debug, Clone, GodotClass)]
#[class(base = RefCounted, init)]
pub struct VnslRuntimeAction {
    #[var]
    pub name: GString,
    #[var]
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
#[class(base = RefCounted, init)]
pub struct VnslRuntimeActionArg {
    #[var]
    pub name: GString,
    #[var]
    pub data: Variant,
}

#[godot_api]
impl VnslRuntimeActionArg {
    #[func]
    pub fn get_as_string(&self) -> String {
        self.data.to()
    }
}
