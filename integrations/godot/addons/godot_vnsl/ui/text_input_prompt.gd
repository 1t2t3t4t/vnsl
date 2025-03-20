extends Control

class_name TextInputPrompt

@onready var label: Label = %Label
@onready var line_edit: LineEdit = %LineEdit
@onready var submit_button: Button = %SubmitButton

signal submit_input(text: String)

func configure_from_action(action: VnslRuntimeAction):
	var action_label = action.get_arg_with_name("label")
	if action_label != null:
		label.text = action_label.get_as_string()
	else:
		label.text = ""


func _on_submit_button_pressed() -> void:
	submit_input.emit(line_edit.text)


func _on_line_edit_text_submitted(new_text: String) -> void:
	submit_input.emit(new_text)


func restore_ui_from_snapshot(store: PersistentStore):
	if store.current_text_input != null:
		show()
		configure_from_action(store.current_text_input)
