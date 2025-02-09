extends VnslActionHandler

class_name TestActionHandler

func _handle_action_name() -> String:
	return "MyAction"


func _handle(action: VnslRuntimeAction, runtime: BaseVnslRuntime) -> void:
	var name := action.get_arg_with_name("name").get_as_string()
	print("Handling %s. Name is %s" % [action.name, name])
