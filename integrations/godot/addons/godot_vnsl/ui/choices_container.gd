extends ScrollContainer

class_name ChoicesContainer

@onready var button_container: VBoxContainer = %ButtonContainer

func clear_button():
	for child in button_container.get_children():
		child.queue_free()
