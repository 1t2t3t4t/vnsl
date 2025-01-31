class_name VnslScriptSaver

extends ResourceFormatSaver

func _get_recognized_extensions(resource: Resource) -> PackedStringArray:
	return ["vnsl"]

func _recognize(resource: Resource) -> bool:
	return resource is VnslScript

func _save(resource: Resource, path: String, flags: int) -> Error:
	print(path)
	print(resource)
	var acc := FileAccess.open("./test", FileAccess.WRITE)
	acc.store_string("Yppp")
	return OK
