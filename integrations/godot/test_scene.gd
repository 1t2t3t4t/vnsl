extends Node

@onready var vnsl_runtime: VnslRuntime = $VnslRuntime

func _ready() -> void:
	vnsl_runtime.step()
