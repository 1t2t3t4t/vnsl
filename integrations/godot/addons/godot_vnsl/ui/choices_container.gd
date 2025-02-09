extends Control

class_name ChoicesContainer

@onready var button_container: VBoxContainer = %ButtonContainer

signal choice_selected(choice: VnslRuntimeChoice)

func _ready() -> void:
	clear_button()


func configure(choices: Array[VnslRuntimeChoice]):
	clear_button()
	for choice in choices:
		var button := Button.new()
		button.text = choice.text
		button.pressed.connect(func(): choice_selected.emit(choice))
		button_container.add_child(button)


func clear_button():
	for child in button_container.get_children():
		child.queue_free()
