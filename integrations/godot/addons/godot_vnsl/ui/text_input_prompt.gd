extends Control

class_name TextInputPrompt

@onready var label: Label = %Label
@onready var line_edit: LineEdit = %LineEdit
@onready var submit_button: Button = %SubmitButton

signal submit_input(text: String)


func _on_submit_button_pressed() -> void:
	submit_input.emit(line_edit.text)


func _on_line_edit_text_submitted(new_text: String) -> void:
	submit_input.emit(new_text)
