use action::parse_action;
use anyhow::Ok;
use dialogue::parse_dialogue;
use pest::iterators::Pair;

use crate::{model::VnslCommand, Rule};

pub fn parse_command(rule: Pair<Rule>) -> anyhow::Result<VnslCommand> {
    let inner = rule.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::dialogue => Ok(parse_dialogue(inner)),
        Rule::action => parse_action(inner),
        _ => unreachable!(),
    }
}

mod dialogue {
    use pest::iterators::Pair;

    use crate::{data_type, model::VnslCommand, Rule};

    pub fn parse_dialogue(rule: Pair<Rule>) -> VnslCommand {
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
}

mod action {
    use std::vec;

    use anyhow::Ok;
    use pest::iterators::Pair;

    use crate::{data_type, model::{VnslAction, VnslActionArg, VnslCommand, VnslDataType}, Rule};

    pub fn parse_action(rule: Pair<Rule>) -> anyhow::Result<VnslCommand> {
        let inner = rule.into_inner();
        let mut action = VnslAction::default();
        for rule in inner {
            match rule.as_rule() {
                Rule::identifier => {
                    action.name = rule.as_str().to_string()
                },
                Rule::args => {
                    action.args = parse_action_args(rule)?;
                },
                _ => unreachable!()
            }
        }
        Ok(VnslCommand::Action(action))
    }

    fn parse_action_args(rule: Pair<Rule>) -> anyhow::Result<Vec<VnslActionArg>> {
        let mut args = vec![];
        let inner = rule.into_inner();
        for rule in inner {
            args.push(parse_action_arg(rule)?);
        }
        Ok(args)
    }

    fn parse_action_arg(rule: Pair<Rule>) -> anyhow::Result<VnslActionArg> {
        let mut name = None;
        let mut arg_data_type = VnslDataType::Bool(false);

        let arg_rules = rule.into_inner();
        for a_rule in arg_rules {
            match a_rule.as_rule() {
                Rule::identifier => {
                    name = Some(a_rule.as_str().to_string());
                },
                Rule::data_type => {
                    arg_data_type = data_type::parse_data_type(a_rule)?;
                },
                _ => unreachable!("Unexpected rules {:?} in arg", a_rule.as_rule())
            }
        }

        Ok(VnslActionArg { name, data_type: arg_data_type })
    }
}