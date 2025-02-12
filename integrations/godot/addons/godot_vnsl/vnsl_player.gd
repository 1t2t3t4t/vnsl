extends Node

class_name VnslPlayer

@onready var vnsl_runtime: VnslRuntime = %VnslRuntime
@onready var ui: VnslPlayerUi = %UI

@export var entry_point: StringName = "MainScene"

var lock = false

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	vnsl_runtime.service_store.register_service("player", self)
	vnsl_runtime.service_store.register_service("ui", ui)

	vnsl_runtime.register_action_handler(TextInputActionHandler.new())
	vnsl_runtime.register_action_handler(BackgroundActionHandler.new())

	vnsl_runtime.load_scene(entry_point)
	vnsl_runtime.step()


func _input(event: InputEvent) -> void:
	if event.is_action_pressed("force_save"):
		var snapshot_res := vnsl_runtime.take_snapshot()
		if snapshot_res.is_ok():
			var snapshot = snapshot_res.result() as String
			var persistent := vnsl_runtime.persistent_store.duplicate(true) as PersistentStore
			persistent.runtime_snapshot_json = snapshot
			ResourceSaver.save(persistent, "res://save.tres")
			print("Save!!")

	if event is InputEventMouseButton:
		if event.button_index == 1 and event.is_pressed():
			step()


func step():
	if not lock:
		vnsl_runtime.step()

# Signals

func _on_vnsl_runtime_show_text(text: String) -> void:
	ui.set_display_text(text)


func _on_vnsl_runtime_set_character_id(id: String) -> void:
	ui.set_speaker(id)


func _on_vnsl_runtime_prompt_choices(choices: Array[VnslRuntimeChoice]) -> void:
	lock = true
	ui.choices_container.show()
	ui.choices_container.configure(choices)

	var selected := await ui.choices_container.choice_selected as VnslRuntimeChoice
	vnsl_runtime.select_choice(selected)

	ui.choices_container.hide()
	lock = false
	step()
