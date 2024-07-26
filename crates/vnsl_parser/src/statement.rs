use anyhow::Ok;
use pest::iterators::Pair;
use vnsl_core::model::VnslStatement;

pub mod choices;
pub mod command;

use crate::Rule;

pub fn parse_statements(rule: Pair<Rule>) -> anyhow::Result<Vec<VnslStatement>> {
    let inner = rule.into_inner();
    let mut result = Vec::new();
    for rule in inner {
        match rule.as_rule() {
            Rule::stmt => {
                result.push(parse_statement(rule)?);
            }
            _ => unreachable!(),
        }
    }

    Ok(result)
}

pub fn parse_statement(rule: Pair<Rule>) -> anyhow::Result<VnslStatement> {
    let mut inner = rule.into_inner();
    assert_eq!(inner.len(), 1);
    let rule = inner.next().unwrap();
    match rule.as_rule() {
        Rule::command => Ok(VnslStatement::Command(command::parse_command(rule)?)),
        Rule::choices => Ok(VnslStatement::Choices(choices::parse_choices(rule)?)),
        _ => unreachable!(),
    }
}
