use pest::iterators::Pair;

use crate::{data_type, model::VnslCommand, Rule};

pub fn parse_command(rule: Pair<Rule>) -> VnslCommand {
    let inner = rule.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::dialogue => parse_dialogue(inner),
        _ => VnslCommand::None,
    }
}

fn parse_dialogue(rule: Pair<Rule>) -> VnslCommand {
    let mut inner = rule.into_inner();
    assert_eq!(inner.len(), 1);
    let rule = inner.next().unwrap();
    match rule.as_rule() {
        Rule::string => {
            let text = data_type::parse_string(rule);
            VnslCommand::Dialogue { text }
        }
        _ => unreachable!(),
    }
}
