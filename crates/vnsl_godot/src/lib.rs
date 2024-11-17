use godot::prelude::*;

mod vnsl;

struct Extension;

#[gdextension]
unsafe impl ExtensionLibrary for Extension {}
