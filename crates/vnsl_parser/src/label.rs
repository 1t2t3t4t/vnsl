use anyhow::Ok;
use pest::iterators::Pair;

use crate::{block, model::VnslLabel, Rule};

pub fn parse_label(rule: Pair<Rule>) -> anyhow::Result<VnslLabel> {
    let inner = rule.into_inner();
    let mut label = VnslLabel::default();
    for rule in inner {
        match rule.as_rule() {
            Rule::identifier => {
                label.name = rule.as_str().to_string();
            }
            Rule::block => {
                label.block = block::parse_block(rule)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(label)
}
