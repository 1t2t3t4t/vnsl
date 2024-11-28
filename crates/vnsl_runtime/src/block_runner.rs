use vnsl_core::model::{
    VnslAction, VnslBlock, VnslChoices, VnslCommand, VnslCondition, VnslDialogue, VnslGlobal,
    VnslJump, VnslSetCharacter, VnslStatement,
};

use crate::{runtime_result::RuntimeResult, RunContext, RuntimeDelegateHandler};

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
    EndOfStack,
    None,
}

impl BlockRunner {
    pub fn new(block: VnslBlock) -> Self {
        Self {
            block,
            current_stmt: 0,
        }
    }

    pub fn step(
        &mut self,
        context: &mut RunContext,
        _delegate_handler: &impl RuntimeDelegateHandler,
    ) -> RuntimeResult<BlockCommand> {
        let Some(stmt) = self.block.statements.get(self.current_stmt) else {
            return Ok(BlockCommand::EndOfStack);
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

    Ok(BlockCommand::None)
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
    }
}

// #[cfg(test)]
// mod test {
//     use vnsl_core::model::{
//         VnslBlock, VnslCondition, VnslConditionBlock, VnslDialogue, VnslLuaEvalExpr, VnslStatement
//     };

//     use crate::{block_runner::BlockCommand, RunContext, RuntimeDelegateHandler};

//     use super::exec_condition;

//     struct MockRuntimeDelegateHandler {
//         true_cond_id: String,
//     }

//     impl RuntimeDelegateHandler for MockRuntimeDelegateHandler {
//         fn check_condition(&self, condition_id: &str, _context: &crate::RunContext) -> bool {
//             self.true_cond_id == condition_id
//         }
//     }

//     fn create_block(label: &str) -> VnslBlock {
//         VnslBlock {
//             statements: vec![VnslStatement::Command(
//                 vnsl_core::model::VnslCommand::Dialogue(VnslDialogue {
//                     text: label.to_string(),
//                 }),
//             )],
//         }
//     }

//     fn setup_condition(else_block: Option<VnslBlock>) -> VnslCondition {
//         VnslCondition {
//             if_block: VnslConditionBlock {
//                 condition: VnslLuaEvalExpr {
//                     code: todo!(),
//                     return_type: vnsl_core::model::VnslLuaEvalType::Bool,
//                 },
//                 block: create_block("if block"),
//             },
//             elif_block: vec![
//                 VnslConditionBlock {
//                     iden: "elif_cond1".to_string(),
//                     block: create_block("elif block1"),
//                 },
//                 VnslConditionBlock {
//                     iden: "elif_cond2".to_string(),
//                     block: create_block("elif block2"),
//                 },
//             ],
//             else_block,
//         }
//     }

//     #[test]
//     fn test_condition_if() {
//         let condition = setup_condition(None);
//         let context = RunContext::default();
//         let handler = MockRuntimeDelegateHandler {
//             true_cond_id: "if_cond".to_string(),
//         };

//         assert_eq!(
//             exec_condition(&condition, &context, &handler),
//             BlockCommand::ForkBlock(create_block("if block"))
//         );
//     }

//     #[test]
//     fn test_condition_elif() {
//         let condition = setup_condition(None);
//         let context = RunContext::default();
//         let mut handler = MockRuntimeDelegateHandler {
//             true_cond_id: "elif_cond1".to_string(),
//         };

//         assert_eq!(
//             exec_condition(&condition, &context, &handler),
//             BlockCommand::ForkBlock(create_block("elif block1"))
//         );

//         handler.true_cond_id = "elif_cond2".to_string();
//         assert_eq!(
//             exec_condition(&condition, &context, &handler),
//             BlockCommand::ForkBlock(create_block("elif block2"))
//         );
//     }

//     #[test]
//     fn test_condition_else() {
//         let condition = setup_condition(Some(create_block("else block")));
//         let context = RunContext::default();
//         let handler = MockRuntimeDelegateHandler {
//             true_cond_id: "none".to_string(),
//         };

//         assert_eq!(
//             exec_condition(&condition, &context, &handler),
//             BlockCommand::ForkBlock(create_block("else block"))
//         );
//     }

//     #[test]
//     fn test_condition_none() {
//         let condition = setup_condition(None);
//         let context = RunContext::default();
//         let handler = MockRuntimeDelegateHandler {
//             true_cond_id: "none".to_string(),
//         };

//         assert_eq!(
//             exec_condition(&condition, &context, &handler),
//             BlockCommand::None
//         );
//     }
// }
