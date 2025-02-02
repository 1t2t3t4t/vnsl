use godot::{
    builtin::{Array, GString, Variant},
    obj::Gd,
    prelude::GodotClass,
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

#[derive(Debug, Clone, GodotClass)]
#[class(base = RefCounted, init)]
pub struct VnslRuntimeActionArg {
    #[var]
    pub name: GString,
    #[var]
    pub data_type: Variant,
}
