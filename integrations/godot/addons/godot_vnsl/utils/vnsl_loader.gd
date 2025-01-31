class_name VnslScriptLoader

extends ResourceFormatLoader

func _get_recognized_extensions() -> PackedStringArray:
	return ["vnsl"]

func _get_resource_type(path: String) -> String:
	return "Resource"

func _load(path: String, original_path: String, use_sub_threads: bool, cache_mode: int) -> Variant:
	var file := FileAccess.open(path, FileAccess.READ)
	var open_err := FileAccess.get_open_error()
	if open_err != OK:
		return open_err
	var txt := file.get_as_text()
	var script := VnslScript.new()
	script.content = txt
	return script
