use godot::engine::{Area2D, CollisionShape2D, Engine, IArea2D, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Killzone {
    timer: Option<Gd<Timer>>,

    base: Base<Area2D>,
}

#[godot_api]
impl Killzone {
    #[func]
    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        godot_print!("Body entered: {:?}", body);

        Engine::singleton().set_time_scale(0.5);

        let mut collision = body.get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision.queue_free();

        if let Some(ref mut timer) = self.timer {
            godot_print!("Starting timer!");
            timer.start();
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
            timer: None,
            base
        }
    }

    fn ready(&mut self) {
        if let timer = self.base().get_node_as::<Timer>("Timer") {
            self.timer = Some(timer);
        }

        let callable = Callable::from_object_method(&self.to_gd(), "on_body_entered");
        let timer_callable = Callable::from_object_method(&self.to_gd(), "on_timer_timeout");

        // Connect the signals
        self.base_mut().connect("body_entered".into(), callable);

        if let Some(ref mut timer) = self.timer {
            timer.connect("timeout".into(), timer_callable);
        }
    }
}