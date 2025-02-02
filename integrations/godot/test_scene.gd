extends Node

@onready var vnsl_runtime: VnslRuntime = $VnslRuntime

func _ready() -> void:
	var handler := TestActionHandler.new()
	self.get_script()
	vnsl_runtime.register_action_handler(handler)


func _process(_delta: float) -> void:
	if not vnsl_runtime.scene_ended():
		print("step")
		vnsl_runtime.step()


func _on_vnsl_runtime_set_character_id(id: String) -> void:
	print("Set char: %s" % [id])


func _on_vnsl_runtime_show_text(text: String) -> void:
	print("Show text: %s" % [text])
