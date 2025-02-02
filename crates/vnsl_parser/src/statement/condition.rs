use anyhow::Ok;
use pest::iterators::Pair;
use thiserror::Error;
use vnsl_core::model::{
    VnslBlock, VnslCondition, VnslConditionBlock, VnslLuaEvalExpr, VnslLuaEvalType,
};

use crate::{block::parse_block, utils, Rule};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ParseConditionError {
    #[error("If condition is missing")]
    MissingIfCondition,

    #[error("Extra else condition")]
    ExtraElseCondition,
}

pub fn parse_condition(rule: Pair<Rule>) -> anyhow::Result<VnslCondition> {
    let inner = rule.into_inner();
    let mut if_cond: Option<VnslConditionBlock> = None;
    let mut elif_cond: Vec<VnslConditionBlock> = vec![];
    let mut else_cond: Option<VnslBlock> = None;
    for inner_rule in inner {
        match inner_rule.as_rule() {
            Rule::if_block => {
                if_cond = Some(parse_condition_iden(inner_rule)?);
            }
            Rule::elif_block => {
                let cond = parse_condition_iden(inner_rule)?;
                elif_cond.push(cond);
            }
            Rule::else_block => {
                if else_cond.is_some() {
                    return Err(ParseConditionError::ExtraElseCondition.into());
                }
                let block = parse_block(utils::extract_inner(inner_rule))?;
                else_cond = Some(block);
            }
            _ => unreachable!(),
        }
    }
    let if_cond = if_cond.ok_or(ParseConditionError::MissingIfCondition)?;
    Ok(VnslCondition {
        if_block: if_cond,
        elif_block: elif_cond,
        else_block: else_cond,
    })
}

fn parse_condition_iden(rule: Pair<Rule>) -> anyhow::Result<VnslConditionBlock> {
    let inners = utils::extract_inners(rule, [Rule::lua_lang, Rule::block]);
    let code = inners
        .get(&Rule::lua_lang)
        .unwrap()
        .as_str()
        .trim()
        .to_string();
    let block = parse_block(inners.get(&Rule::block).unwrap().clone())?;
    Ok(VnslConditionBlock {
        condition: VnslLuaEvalExpr {
            code,
            return_type: VnslLuaEvalType::Bool,
        },
        block,
    })
}
