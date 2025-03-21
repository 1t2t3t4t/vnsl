extends Node

class_name VnslPlayer

@onready var vnsl_runtime: VnslRuntime = %VnslRuntime
@onready var ui: VnslPlayerUi = %UI

@export var entry_point: StringName = "MainScene"

var _handlers := [
	TextInputActionHandler.new(),
	BackgroundActionHandler.new()
] as Array[BaseActionHandler]

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	ui._player = self
	ui.choices_container.choice_selected.connect(_on_choice_selected)

	vnsl_runtime.service_store.register_service("player", self)
	vnsl_runtime.service_store.register_service("ui", ui)

	for handler in _handlers:
		vnsl_runtime.register_action_handler(handler)
		handler._ready(vnsl_runtime.service_store)

	start_scene(entry_point)


func _input(event: InputEvent) -> void:
	if event.is_action_pressed("force_save"):
		var snapshot_res := take_snapshot()
		if snapshot_res != null:
			ResourceSaver.save(snapshot_res, "res://save.save.tres")
			print("Save!!")

	if event.is_action_pressed("force_load"):
		if FileAccess.file_exists("res://save.save.tres"):
			var store := ResourceLoader.load(
				"res://save.save.tres",
				"PersistentStore",
				ResourceLoader.CACHE_MODE_IGNORE_DEEP
			) as PersistentStore
			load_snapshot(store)
			print("Loaded!!")

	if event is InputEventMouseButton:
		if event.button_index == 1 and event.is_pressed():
			step()


func step():
	if vnsl_runtime.scene_ended():
		get_tree().quit()

	if !_is_locked():
		vnsl_runtime.step().print_err_if_available()


func start_scene(name: StringName):
	vnsl_runtime.load_scene(name).print_err_if_available()
	step()


func take_snapshot() -> PersistentStore:
	var snapshot_res := vnsl_runtime.take_snapshot()
	if snapshot_res.is_err():
		return null
	var snapshot = snapshot_res.result() as String
	var persistent := vnsl_runtime.persistent_store.duplicate(true) as PersistentStore
	persistent.runtime_snapshot_json = snapshot
	return persistent


func load_snapshot(store: PersistentStore):
	ui.reset_ui_state()
	vnsl_runtime.load_persistent(store)
	ui.restore_ui_from_snapshot(store)


func _is_locked() -> bool:
	return ui.choices_container.visible || ui.text_input_prompt.visible


# Signals

func _on_vnsl_runtime_show_text(text: String) -> void:
	ui.set_display_text(text)


func _on_vnsl_runtime_set_character_id(id: String) -> void:
	ui.set_speaker(id)


func _on_vnsl_runtime_prompt_choices(choices: Array[VnslRuntimeChoice]) -> void:
	vnsl_runtime.persistent_store.current_choice_selection = choices.duplicate(true)
	ui.choices_container.show()
	ui.choices_container.configure(choices)


func _on_vnsl_runtime_change_scene(scene_name: String) -> void:
	start_scene(scene_name)


func _on_choice_selected(selected: VnslRuntimeChoice):
	vnsl_runtime.select_choice(selected)

	ui.choices_container.hide()
	vnsl_runtime.persistent_store.current_choice_selection = []
	step()
