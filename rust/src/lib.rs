use godot::prelude::*;
mod player;
mod killzone;

struct FirstRustyGame;

#[cfg(feature = "gdext")]
#[gdextension]
unsafe impl ExtensionLibrary for FirstRustyGame {}