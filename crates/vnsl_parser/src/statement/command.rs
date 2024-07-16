use anyhow::Ok;
use pest::iterators::Pair;

use crate::{model::VnslCommand, Rule};

mod action;
mod dialogue;
mod jump;

pub fn parse_command(rule: Pair<Rule>) -> anyhow::Result<VnslCommand> {
    let inner = rule.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::dialogue => Ok(dialogue::parse_dialogue(inner)),
        Rule::action => action::parse_action(inner),
        Rule::jump => jump::parse_jump(inner),
        _ => unreachable!(),
    }
}
