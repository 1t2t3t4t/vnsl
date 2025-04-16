extends BaseActionHandler

class_name BackgroundActionHandler

func _handle_action_name() -> Array[String]:
	return ["bg"]


func _handle(action: VnslRuntimeAction, service_store: ServiceStore) -> Variant:
	var img_path := action.get_arg_with_name("path").get_as_string()
	var img := ResourceLoader.load(img_path) as Texture2D
	_get_ui(service_store).background_texture.texture = img
	_get_persistent_store(service_store).current_bg_texture = img
	return true
