extends BaseVnslRuntime

class_name VnslRuntime

@export var vnsl_scripts_path: String = "res://scripts/"

func _ready() -> void:
	service_store.register_service("runtime", self)
	var scripts := _scan_scripts(vnsl_scripts_path)
	construct_scene_map(scripts)


func _scan_scripts(current_path: String) -> Array[String]:
	var dir := DirAccess.open(current_path)
	var result: Array[String] = []

	for sub_dir in dir.get_directories():
		result.append_array(_scan_scripts(current_path.path_join(sub_dir)))

	for file in dir.get_files():
		result.append(current_path.path_join(file))

	return result
