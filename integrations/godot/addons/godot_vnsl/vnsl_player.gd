extends Node

class_name VnslPlayer

@onready var vnsl_runtime: VnslRuntime = %VnslRuntime
@onready var ui: VnslPlayerUi = $UI

@export var entry_point: StringName = "MainScene"

var lock = false

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	vnsl_runtime.service_store.register_service("player", self)
	vnsl_runtime.service_store.register_service("ui", ui)

	vnsl_runtime.register_action_handler(TextInputActionHandler.new())

	vnsl_runtime.load_scene(entry_point)


func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.button_index == 1 and event.is_pressed():
			step()


func step():
	if not lock:
		vnsl_runtime.step()


func _on_vnsl_runtime_show_text(text: String) -> void:
	ui.set_display_text(text)


func _on_vnsl_runtime_set_character_id(id: String) -> void:
	ui.set_speaker(id)
