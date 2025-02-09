extends Node

@onready var vnsl_runtime: VnslRuntime = $VnslRuntime

func _ready() -> void:
	vnsl_runtime.register_action_handler(TestActionHandler.new())
	vnsl_runtime.service_store.register_service("my_service", SomeService.new())

	var res := vnsl_runtime.load_scene("MainScene")
	if res.is_err():
		print(res.err_message())


func _process(_delta: float) -> void:
	if not vnsl_runtime.scene_ended():
		print("step")
		vnsl_runtime.step()


func _on_vnsl_runtime_set_character_id(id: String) -> void:
	print("Set char: %s" % [id])


func _on_vnsl_runtime_show_text(text: String) -> void:
	print("Show text: %s" % [text])
