use godot::engine::{CharacterBody2D, ICharacterBody2D, AnimatedSprite2D};
use godot::engine::ProjectSettings;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    #[export]
    speed: f32,
    #[export]
    jump_velocity: f32,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 130.0,
            jump_velocity: -300.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut animated_sprite_2d = self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        // get the gravity from the project settings to be synced with RigidBody nodes.
        let gravity: f32 = ProjectSettings::singleton().get_setting("physics/2d/default_gravity".into()).to();

        // Add the gravity.
        let mut velocity = self.base().get_velocity();
        if !self.base().is_on_floor() {
            velocity.y += gravity * delta as f32;
        }

        let input = Input::singleton();

        // Handle jump.
        if input.is_action_just_pressed("jump".into()) && self.base().is_on_floor() {
            velocity.y = self.jump_velocity;
        }

        // check horizontal input
        let direction = input.get_axis("move_left".into(), "move_right".into());

        if direction != 0.0 {
            velocity.x = direction * self.speed;
        } else {
            velocity.x = 0.0;
        }

        // Apply the velocity to the player.
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        // Handle animations
        if self.base().is_on_floor() {
            if velocity.x == 0.0 {
                animated_sprite_2d.set_animation("idle".into());
            } else {
                animated_sprite_2d.set_animation("run".into());
                animated_sprite_2d.set_flip_h(velocity.x < 0.0);
            }
        } else {
            animated_sprite_2d.set_animation("jump".into());
        }
        animated_sprite_2d.play();
    }
}