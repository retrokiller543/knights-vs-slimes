extends AnimatedSprite2D

@onready var animated_sprite_2d = $"."
@onready var player = $".."



# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if animated_sprite_2d.current_animation  == "hurt" and animated_sprite_2d.animation_finished:
		player.state = 0
		
