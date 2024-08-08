use crate::Rule;
use anyhow::Ok;
use pest::iterators::Pair;
use vnsl_core::model::VnslCommand;

mod action;
mod character;
mod dialogue;
mod global;
mod jump;

pub fn parse_command(rule: Pair<Rule>) -> anyhow::Result<VnslCommand> {
    let inner = rule.into_inner().next().unwrap();
    let cmd = match inner.as_rule() {
        Rule::dialogue => VnslCommand::Dialogue(dialogue::parse_dialogue(inner)?),
        Rule::set_character => VnslCommand::SetCharacter(character::parse_set_character(inner)?),
        Rule::action => VnslCommand::Action(action::parse_action(inner)?),
        Rule::jump => VnslCommand::Jump(jump::parse_jump(inner)?),
        Rule::global => VnslCommand::Global(global::parse_global(inner)?),
        _ => unreachable!(),
    };
    Ok(cmd)
}
