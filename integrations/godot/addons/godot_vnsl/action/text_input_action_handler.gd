extends BaseActionHandler

class_name TextInputActionHandler

func _ready(store: ServiceStore):
	super._ready(store)
	_get_ui(store).text_input_prompt.submit_input.connect(_on_text_submitted.bind(store))


func _handle_action_name() -> Array[String]:
	return ["textInput"]


func _handle(action: VnslRuntimeAction, service_store: ServiceStore):
	var ui := _get_ui(service_store)
	var player := _get_player(service_store)
	_get_persistent_store(service_store).current_text_input = action.duplicate(true)

	ui.text_input_prompt.show()
	ui.text_input_prompt.configure_from_action(action)


func _on_text_submitted(text: String, store: ServiceStore):
	var action := _get_persistent_store(store).current_text_input
	if action == null:
		return

	var ui := _get_ui(store)
	var player := _get_player(store)
	var key = action.get_arg_with_name("key")
	player.vnsl_runtime.set_global_val_string(key.get_as_string(), text)

	ui.text_input_prompt.hide()
	player.step()

	_get_persistent_store(store).current_text_input = null
