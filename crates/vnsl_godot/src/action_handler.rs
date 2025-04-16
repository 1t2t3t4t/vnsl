use std::collections::HashMap;

use godot::{
    builtin::{Array, GString, Variant},
    classes::RefCounted,
    global::godot_warn,
    obj::{Base, Gd, WithBaseField},
    prelude::{godot_api, GodotClass},
};

use crate::{resources::VnslRuntimeAction, service_store::ServiceStore};

#[derive(Debug, Default)]
pub struct ActionHandlerStore {
    handlers: HashMap<String, Gd<VnslActionHandler>>,
}

impl ActionHandlerStore {
    pub fn register_action_handler(&mut self, handler: Gd<VnslActionHandler>) {
        let names = handler.bind().handle_action_name();
        for name in names.iter_shared() {
            if self
                .handlers
                .insert(name.to_string(), handler.clone())
                .is_some()
            {
                godot_warn!("Handler with name {} is already registered", name)
            }
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
    pub fn should_handle_action(&self, _name: String) -> bool {
        unimplemented!("Action handler didn't implement should_handle_action",)
    }

    #[func(virtual)]
    pub fn handle_action_name(&self) -> Array<GString> {
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
