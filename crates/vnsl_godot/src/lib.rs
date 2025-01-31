use godot::prelude::*;

mod resources;
mod result;
mod runtime;

struct Extension;

#[gdextension]
unsafe impl ExtensionLibrary for Extension {}
