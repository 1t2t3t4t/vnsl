use godot::prelude::*;
use vnsl_core::model::VnslDataType;

mod action_handler;
mod gd_result;
mod runtime;
mod model;
mod resources;
mod service_store;

struct Extension;

#[gdextension]
unsafe impl ExtensionLibrary for Extension {}

trait ToGodotVariant {
    fn to_gd_variant(&self) -> Variant;
}

impl ToGodotVariant for VnslDataType {
    fn to_gd_variant(&self) -> Variant {
        match self {
            VnslDataType::String(s) => s.to_variant(),
            VnslDataType::Number(n) => n.to_variant(),
            VnslDataType::Bool(b) => b.to_variant(),
        }
    }
}
