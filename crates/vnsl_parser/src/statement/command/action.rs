use std::vec;

use crate::{
    data_type,
    error::{unexpected_rule, ParsingResult},
    Rule,
};
use pest::iterators::Pair;
use vnsl_core::model::{VnslAction, VnslActionArg, VnslDataType};

pub fn parse_action(rule: Pair<Rule>) -> ParsingResult<VnslAction> {
    let inner = rule.into_inner();
    let mut action = VnslAction::default();
    for rule in inner {
        match rule.as_rule() {
            Rule::identifier => action.name = rule.as_str().to_string(),
            Rule::args => {
                action.args = parse_action_args(rule)?;
            }
            _ => {
                return unexpected_rule(&rule, &[Rule::identifier, Rule::args], "action");
            }
        }
    }
    Ok(action)
}

fn parse_action_args(rule: Pair<Rule>) -> ParsingResult<Vec<VnslActionArg>> {
    let mut args = vec![];
    let inner = rule.into_inner();
    for rule in inner {
        args.push(parse_action_arg(rule)?);
    }
    Ok(args)
}

fn parse_action_arg(rule: Pair<Rule>) -> ParsingResult<VnslActionArg> {
    let mut name = None;
    let mut arg_data_type = VnslDataType::Bool(false);

    let arg_rules = rule.into_inner();
    for a_rule in arg_rules {
        match a_rule.as_rule() {
            Rule::identifier => {
                name = Some(a_rule.as_str().to_string());
            }
            Rule::data_type => {
                arg_data_type = data_type::parse_data_type(a_rule)?;
            }
            _ => {
                return unexpected_rule(
                    &a_rule,
                    &[Rule::identifier, Rule::data_type],
                    "action argument",
                );
            }
        }
    }

    Ok(VnslActionArg {
        name,
        data_type: arg_data_type,
    })
}
