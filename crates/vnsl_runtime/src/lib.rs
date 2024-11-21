use block_stack::BlockStack;
use vnsl_core::model::VnslScene;

mod block_stack;

#[derive(Debug, Default)]
struct RunContext {
    block_stack: BlockStack,
}

#[derive(Debug)]
pub struct Runtime {
    current_scene: VnslScene,
    context: RunContext,
}

pub enum RuntimeStepCommand {
    None,
}

impl Runtime {
    pub fn new(scene: VnslScene) -> Self {
        Self {
            current_scene: scene,
            context: RunContext::default(),
        }
    }

    pub fn step(&mut self) -> RuntimeStepCommand {
        RuntimeStepCommand::None
    }
}
