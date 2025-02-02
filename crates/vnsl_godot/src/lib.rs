use godot::prelude::*;

mod action;
mod gd_result;
mod gd_runtime;
mod model;
mod resources;

struct Extension;

#[gdextension]
unsafe impl ExtensionLibrary for Extension {}
