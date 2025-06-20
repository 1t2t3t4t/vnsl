extends BaseVnslRuntime

class_name VnslRuntime

@export var persistent_store := PersistentStore.new()


func load_persistent(store: PersistentStore):
	persistent_store = store
	_register_services()
	load_snapshot(store.runtime_snapshot).print_err_if_available()


func _register_services():
	service_store.register_service("runtime", self)
	service_store.register_service("persistent_store", persistent_store)


func _ready() -> void:
	_register_services()


func load_scripts(vnsl_scripts_path: String):
	var scripts := _scan_scripts(vnsl_scripts_path)
	construct_scene_map(scripts)


func _scan_scripts(current_path: String) -> Array[String]:
	var dir := DirAccess.open(current_path)
	var result: Array[String] = []

	for sub_dir in dir.get_directories():
		result.append_array(_scan_scripts(current_path.path_join(sub_dir)))

	for file in dir.get_files():
		if file.get_extension() == "vnsl":
			result.append(current_path.path_join(file))

	return result
