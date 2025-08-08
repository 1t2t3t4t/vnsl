use vnsl_core::model::{
    VnslBlock, VnslCondition, VnslConditionBlock, VnslDialogue, VnslLuaEvalExpr, VnslLuaEvalType,
    VnslStatement,
};

use crate::{block_runner::BlockCommand, RunContext};

use super::{exec_condition, BlockRunner};

fn create_block(label: &str) -> VnslBlock {
    VnslBlock {
        statements: vec![VnslStatement::Command(
            vnsl_core::model::VnslCommand::Dialogue(VnslDialogue {
                text: label.to_string(),
                set_char: None,
            }),
        )],
    }
}

fn create_expr(code: &str) -> VnslLuaEvalExpr {
    VnslLuaEvalExpr {
        code: code.to_string(),
        return_type: VnslLuaEvalType::Bool,
    }
}

#[test]
fn test_block_end() {
    let mut context = RunContext::default();
    let block = create_block("Test");
    let mut runner = BlockRunner::new(block);

    assert_eq!(runner.block_ended(), false);
    assert_eq!(
        runner.step(&mut context),
        Ok(BlockCommand::DisplayText(VnslDialogue {
            text: "Test".to_string(),
            set_char: None
        }))
    );
    assert_eq!(runner.block_ended(), true);
    assert_eq!(runner.step(&mut context), Ok(BlockCommand::NoOps));
}

#[test]
fn test_condition_if() {
    let context = RunContext::default();
    let condition = VnslCondition {
        if_block: VnslConditionBlock {
            condition: create_expr("global.cond == 1"),
            block: create_block("if block"),
        },
        elif_block: vec![],
        else_block: None,
    };

    context.lua_runtime.set_globals_val("cond", 1).unwrap();
    assert_eq!(
        exec_condition(&condition, &context).unwrap(),
        BlockCommand::ForkBlock(create_block("if block"))
    );
}

#[test]
fn test_condition_elif() {
    let context = RunContext::default();
    let condition = VnslCondition {
        if_block: VnslConditionBlock {
            condition: create_expr("global.cond == 1"),
            block: create_block("if block"),
        },
        elif_block: vec![
            VnslConditionBlock {
                condition: create_expr("global.cond == 2"),
                block: create_block("elif block"),
            },
            VnslConditionBlock {
                condition: create_expr("global.cond == 3"),
                block: create_block("elif block 2"),
            },
        ],
        else_block: None,
    };

    context.lua_runtime.set_globals_val("cond", 2).unwrap();
    assert_eq!(
        exec_condition(&condition, &context).unwrap(),
        BlockCommand::ForkBlock(create_block("elif block"))
    );

    context.lua_runtime.set_globals_val("cond", 3).unwrap();
    assert_eq!(
        exec_condition(&condition, &context).unwrap(),
        BlockCommand::ForkBlock(create_block("elif block 2"))
    );
}

#[test]
fn test_condition_else() {
    let context = RunContext::default();
    context.lua_runtime.set_globals_val("cond", 300).unwrap();
    let condition = VnslCondition {
        if_block: VnslConditionBlock {
            condition: create_expr("global.cond == 1"),
            block: create_block("if block"),
        },
        elif_block: vec![VnslConditionBlock {
            condition: create_expr("global.cond == 2"),
            block: create_block("elif block"),
        }],
        else_block: Some(create_block("else")),
    };

    assert_eq!(
        exec_condition(&condition, &context).unwrap(),
        BlockCommand::ForkBlock(create_block("else"))
    );
}

#[test]
fn test_condition_none() {
    let context = RunContext::default();
    context.lua_runtime.set_globals_val("cond", 300).unwrap();
    let condition = VnslCondition {
        if_block: VnslConditionBlock {
            condition: create_expr("cond == 1"),
            block: create_block("if block"),
        },
        elif_block: vec![VnslConditionBlock {
            condition: create_expr("cond == 2"),
            block: create_block("elif block"),
        }],
        else_block: None,
    };

    assert_eq!(
        exec_condition(&condition, &context).unwrap(),
        BlockCommand::NoOps
    );
}
