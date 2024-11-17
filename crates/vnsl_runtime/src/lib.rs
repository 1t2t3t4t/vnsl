use vnsl_core::model::VnslScene;

pub struct Runtime {
    current_scene: VnslScene,
}

pub enum RuntimeStepCommand {
    None,
}

impl Runtime {
    pub const fn new(scene: VnslScene) -> Self {
        Self {
            current_scene: scene,
        }
    }

    pub fn step(&mut self) -> RuntimeStepCommand {
        RuntimeStepCommand::None
    }
}
