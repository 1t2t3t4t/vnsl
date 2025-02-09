extends Control

class_name VnslPlayerUi

@onready var speaker_label: Label = %SpeakerLabel
@onready var display_text_box: RichTextLabel = %DisplayTextBox

func set_display_text(text: String):
	display_text_box.text = text


func set_speaker(name: String):
	speaker_label.text = name
