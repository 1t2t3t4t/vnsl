use godot::prelude::*;

mod action;
mod model;
mod resources;
mod result;
mod runtime;

struct Extension;

#[gdextension]
unsafe impl ExtensionLibrary for Extension {}
