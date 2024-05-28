use godot::prelude::*;
mod player;
#[cfg(feature = "db")]
mod database_node;
mod killzone;

struct FirstRustyGame;

#[gdextension]
unsafe impl ExtensionLibrary for FirstRustyGame {}