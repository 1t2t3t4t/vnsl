class_name VnslScriptSaver

extends ResourceFormatSaver

func _get_recognized_extensions(resource: Resource) -> PackedStringArray:
	return ["vnsl"]


func _recognize(resource: Resource) -> bool:
	return resource is VnslScript


func _save(resource: Resource, path: String, flags: int) -> Error:
	var vnsl_script := resource as VnslScript
	var file := FileAccess.open(path, FileAccess.WRITE)
	var open_err := FileAccess.get_open_error()
	if open_err != OK:
		return open_err
	file.store_string(vnsl_script.content)
	return file.get_error()
