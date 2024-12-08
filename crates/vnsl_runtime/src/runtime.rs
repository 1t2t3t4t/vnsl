#[cfg(test)]
mod runtime_snapshot;

use vnsl_core::model::{VnslAction, VnslBlock, VnslChoice, VnslChoices, VnslScene};

use crate::{
    block_runner::{BlockCommand, BlockRunner},
    block_stack::RunStack,
    runtime_result::{RuntimeError, RuntimeResult},
    RunContext,
};

#[derive(Debug)]
pub struct Runtime {
    current_scene: Option<VnslScene>,
    context: RunContext,
    run_stack: RunStack,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeCommand {
    SetCharacterId(String),
    ShowText(String),
    ExecuteAction(VnslAction),
    PromptChoices(VnslChoices),
    EndOfScene,
}

impl Runtime {
    pub fn new() -> Self {
        let context = RunContext::default();
        let run_stack = RunStack::default();
        Self {
            current_scene: None,
            context,
            run_stack,
        }
    }

    pub fn load_scene(&mut self, scene: VnslScene) {
        let main_block = scene.main_block.clone();
        self.current_scene = Some(scene);
        self.run_stack.push(BlockRunner::new(main_block));
    }

    pub fn step(&mut self) -> RuntimeResult<RuntimeCommand> {
        let Some(current_scene) = &self.current_scene else {
            return Err(RuntimeError::NoSceneLoaded);
        };

        let Some(cmd) = self.run_stack.top_mut().map(|s| s.step(&mut self.context)) else {
            return Ok(RuntimeCommand::EndOfScene);
        };

        match cmd? {
            BlockCommand::ForkBlock(vnsl_block) => {
                self.fork_block(vnsl_block);
                self.step()
            }
            BlockCommand::Jump(vnsl_jump) => {
                let Some(label) = current_scene.labels.get(&vnsl_jump.to_label) else {
                    todo!("Handle missing label")
                };
                self.fork_block(label.block.clone());
                self.step()
            }
            BlockCommand::Global(vnsl_global) => {
                self.context
                    .lua_runtime
                    .set_globals_val_data_type(&vnsl_global.name, vnsl_global.value)?;
                self.step()
            }

            BlockCommand::DisplayText(vnsl_dialogue) => {
                Ok(RuntimeCommand::ShowText(vnsl_dialogue.text.clone()))
            }
            BlockCommand::SetCharacter(vnsl_set_character) => Ok(RuntimeCommand::SetCharacterId(
                vnsl_set_character.id.clone(),
            )),
            BlockCommand::Action(vnsl_action) => Ok(RuntimeCommand::ExecuteAction(vnsl_action)),
            BlockCommand::Choices(vnsl_choices) => Ok(RuntimeCommand::PromptChoices(vnsl_choices)),
            BlockCommand::EndOfStack => {
                self.pop_block_stack();
                self.step()
            }
            BlockCommand::NoOps => self.step(),
        }
    }

    pub fn select_choice(&mut self, choice: &VnslChoice) {
        self.fork_block(choice.block.clone());
    }

    fn pop_block_stack(&mut self) {
        self.run_stack.pop();
    }

    fn fork_block(&mut self, block: VnslBlock) {
        let runner = BlockRunner::new(block);
        self.run_stack.push(runner);
    }
}
