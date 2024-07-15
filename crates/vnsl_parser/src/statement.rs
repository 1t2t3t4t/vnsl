use pest::iterators::Pair;

use crate::{
    model::{VnslCommand, VnslStatement},
    Rule,
};

pub fn parse_statements(rule: Pair<Rule>) -> Vec<VnslStatement> {
    let inner = rule.into_inner();
    let mut result = Vec::new();
    for rule in inner {
        match rule.as_rule() {
            Rule::stmt => {
                result.push(parse_statement(rule));
            }
            _ => unreachable!(),
        }
    }

    result
}

pub fn parse_statement(rule: Pair<Rule>) -> VnslStatement {
    let mut inner = rule.into_inner();
    assert_eq!(inner.len(), 1);
    let rule = inner.next().unwrap();
    match rule.as_rule() {
        Rule::command => VnslStatement::Command(VnslCommand::Dialogue {
            text: "test".to_string(),
        }),
        _ => unreachable!(),
    }
}
