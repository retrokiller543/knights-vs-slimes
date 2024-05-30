use godot::engine::{Area2D, CollisionShape2D, Engine, IArea2D, Timer};
use godot::prelude::*;

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Killzone {
    #[export]
    damage: i32,

    // should maybe generate the timer instead of importing it/having it the end user put into the tree
    timer: Option<Gd<Timer>>,

    base: Base<Area2D>,
}

#[godot_api]
impl Killzone {
    #[signal]
    fn killzone_entered();

    #[func]
    /// # Handles the death of the player when it collides with the killzone
    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        self.base_mut().emit_signal("killzone_entered".into(), &[]);

        let class = body.get_class();
        if class == "Player".into() {
            let mut player = body.get_node_as::<Player>(".");
            player.bind_mut().take_damage(self.damage);
        }
    }

    #[func]
    fn on_timer_timeout(&self) {
        Engine::singleton().set_time_scale(1.0);

        Engine::singleton()
            .get_main_loop()
            .unwrap()
            .cast::<SceneTree>()
            .reload_current_scene();
    }
}

#[godot_api]
impl IArea2D for Killzone {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            damage: 1,
            timer: None,
            base
        }
    }

    fn ready(&mut self) {
        let timer = self.base().get_node_as::<Timer>("Timer");
        self.timer = Some(timer);

        let callable = Callable::from_object_method(&self.to_gd(), "on_body_entered");
        let timer_callable = Callable::from_object_method(&self.to_gd(), "on_timer_timeout");

        // Connect the signals
        self.base_mut().connect("body_entered".into(), callable);

        if let Some(ref mut timer) = self.timer {
            timer.connect("timeout".into(), timer_callable);
        }
    }
}