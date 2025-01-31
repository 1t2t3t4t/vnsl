use godot::prelude::*;

mod result;
mod runtime;
mod scene_map;

struct Extension;

#[gdextension]
unsafe impl ExtensionLibrary for Extension {}
