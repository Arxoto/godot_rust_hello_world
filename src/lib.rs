//! see https://godot-rust.github.io/book/intro/hello-world.html
//!
//! When exporting your project, you need to use paths inside res://.
//! Outside paths like .. are not supported.

use godot::prelude::*;

mod adapter;

mod attrs;

struct AbilitySystemComponent;

#[gdextension]
unsafe impl ExtensionLibrary for AbilitySystemComponent {}
