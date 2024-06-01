extends Area2D

enum State {TRACKING = 0, IDLE = 1}
signal state_changed(new_state: State)
signal target_changed(new_target: Node2D)
signal target_position_changed(new_position: Vector2)

var target_node: Node2D = null :
	set(new_target):
		if target_node != new_target:
			target_node = new_target
			target_changed.emit(target_node)

var current_target_pos: Vector2 = Vector2.ZERO :
	set(new_position):
		if current_target_pos != new_position:
			current_target_pos = new_position
			target_position_changed.emit(current_target_pos)

var state: State = State.IDLE :
	set(new_value):
		if state != new_value:
			state = new_value
			state_changed.emit(state)

var previous_target_pos: Vector2 = Vector2.ZERO
var target_is_in_area: bool = false : 
	set(new_value):
		if target_is_in_area != new_value:
			target_is_in_area = new_value

func _on_body_entered(body: Node2D):
	if not body.is_class("Player"):
		return
	
	var over_lapping_bodies: Array[Node2D] = get_overlapping_bodies()
	for overlapping_body in over_lapping_bodies:
		if body.is_class("Player") and target_node == null:
			target_node = overlapping_body
			target_is_in_area = true
			state = State.TRACKING

func _on_body_exited(body):
	if not body.is_class("Player"):
		return
	
	if body == target_node:
		target_node = null
		target_is_in_area = false
		state = State.IDLE

func _on_reset_nav_path_timeout():
	if target_node != null and target_is_in_area:
		current_target_pos = target_node.global_position
