extends Control

class_name VnslPlayerUi

@export var scripts_path: String = "res://scenes/"

@onready var speaker_label: Label = %SpeakerLabel
@onready var display_text_box: RichTextLabel = %DisplayTextBox
@onready var background_texture: TextureRect = %BackgroundTexture

@onready var choices_container: ChoicesContainer = %ChoicesContainer
@onready var text_input_prompt: TextInputPrompt = %TextInputPrompt

@onready var player: VnslPlayer = %VnslPlayer

func _ready() -> void:
	choices_container.hide()
	text_input_prompt.hide()
	player.set_ui_node(self)


func set_display_text(text: String):
	display_text_box.text = text
	player.vnsl_runtime.persistent_store.current_display_text = text


func set_speaker(name: String):
	speaker_label.text = name


func reset_ui_state():
	background_texture.texture = null
	display_text_box.text = ""
	speaker_label.text = ""

	text_input_prompt.hide()
	choices_container.hide()


func restore_ui_from_snapshot(store: PersistentStore):
	background_texture.texture = store.current_bg_texture
	display_text_box.text = store.current_display_text
	speaker_label.text = store.current_character

	text_input_prompt.restore_ui_from_snapshot(store)
	choices_container.restore_ui_from_snapshot(store)
