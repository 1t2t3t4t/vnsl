#[cfg(test)]
mod runtime_snapshot;
mod text;

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
    ChangeScene(String),
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

    pub fn scene_ended(&self) -> bool {
        self.run_stack.len() == 0
    }

    pub fn step(&mut self) -> RuntimeResult<RuntimeCommand> {
        let Some(current_scene) = &self.current_scene else {
            return Err(RuntimeError::NoSceneLoaded);
        };

        if self.scene_ended() {
            return Ok(RuntimeCommand::EndOfScene);
        }

        let Some(cmd) = self.run_stack.top_mut().map(|s| s.step(&mut self.context)) else {
            return Err(RuntimeError::EndOfStack);
        };

        let cmd = match cmd? {
            BlockCommand::ForkBlock(vnsl_block) => {
                self.fork_block(vnsl_block);
                self.step()
            }
            BlockCommand::Jump(vnsl_jump) => {
                let Some(label) = current_scene.labels.get(&vnsl_jump.to_label) else {
                    return Err(RuntimeError::InvalidLabelJump(vnsl_jump.to_label));
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

            BlockCommand::DisplayText(vnsl_dialogue) => Ok(RuntimeCommand::ShowText(
                text::process_display_text(vnsl_dialogue.text, &self.context),
            )),
            BlockCommand::SetCharacter(vnsl_set_character) => {
                Ok(RuntimeCommand::SetCharacterId(vnsl_set_character.id))
            }
            BlockCommand::Action(vnsl_action) => Ok(RuntimeCommand::ExecuteAction(vnsl_action)),
            BlockCommand::Choices(vnsl_choices) => Ok(RuntimeCommand::PromptChoices(vnsl_choices)),
            BlockCommand::ChangeScene(vnsl_go_to) => {
                self.clear_block_stack();
                Ok(RuntimeCommand::ChangeScene(vnsl_go_to.scene_id))
            }
            BlockCommand::Return => {
                self.pop_block_stack();
                self.step()
            }
            BlockCommand::NoOps => {
                self.clear_end_block_stack();
                self.step()
            }
        };

        self.clear_end_block_stack();

        cmd
    }

    pub fn select_choice(&mut self, choice: &VnslChoice) {
        self.fork_block(choice.block.clone());
    }

    fn clear_end_block_stack(&mut self) {
        while self.run_stack.top().map(|b| b.block_ended()) == Some(true) {
            self.pop_block_stack();
        }
    }

    fn pop_block_stack(&mut self) {
        self.run_stack.pop();
    }

    fn clear_block_stack(&mut self) {
        while self.run_stack.len() > 0 {
            self.run_stack.pop();
        }
    }

    fn fork_block(&mut self, block: VnslBlock) {
        let runner = BlockRunner::new(block);
        self.run_stack.push(runner);
    }
}
