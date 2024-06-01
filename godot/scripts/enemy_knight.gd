extends CharacterBody2D

enum State { TRACKING, HOME }
signal state_changed(changed_to: State)

@onready var reset_nav_path: Timer = $reset_nav_path
@onready var nav_agent: NavigationAgent2D = $NavigationAgent2D
@onready var bird_enemy: Node2D = $".."
@onready var target_area = $"../TargetArea"
@onready var animated_sprite = $AnimatedSprite2D2

var player: Player
var speed: float
var state: State = State.HOME :
	set(new_value):
		if state != new_value:
			state = new_value
			state_changed.emit(state)
			if state == State.HOME:
				make_path_home()
			elif state == State.TRACKING:
				make_path()

var tracking_area: Area2D
var home_point: Vector2 = Vector2.ZERO
var target_node: Node2D = null :
	set(new_target):
		if new_target == null:
			state = State.HOME
			target_position = home_point
		else:
			state = State.TRACKING
		
		target_node = new_target
		make_path()

var target_position: Vector2 = Vector2.ZERO

func _ready():
	player = bird_enemy.player
	speed = bird_enemy.speed
	home_point = global_position
	tracking_area = bird_enemy.tracking_area

func make_path() -> void:
	nav_agent.set_target_position(target_position)

func make_path_home() -> void:
	nav_agent.set_target_position(home_point)

func _physics_process(_delta) -> void:
	if target_node:
		animated_sprite.play("flying")
		move_with_collision_avoidance()
	else:
		move_with_collision_avoidance()

	if global_position.distance_to(home_point) < 1.0 and state == State.HOME:
		animated_sprite.play("idle")

func move_with_collision_avoidance() -> void:
	if nav_agent.is_navigation_finished():
		return

	# Get the next path position and the safe velocity from the navigation agent
	var target = nav_agent.get_next_path_position()
	var safe_velocity = nav_agent.get_velocity()

	# Move towards the target while avoiding collisions
	var direction = (target - global_position).normalized()
	nav_agent.velocity = direction * speed

	# Flip the sprite based on movement direction
	if direction.x != 0:
		animated_sprite.flip_h = direction.x > 0

	# Get the safe velocity to avoid obstacles
	nav_agent.velocity = nav_agent.get_velocity()
	
	velocity = safe_velocity

	# Apply the velocity to the character
	move_and_slide()

func _on_target_area_target_changed(new_target):
	target_node = new_target

func _on_target_area_target_position_changed(new_position):
	target_position = new_position
	make_path()

func _on_navigation_agent_2d_target_reached():
	if state == State.TRACKING:
		animated_sprite.play("idle")
