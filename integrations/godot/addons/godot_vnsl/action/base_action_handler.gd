extends VnslActionHandler

class_name BaseActionHandler

func _ready(service_store: ServiceStore) -> void:
	pass


func _get_runtime(service_store: ServiceStore) -> VnslRuntime:
	return service_store.get_service("runtime") as VnslRuntime


func _get_persistent_store(service_store: ServiceStore) -> PersistentStore:
	return service_store.get_service("persistent_store") as PersistentStore


func _get_ui(service_store: ServiceStore) -> VnslPlayerUi:
	return service_store.get_service("ui") as VnslPlayerUi


func _get_player(service_store: ServiceStore) -> VnslPlayer:
	return service_store.get_service("player") as VnslPlayer
