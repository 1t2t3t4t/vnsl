use crate::error::{unexpected_rule, ParsingResult};
use crate::Rule;
use pest::iterators::Pair;
use vnsl_core::model::VnslCommand;

mod action;
mod character;
mod dialogue;
mod global;
mod goto;
mod jump;

pub fn parse_command(rule: Pair<Rule>) -> ParsingResult<VnslCommand> {
    let inner = rule.into_inner().next().unwrap();
    let cmd = match inner.as_rule() {
        Rule::dialogue => VnslCommand::Dialogue(dialogue::parse_dialogue(inner)?),
        Rule::set_character => VnslCommand::SetCharacter(character::parse_set_character(inner)?),
        Rule::action => VnslCommand::Action(action::parse_action(inner)?),
        Rule::jump => VnslCommand::Jump(jump::parse_jump(inner)?),
        Rule::global => VnslCommand::Global(global::parse_global(inner)?),
        Rule::goto => VnslCommand::GoTo(goto::parse_goto(inner)?),
        Rule::r#return => VnslCommand::Return,
        Rule::pass => VnslCommand::Pass,
        _ => {
            return unexpected_rule(
                &inner,
                &[
                    Rule::dialogue,
                    Rule::set_character,
                    Rule::action,
                    Rule::jump,
                    Rule::global,
                    Rule::goto,
                    Rule::r#return,
                    Rule::pass,
                ],
                "command",
            );
        }
    };
    Ok(cmd)
}
