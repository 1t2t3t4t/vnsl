use pest::iterators::Pair;
use vnsl_core::model::{
    VnslBlock, VnslCondition, VnslConditionBlock, VnslLuaEvalExpr, VnslLuaEvalType,
};

use crate::{
    block::parse_block,
    error::{unexpected_rule, ParsingError, ParsingErrorKind, ParsingResult},
    utils, Rule,
};

pub fn parse_condition(rule: Pair<Rule>) -> ParsingResult<VnslCondition> {
    let inner = rule.clone().into_inner();
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
                    return Err(ParsingError::new(
                        &inner_rule,
                        ParsingErrorKind::ExtraElement("else block".to_string()),
                    ));
                }
                let block = parse_block(utils::extract_inner(inner_rule))?;
                else_cond = Some(block);
            }
            _ => {
                return unexpected_rule(
                    &inner_rule,
                    &[Rule::if_block, Rule::elif_block, Rule::else_block],
                    "condition",
                );
            }
        }
    }
    let if_cond =
        if_cond.ok_or_else(|| ParsingError::missing_required(&rule, "if condition block"))?;
    Ok(VnslCondition {
        if_block: if_cond,
        elif_block: elif_cond,
        else_block: else_cond,
    })
}

fn parse_condition_iden(rule: Pair<Rule>) -> ParsingResult<VnslConditionBlock> {
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
