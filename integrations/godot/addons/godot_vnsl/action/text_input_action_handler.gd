extends BaseActionHandler

class_name TextInputActionHandler

func _handle_action_name() -> String:
	return "textInput"


func _handle(action: VnslRuntimeAction, service_store: ServiceStore):
	var ui := _get_ui(service_store)
	var player := _get_player(service_store)
	player.lock = true
	ui.text_input_prompt.show()

	var label = action.get_arg_with_name("label")
	if label != null:
		ui.text_input_prompt.label.text = label.get_as_string()
	else:
		ui.text_input_prompt.label.text = ""

	var text = await ui.text_input_prompt.submit_input as String
	var key = action.get_arg_with_name("key")
	player.vnsl_runtime.set_global_val_string(key.get_as_string(), text)

	player.lock = false
	ui.text_input_prompt.hide()
	player.step()
