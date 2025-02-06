use godot::{
    builtin::GString,
    classes::RefCounted,
    obj::{Base, Gd, WithBaseField},
    prelude::{godot_api, GodotClass},
};

use crate::model::VnslRuntimeAction;

#[derive(GodotClass)]
#[class(base=RefCounted, init)]
pub struct VnslActionHandler {
    #[base]
    base: Base<RefCounted>,
}

#[godot_api]
impl VnslActionHandler {
    #[func(virtual)]
    pub fn handle_action_name(&self) -> GString {
        unimplemented!("Action handler didn't implement handle_action_name",)
    }

    #[func(virtual)]
    #[allow(unused_variables)]
    pub fn handle(&mut self, action: Gd<VnslRuntimeAction>) {
        let global_name = self.base().get_script().call("get_global_name", &[]);
        unimplemented!("Action handler {} is not handled", global_name)
    }
}
