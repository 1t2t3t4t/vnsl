extends VnslActionHandler

class_name TextInputActionHandler

func _handle_action_name() -> String:
	return "textInput"


func _handle(action: VnslRuntimeAction, service_store: ServiceStore) -> bool:
	var ui := service_store.get_service("ui") as VnslPlayerUi
	var player := service_store.get_service("player") as VnslPlayer
	player.lock = true
	ui.text_input_prompt.show()
	ui.text_input_prompt.submit_input.connect(_on_text_submit.bind(player, ui))
	return false


func _on_text_submit(text: String, player: VnslPlayer, ui: VnslPlayerUi):
	print(text)
	player.lock = false
	ui.text_input_prompt.hide()
	player.step()
	ui.text_input_prompt.submit_input.disconnect(_on_text_submit)
	pass
