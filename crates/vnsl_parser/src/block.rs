use anyhow::Ok;
use pest::iterators::Pair;

use crate::{model::VnslBlock, statement, Rule};

pub fn parse_block(rule: Pair<Rule>) -> anyhow::Result<VnslBlock> {
    let inner = rule.into_inner();
    let mut block = VnslBlock::default();
    for rule in inner {
        match rule.as_rule() {
            Rule::stmt => {
                let statement = statement::parse_statement(rule)?;
                block.statements.push(statement);
            }
            _ => unreachable!(),
        }
    }
    Ok(block)
}
