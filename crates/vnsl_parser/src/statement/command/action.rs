use std::vec;

use anyhow::Ok;
use pest::iterators::Pair;

use crate::{
    data_type,
    model::{VnslAction, VnslActionArg, VnslDataType},
    Rule,
};

pub fn parse_action(rule: Pair<Rule>) -> anyhow::Result<VnslAction> {
    let inner = rule.into_inner();
    let mut action = VnslAction::default();
    for rule in inner {
        match rule.as_rule() {
            Rule::identifier => action.name = rule.as_str().to_string(),
            Rule::args => {
                action.args = parse_action_args(rule)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(action)
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
            }
            Rule::data_type => {
                arg_data_type = data_type::parse_data_type(a_rule)?;
            }
            _ => unreachable!("Unexpected rules {:?} in arg", a_rule.as_rule()),
        }
    }

    Ok(VnslActionArg {
        name,
        data_type: arg_data_type,
    })
}
