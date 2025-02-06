extends VnslActionHandler

class_name TestActionHandler

func _handle_action_name() -> String:
	return "MyAction"


func _handle(action: VnslRuntimeAction) -> void:
	print("Handling", action.name)
