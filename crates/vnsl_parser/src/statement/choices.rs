use crate::{block, data_type, Rule};
use pest::iterators::Pair;
use uuid::Uuid;
use vnsl_core::model::{VnslChoice, VnslChoices};

fn gen_uuid() -> String {
    if cfg!(test) {
        "TestUuid".to_string()
    } else {
        Uuid::new_v4().to_string()
    }
}

pub fn parse_choices(rule: Pair<Rule>) -> anyhow::Result<VnslChoices> {
    let inner = rule.into_inner();
    let mut choices = vec![];
    for rule in inner {
        match rule.as_rule() {
            Rule::choice => {
                choices.push(parse_choice(rule)?);
            }
            _ => unreachable!(),
        }
    }
    Ok(VnslChoices { choices })
}

fn parse_choice(rule: Pair<Rule>) -> anyhow::Result<VnslChoice> {
    let inner = rule.into_inner();
    let mut choice = VnslChoice::default();
    choice.id = gen_uuid();
    for rule in inner {
        match rule.as_rule() {
            Rule::string => {
                choice.text = data_type::parse_string(rule);
            }
            Rule::block => {
                choice.block = block::parse_block(rule)?;
            }
            _ => unreachable!("Found unexpected rule {:?} for choice", rule.as_rule()),
        }
    }
    Ok(choice)
}
