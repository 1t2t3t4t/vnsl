use anyhow::{Ok, Result};
use pest::iterators::Pair;
use vnsl_core::model::{VnslLuaEvalExpr, VnslLuaEvalType, VnslLuaExpr};

use crate::{utils, Rule};

pub fn parse_lua_expr(rule: Pair<Rule>) -> Result<VnslLuaExpr> {
    let lua = utils::extract_inner_as_rule(rule, Rule::lua_script)
        .as_str()
        .to_string()
        .split("\n")
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>()
        .join("\n");
    Ok(VnslLuaExpr { lua })
}

pub fn parse_lua_eval_expr(
    rule: Pair<Rule>,
    eval_type: VnslLuaEvalType,
) -> Result<VnslLuaEvalExpr> {
    let code = utils::extract_inner_as_rule(rule, Rule::lua_lang)
        .as_str()
        .trim()
        .to_string();

    Ok(VnslLuaEvalExpr {
        code,
        return_type: eval_type,
    })
}
