extends CharacterBody2D
enum State {TRACKING, HOME}
signal state_changed(changed_to: State)

@onready var reset_nav_path: Timer = $reset_nav_path
@onready var nav_agent: NavigationAgent2D = $NavigationAgent2D
@onready var bird_enemy: Node2D = $".."

var player: Player
var speed: float
var state: State = State.HOME :
	set(new_value):
		if state != new_value:
			state = new_value
			state_changed.emit(state)
var tracking_area: Area2D

var home_point: Vector2 = Vector2.ZERO


# Called when the node enters the scene tree for the first time.
func _ready():
	player = bird_enemy.player
	speed = bird_enemy.speed
	home_point = global_position
	tracking_area = bird_enemy.tracking_area
	
func make_path() -> void:
	nav_agent.set_target_position(player.global_position)
	
func make_path_home() -> void:
	nav_agent.set_target_position(player.global_position)
	
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta) -> void:
	if not state == State.TRACKING:
		make_path_home()
	var dir = to_local(nav_agent.get_next_path_position()).normalized()
	velocity = dir * speed
	move_and_slide()
	

func _on_reset_nav_path_timeout():
	if state == State.TRACKING:
		make_path()

func _on_target_area_body_entered(_body: Node2D):
	state = State.TRACKING

func _on_target_area_body_exited(_body: Node2D):
	state = State.HOME


func _on_state_changed(changed_to: State):
	if changed_to == State.TRACKING:
		make_path()
	elif changed_to == State.HOME:
		make_path_home()
