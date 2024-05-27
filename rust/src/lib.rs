use godot::prelude::*;
mod player;

struct FirstRustyGame;

#[gdextension]
unsafe impl ExtensionLibrary for FirstRustyGame {}