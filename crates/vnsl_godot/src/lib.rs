use godot::prelude::*;

mod result;
mod runtime;

struct Extension;

#[gdextension]
unsafe impl ExtensionLibrary for Extension {}
