extends Node

@onready var vnsl_runtime: VnslRuntime = $VnslRuntime
var scene_ended := false

func _ready() -> void:
	var handler := TestActionHandler.new()
	self.get_script()
	vnsl_runtime.register_action_handler(handler)


func _process(delta: float) -> void:
	if not scene_ended:
		vnsl_runtime.step()


func _on_vnsl_runtime_scene_end() -> void:
	scene_ended = true


func _on_vnsl_runtime_set_character_id(id: String) -> void:
	print("Set char: %s" % [id])


func _on_vnsl_runtime_show_text(text: String) -> void:
	print("Show text: %s" % [text])
