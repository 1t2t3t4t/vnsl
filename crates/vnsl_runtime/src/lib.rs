use block_runner::BlockRunner;
use block_stack::RunStack;
use vnsl_core::model::VnslScene;

mod block_runner;
mod block_stack;

#[derive(Debug, Default)]
struct RunContext {}

#[derive(Debug)]
pub struct Runtime {
    current_scene: VnslScene,
    context: RunContext,
    run_stack: RunStack,
}

pub enum RuntimeCommand {
    SetCharacterId(String),
    EndOfStack,
    None,
}

impl Runtime {
    pub fn new(scene: VnslScene) -> Self {
        let context = RunContext::default();
        let mut run_stack = RunStack::default();
        run_stack.push(BlockRunner::new(scene.main_block.clone()));
        Self {
            current_scene: scene,
            context,
            run_stack,
        }
    }

    pub fn step(&mut self) -> RuntimeCommand {
        let _ = self.run_stack.top_mut().map(|s| s.step(&mut self.context));
        RuntimeCommand::None
    }
}
