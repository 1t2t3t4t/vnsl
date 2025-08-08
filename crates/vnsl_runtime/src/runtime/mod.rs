#[cfg(test)]
mod snapshot_test;
#[cfg(test)]
mod test;

mod text;

use vnsl_core::model::{VnslAction, VnslBlock, VnslChoice, VnslDataType, VnslScene};

use crate::{
    block_runner::{BlockCommand, BlockRunner},
    result::{RuntimeError, RuntimeResult},
    runstack::RunStack,
    snapshot::Snapshot,
    RunContext,
};

#[derive(Debug, Default)]
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
    PromptChoices(Vec<VnslChoice>),
    ChangeScene(String),
    EndOfScene,
}

impl Runtime {
    pub fn get_current_scene(&self) -> Option<&VnslScene> {
        self.current_scene.as_ref()
    }

    pub fn get_run_context(&self) -> &RunContext {
        &self.context
    }

    pub fn get_run_stack(&self) -> &RunStack {
        &self.run_stack
    }

    pub fn load_scene(&mut self, scene: VnslScene) {
        let main_block = scene.main_block.clone();
        self.current_scene = Some(scene);
        self.run_stack.push(BlockRunner::new(main_block));
    }

    pub fn scene_ended(&self) -> bool {
        self.run_stack.len() == 0
    }

    pub fn process_current_command(&mut self) -> RuntimeResult<RuntimeCommand> {
        if self.scene_ended() {
            return Ok(RuntimeCommand::EndOfScene);
        }

        let Some(cmd) = self
            .run_stack
            .top_mut()
            .map(|s| s.get_current_command(&mut self.context, -1))
        else {
            return Err(RuntimeError::EndOfStack);
        };

        self.process_block_command(cmd?)
    }

    pub fn step(&mut self) -> RuntimeResult<RuntimeCommand> {
        if self.scene_ended() {
            return Ok(RuntimeCommand::EndOfScene);
        }

        let Some(cmd) = self.run_stack.top_mut().map(|s| s.step(&mut self.context)) else {
            return Err(RuntimeError::EndOfStack);
        };

        self.process_block_command(cmd?)
    }

    fn process_block_command(&mut self, cmd: BlockCommand) -> RuntimeResult<RuntimeCommand> {
        let Some(current_scene) = &self.current_scene else {
            return Err(RuntimeError::NoSceneLoaded);
        };
        let cmd = match cmd {
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
                let Some(name) = self
                    .context
                    .lua_runtime
                    .try_get_globals_val(&vnsl_set_character.id)
                else {
                    return Ok(RuntimeCommand::SetCharacterId(vnsl_set_character.id));
                };
                Ok(RuntimeCommand::SetCharacterId(name))
            }
            BlockCommand::Action(vnsl_action) => Ok(RuntimeCommand::ExecuteAction(vnsl_action)),
            BlockCommand::Choices(vnsl_choices) => {
                let mut choices = vec![];
                for choice in vnsl_choices.choices {
                    if let Some(eval) = &choice.condition {
                        let result = self.context.lua_runtime.eval_expr::<bool>(&eval.code)?;
                        if result == true {
                            choices.push(choice);
                        }
                    } else {
                        choices.push(choice);
                    }
                }
                Ok(RuntimeCommand::PromptChoices(choices))
            }
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

    pub fn set_global_val(&mut self, name: &str, val: VnslDataType) -> RuntimeResult<()> {
        self.context
            .lua_runtime
            .set_globals_val_data_type(name, val)
    }

    pub fn get_global_val(&self, name: &str) -> Option<VnslDataType> {
        self.context.lua_runtime.try_get_globals_val(name)
    }

    pub fn snapshot(&self) -> Snapshot {
        self.into()
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

impl From<Snapshot> for Runtime {
    fn from(value: Snapshot) -> Self {
        let ctx = RunContext::default();
        ctx.lua_runtime
            .import_globals(value.lua_globals)
            .expect("import globals");
        Self {
            current_scene: value.current_scene,
            context: ctx,
            run_stack: value.run_stack,
        }
    }
}

#[cfg(debug_assertions)]
impl Runtime {
    pub fn run_stack_string(&self) -> String {
        format!("{:#?}", self.run_stack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_gloal_var() {
        let mut runtime = Runtime::default();
        runtime
            .set_global_val("test", VnslDataType::String("test".to_string()))
            .unwrap();
        assert_eq!(
            runtime.get_global_val("test"),
            Some(VnslDataType::String("test".to_string()))
        );
        runtime
            .set_global_val("num", VnslDataType::Number(42.0))
            .unwrap();
        assert_eq!(
            runtime.get_global_val("num"),
            Some(VnslDataType::Number(42.0))
        );
        runtime
            .set_global_val("bool", VnslDataType::Bool(true))
            .unwrap();
        assert_eq!(
            runtime.get_global_val("bool"),
            Some(VnslDataType::Bool(true))
        );
        assert_eq!(runtime.get_global_val("empty"), None);
    }
}
