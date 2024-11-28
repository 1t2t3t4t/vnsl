use block_runner::{BlockCommand, BlockRunner};
use block_stack::RunStack;
use lua_runtime::{LuaRuntime, ToLua};
use runtime_result::RuntimeResult;
use vnsl_core::model::{VnslBlock, VnslScene};

mod block_runner;
mod block_stack;
mod lua_runtime;
mod runtime_result;

pub trait RuntimeDelegateHandler {}

#[derive(Debug, Default)]
pub struct RunContext {
    lua_runtime: LuaRuntime,
}

#[derive(Debug)]
pub struct Runtime {
    current_scene: VnslScene,
    context: RunContext,
    run_stack: RunStack,
}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn step(
        &mut self,
        delegate_handler: &impl RuntimeDelegateHandler,
    ) -> RuntimeResult<RuntimeCommand> {
        let Some(cmd) = self
            .run_stack
            .top_mut()
            .map(|s| s.step(&mut self.context, delegate_handler))
        else {
            return Ok(RuntimeCommand::None);
        };

        match cmd? {
            BlockCommand::ForkBlock(vnsl_block) => {
                self.fork_block(vnsl_block);
                self.step(delegate_handler)?;
                Ok(RuntimeCommand::None)
            }
            BlockCommand::Jump(vnsl_jump) => {
                let Some(label) = self.current_scene.labels.get(&vnsl_jump.to_label) else {
                    todo!("Handle missing label")
                };
                self.fork_block(label.block.clone());
                Ok(RuntimeCommand::None)
            }
            BlockCommand::Global(vnsl_global) => {
                self.context
                    .lua_runtime
                    .set_globals_val_data_type(&vnsl_global.name, vnsl_global.value.clone())?;
                Ok(RuntimeCommand::None)
            }

            BlockCommand::DisplayText(vnsl_dialogue) => todo!(),
            BlockCommand::SetCharacter(vnsl_set_character) => todo!(),
            BlockCommand::Action(vnsl_action) => todo!(),
            BlockCommand::Choices(vnsl_choices) => todo!(),
            BlockCommand::EndOfStack => todo!(),
            BlockCommand::None => todo!(),
        }
    }

    fn fork_block(&mut self, block: VnslBlock) {
        let runner = BlockRunner::new(block);
        self.run_stack.push(runner);
    }
}
