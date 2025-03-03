use std::collections::HashMap;

use godot::{
    builtin::{GString, Variant},
    classes::RefCounted,
    obj::{Base, Gd, WithBaseField},
    prelude::{godot_api, GodotClass},
};

use crate::{model::VnslRuntimeAction, service_store::ServiceStore};

#[derive(Debug, Default)]
pub struct ActionHandlerStore {
    handlers: HashMap<String, Gd<VnslActionHandler>>,
}

impl ActionHandlerStore {
    pub fn register_action_handler(
        &mut self,
        handler: Gd<VnslActionHandler>,
    ) -> Option<Gd<VnslActionHandler>> {
        let name = handler.bind().handle_action_name().to_string();
        if let Some(handler) = self.handlers.get(&name) {
            Some(handler.clone())
        } else {
            self.handlers.insert(name.clone(), handler);
            None
        }
    }

    pub fn get(&self, name: &str) -> Option<Gd<VnslActionHandler>> {
        self.handlers.get(name).cloned()
    }
}

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
    pub fn handle(
        &mut self,
        action: Gd<VnslRuntimeAction>,
        service_store: Gd<ServiceStore>,
    ) -> Variant {
        let global_name = self.base().get_script().call("get_global_name", &[]);
        unimplemented!("Action handler {} is not handled", global_name)
    }
}
