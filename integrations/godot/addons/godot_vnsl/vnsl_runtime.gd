extends BaseVnslRuntime

class_name VnslRuntime

@export var scripts_path: String = "res://scripts/"

func _ready() -> void:
	var result := _scan_scripts(scripts_path)
	construct_scene_map(result)
	var res := load_scene("MainScene")
	print(res.err_message())


func _scan_scripts(current_path: String) -> Array[String]:
	var dir := DirAccess.open(current_path)
	var result: Array[String] = []

	for sub_dir in dir.get_directories():
		result.append_array(_scan_scripts(current_path.path_join(sub_dir)))

	for file in dir.get_files():
		result.append(current_path.path_join(file))

	return result
