extends Control

@onready var not_implemented = %"NotImplemented"
@onready var not_implemented_timer = $CanvasLayer/VBoxContainer2/NotImplementedTimer

func _on_start_button_pressed():
	get_tree().change_scene_to_file("res://scenes/level_1.tscn")


func _on_options_button_pressed():
	not_implemented.visible = true
	not_implemented_timer.start()


func _on_quit_button_pressed():
	get_tree().quit()


func _on_not_implemented_timer_timeout():
	not_implemented.visible = false
