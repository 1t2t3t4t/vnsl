extends Node

@export var scene_map: VnslSceneMap

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var ss := VnslScript.new()
	scene_map.set_scene("test", ss)
	print(scene_map.get_scene("t"))
	print(scene_map.get_scene("test"))
