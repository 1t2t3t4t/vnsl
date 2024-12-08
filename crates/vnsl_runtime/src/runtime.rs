use vnsl_core::model::{VnslAction, VnslBlock, VnslScene};

use crate::{
    block_runner::{BlockCommand, BlockRunner},
    block_stack::RunStack,
    runtime_result::RuntimeResult,
    RunContext, RuntimeDelegateHandler,
};

#[derive(Debug)]
pub struct Runtime {
    current_scene: VnslScene,
    context: RunContext,
    run_stack: RunStack,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeCommand {
    SetCharacterId(String),
    ShowText(String),
    ExecuteAction(VnslAction),
    EndOfStack,
    NoOps,
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
        if self.run_stack.len() == 0 {
            return Ok(RuntimeCommand::EndOfStack);
        }
        let Some(cmd) = self
            .run_stack
            .top_mut()
            .map(|s| s.step(&mut self.context, delegate_handler))
        else {
            println!("Could not get cmd from block runner {:#?}", self.run_stack);
            return Ok(RuntimeCommand::NoOps);
        };

        match cmd? {
            BlockCommand::ForkBlock(vnsl_block) => {
                self.fork_block(vnsl_block);
                self.step(delegate_handler)
            }
            BlockCommand::Jump(vnsl_jump) => {
                let Some(label) = self.current_scene.labels.get(&vnsl_jump.to_label) else {
                    todo!("Handle missing label")
                };
                self.fork_block(label.block.clone());
                Ok(RuntimeCommand::NoOps)
            }
            BlockCommand::Global(vnsl_global) => {
                self.context
                    .lua_runtime
                    .set_globals_val_data_type(&vnsl_global.name, vnsl_global.value.clone())?;
                Ok(RuntimeCommand::NoOps)
            }

            BlockCommand::DisplayText(vnsl_dialogue) => {
                Ok(RuntimeCommand::ShowText(vnsl_dialogue.text.clone()))
            }
            BlockCommand::SetCharacter(vnsl_set_character) => Ok(RuntimeCommand::SetCharacterId(
                vnsl_set_character.id.clone(),
            )),
            BlockCommand::Action(vnsl_action) => {
                Ok(RuntimeCommand::ExecuteAction(vnsl_action.clone()))
            }
            BlockCommand::Choices(vnsl_choices) => todo!(),
            BlockCommand::EndOfStack => {
                self.pop_block_stack();
                self.step(delegate_handler)
            }
            BlockCommand::NoOps => Ok(RuntimeCommand::NoOps),
        }
    }

    fn pop_block_stack(&mut self) {
        self.run_stack.pop();
    }

    fn fork_block(&mut self, block: VnslBlock) {
        let runner = BlockRunner::new(block);
        self.run_stack.push(runner);
    }
}
