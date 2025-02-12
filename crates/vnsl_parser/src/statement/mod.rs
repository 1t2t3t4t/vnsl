use anyhow::Ok;
use pest::iterators::Pair;
use vnsl_core::model::VnslStatement;

pub mod choices;
pub mod command;
pub mod condition;
pub mod lua_expr;

use crate::Rule;

pub fn parse_statement(rule: Pair<Rule>) -> anyhow::Result<VnslStatement> {
    let mut inner = rule.into_inner();
    assert_eq!(inner.len(), 1);
    let rule = inner.next().unwrap();
    match rule.as_rule() {
        Rule::command => Ok(VnslStatement::Command(command::parse_command(rule)?)),
        Rule::choices => Ok(VnslStatement::Choices(choices::parse_choices(rule)?)),
        Rule::condition => Ok(VnslStatement::Condition(condition::parse_condition(rule)?)),
        Rule::lua_expr => Ok(VnslStatement::LuaExpr(lua_expr::parse_lua_expr(rule)?)),
        _ => unreachable!("Unexpedted rule {:?}", rule.as_rule()),
    }
}
