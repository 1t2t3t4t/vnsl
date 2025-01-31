class_name VnslScriptLoader

extends ResourceFormatLoader

func _get_recognized_extensions() -> PackedStringArray:
	return ["vnsl"]

func _get_resource_type(path: String) -> String:
	return "VnslScript"
