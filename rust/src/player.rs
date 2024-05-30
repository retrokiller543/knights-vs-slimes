use godot::engine::{CharacterBody2D, ICharacterBody2D, AnimatedSprite2D, Timer, Engine};
use godot::engine::ProjectSettings;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    #[export]
    speed: f32,
    #[export]
    jump_velocity: f32,
    #[export]
    health: i32,
    #[export]
    state: PlayerState,

    timer: Option<Gd<Timer>>,
    animator: Option<Gd<AnimatedSprite2D>>,

    base: Base<CharacterBody2D>,
}

#[derive(Debug, PartialEq, Eq, GodotConvert, Var, Export)]
#[godot(via = i32)]
pub enum PlayerState {
    Idle,
    Running,
    Jumping,
    Hurt,
    Dead,
}

#[godot_api]
impl Player {
    #[signal]
    fn on_jump();
    #[signal]
    fn on_hurt(new_health: i32);
    #[signal]
    fn on_death();

    #[func]
    pub fn take_damage(&mut self, amount: i32) {
        if self.state == PlayerState::Dead {
            return;
        }

        let new_health = self.health - amount;

        if new_health <= 0 {
            self.state = PlayerState::Dead;
            self.base_mut().emit_signal("on_death".into(), &[]);
            Engine::singleton().set_time_scale(0.5);

            let mut animated_sprite_2d = self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
            animated_sprite_2d.set_animation("death".into());

            if let Some(ref mut timer) = self.timer {
                timer.start();
            }
        } else {
            self.base_mut().emit_signal("on_hurt".into(), &[new_health.to_variant()]);
            self.apply_damage_effect();
        }

        self.health = new_health;
    }

    #[func]
    fn get_direction(&self) -> f32 {
        Input::singleton().get_axis("move_left".into(), "move_right".into())
    }

    fn apply_damage_effect(&mut self) {
        if self.state == PlayerState::Hurt || self.state == PlayerState::Dead {
            return;
        }

        self.state = PlayerState::Hurt;

        let mut velocity = self.base().get_velocity();
        velocity.y = self.jump_velocity * 0.5;
        self.base_mut().set_velocity(velocity);

        let mut animated_sprite_2d = self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        animated_sprite_2d.set_animation("hurt".into());
    }

    fn update_velocity(&mut self, delta: f64) -> Vector2 {
        let gravity: f32 = ProjectSettings::singleton().get_setting("physics/2d/default_gravity".into()).to();
        let mut velocity = self.base().get_velocity();
        if !self.base().is_on_floor() {
            velocity.y += gravity * delta as f32;
        }

        let input = Input::singleton();
        if input.is_action_just_pressed("jump".into()) && self.base().is_on_floor() && self.health > 0 {
            velocity.y = self.jump_velocity;
            self.base_mut().emit_signal("on_jump".into(), &[]);
        }

        let direction = self.get_direction();
        if direction != 0.0 && self.health > 0 && self.state != PlayerState::Hurt && self.state != PlayerState::Dead {
            velocity.x = direction * self.speed;
        } else if self.health > 0 && self.state != PlayerState::Hurt && self.state != PlayerState::Dead {
            velocity.x = 0.0;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        velocity
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

    #[func]
    fn on_animation_finished(&mut self) {
        if let Some(ref mut animator) = self.animator {
            godot_print!("Animation finished: {}", animator.get_animation());
            if animator.get_animation() == "hurt".into() {
                self.state = PlayerState::Idle;
            } else if animator.get_animation() == "death".into() {
                self.state = PlayerState::Dead;
            }
        }
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 130.0,
            jump_velocity: -300.0,
            health: 3,
            timer: None,
            animator: None,
            state: PlayerState::Idle,
            base,
        }
    }

    fn ready(&mut self,) {
        if let Some(timer) = self.base().try_get_node_as::<Timer>("Timer") {
            self.timer = Some(timer);
        } else {
            godot_error!("Timer not found, please add a timer as a child to the player");
            //panic!("Timer not found, please add a timer as a child to the player");
        }
        
        let timer_callable = Callable::from_object_method(&self.to_gd(), "on_timer_timeout");

        if let Some(ref mut timer) = self.timer {
            timer.connect("timeout".into(), timer_callable);
        } else {
            godot_error!("Cannot connect timer to on_timer_timeout");
        }

        if let Some(animated_sprite_2d) = self.base().try_get_node_as::<AnimatedSprite2D>("AnimatedSprite2D") {
            self.animator = Some(animated_sprite_2d);
        } else {
            godot_error!("AnimatedSprite2D not found, please add an AnimatedSprite2D as a child to the player");
            //panic!("AnimatedSprite2D not found, please add an AnimatedSprite2D as a child to the player");
        }

        let animator_callable = Callable::from_object_method(&self.to_gd(), "on_animation_finished");

        if let Some(ref mut animator) = self.animator {
            animator.connect("animation_finished".into(), animator_callable);
        } else {
            godot_error!("Cannot connect animator to on_animator_finished");
        }
    }

    fn to_string(&self) -> GString {
        format!("Player(speed={}, jump_velocity={})", self.speed, self.jump_velocity).into()
    }

    fn physics_process(&mut self, delta: f64) {
        let mut animated_sprite_2d = self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let velocity = self.update_velocity(delta);

        // Handle animations
        if self.state == PlayerState::Hurt || self.state == PlayerState::Dead {
            animated_sprite_2d.play();
            return;
        }

        if self.base().is_on_floor() && self.health > 0 {
            if velocity.x == 0.0 {
                self.state = PlayerState::Idle;
                animated_sprite_2d.set_animation("idle".into());
            } else {
                self.state = PlayerState::Running;
                animated_sprite_2d.set_animation("run".into());
                animated_sprite_2d.set_flip_h(velocity.x < 0.0);
            }
        } else if self.health > 0 {
            self.state = PlayerState::Jumping;
            animated_sprite_2d.set_animation("jump".into());
        }

        animated_sprite_2d.play();
    }
}