extends Node


var score = 0
@onready var score_label = $ScoreLabel
@onready var death_lable = $"../Player/DeathLable"

func add_point():
	score += 1
	score_label.text = "You collected " + str(score) + " coins."
	
func display_death_screen():
	death_lable.text = "You died!"
