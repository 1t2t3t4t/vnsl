extends Node2D

@onready var vnsl_player_ui: VnslPlayerUi = %VnslPlayerUi

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	vnsl_player_ui.player.on_scene_ended.connect(_on_scene_ended)


func _on_scene_ended(scene_name: String):
	print("Scene ended:", scene_name)
	vnsl_player_ui.hide()
