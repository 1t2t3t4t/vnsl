#[cfg(test)]
mod test;

use vnsl_core::model::{
    VnslAction, VnslBlock, VnslChoices, VnslCommand, VnslCondition, VnslDialogue, VnslGlobal,
    VnslGoTo, VnslJump, VnslSetCharacter, VnslStatement,
};

use crate::{
    runtime_result::{RuntimeError, RuntimeResult},
    RunContext,
};

#[derive(Debug)]
pub struct BlockRunner {
    block: VnslBlock,
    current_stmt: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockCommand {
    DisplayText(VnslDialogue),
    SetCharacter(VnslSetCharacter),
    Action(VnslAction),
    Jump(VnslJump),
    Global(VnslGlobal),
    Choices(VnslChoices),
    ForkBlock(VnslBlock),
    ChangeScene(VnslGoTo),
    Return,
    NoOps,
}

impl BlockRunner {
    pub fn new(block: VnslBlock) -> Self {
        Self {
            block,
            current_stmt: 0,
        }
    }

    pub fn block_ended(&self) -> bool {
        self.block.statements.get(self.current_stmt).is_none()
    }

    pub fn step(&mut self, context: &mut RunContext) -> RuntimeResult<BlockCommand> {
        let Some(stmt) = self.block.statements.get(self.current_stmt) else {
            return Err(RuntimeError::EndOfStack);
        };
        let result = match stmt {
            VnslStatement::Command(vnsl_command) => Ok(exec_command(vnsl_command)),
            VnslStatement::Choices(vnsl_choices) => Ok(BlockCommand::Choices(vnsl_choices.clone())),
            VnslStatement::Condition(vnsl_condition) => exec_condition(vnsl_condition, context),
        };

        self.current_stmt += 1;

        result
    }
}

fn exec_condition(
    vnsl_condition: &VnslCondition,
    context: &RunContext,
) -> RuntimeResult<BlockCommand> {
    if context
        .lua_runtime
        .eval_expr(&vnsl_condition.if_block.condition.code)?
    {
        return Ok(BlockCommand::ForkBlock(
            vnsl_condition.if_block.block.clone(),
        ));
    }

    for block in &vnsl_condition.elif_block {
        if context.lua_runtime.eval_expr(&block.condition.code)? {
            return Ok(BlockCommand::ForkBlock(block.block.clone()));
        }
    }

    if let Some(else_block) = &vnsl_condition.else_block {
        return Ok(BlockCommand::ForkBlock(else_block.clone()));
    }

    Ok(BlockCommand::NoOps)
}

fn exec_command(cmd: &VnslCommand) -> BlockCommand {
    match cmd {
        VnslCommand::Dialogue(vnsl_dialogue) => BlockCommand::DisplayText(vnsl_dialogue.clone()),
        VnslCommand::SetCharacter(vnsl_set_character) => {
            BlockCommand::SetCharacter(vnsl_set_character.clone())
        }
        VnslCommand::Action(vnsl_action) => BlockCommand::Action(vnsl_action.clone()),
        VnslCommand::Jump(vnsl_jump) => BlockCommand::Jump(vnsl_jump.clone()),
        VnslCommand::Global(vnsl_global) => BlockCommand::Global(vnsl_global.clone()),
        VnslCommand::GoTo(vnsl_go_to) => BlockCommand::ChangeScene(vnsl_go_to.clone()),
        VnslCommand::Return => BlockCommand::Return,
    }
}
