use vnsl_core::model::{VnslBlock, VnslCommand, VnslStatement};

use crate::RunContext;

#[derive(Debug)]
pub struct BlockRunner {
    block: VnslBlock,
    current_stmt: usize,
}

pub enum BlockCommand {
    EndOfStack,
}

impl BlockRunner {
    pub fn new(block: VnslBlock) -> Self {
        Self {
            block,
            current_stmt: 0,
        }
    }

    pub fn step(&mut self, context: &mut RunContext) -> BlockCommand {
        let Some(stmt) = self.block.statements.get(self.current_stmt) else {
            return BlockCommand::EndOfStack;
        };
        let result = match stmt {
            VnslStatement::Command(vnsl_command) => exec_command(vnsl_command),
            VnslStatement::Choices(vnsl_choices) => BlockCommand::EndOfStack,
            VnslStatement::Condition(vnsl_condition) => BlockCommand::EndOfStack,
            VnslStatement::Expr(_) => unimplemented!("Expr is not supported yet"),
        };

        self.current_stmt += 1;

        result
    }
}

fn exec_command(cmd: &VnslCommand) -> BlockCommand {
    match cmd {
        VnslCommand::Dialogue(vnsl_dialogue) => todo!(),
        VnslCommand::SetCharacter(vnsl_set_character) => todo!(),
        VnslCommand::Action(vnsl_action) => todo!(),
        VnslCommand::Jump(vnsl_jump) => todo!(),
        VnslCommand::Global(vnsl_global) => todo!(),
    }
}
