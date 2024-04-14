use godot::prelude::*;

mod my_node;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
