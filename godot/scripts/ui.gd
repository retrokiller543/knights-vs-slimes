extends CanvasLayer
class_name UI

@onready var score_lable: Label = %Score
@onready var healtbar: ProgressBar = %Healtbar
@onready var death_lable = %DeathLable

var score: int = 0:
	set(new_score):
		score = new_score
		_update_score_lable()
		
var player_health: int = 3:
	set(new_health):
		player_health = new_health
		_update_player_health()

func _ready() -> void:
	_update_score_lable()
	healtbar.max_value = 3

func _update_score_lable() -> void:
	score_lable.text = 'Score: ' + str(score)
	
func add_points(points: int) -> void:
	score += points
	
func _update_player_health() -> void:
	healtbar.value = player_health


func _on_player_hurt(new_health: int) -> void:
	player_health = new_health

func _on_player_death():
	death_lable.visible = true
