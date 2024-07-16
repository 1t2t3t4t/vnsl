use pest::iterators::Pair;

use crate::{model::VnslCommand, Rule};

pub fn parse_jump(rule: Pair<Rule>) -> anyhow::Result<VnslCommand> {
    let mut inner = rule.into_inner();
    assert_eq!(inner.len(), 1);
    let iden = inner.next().unwrap().as_str().to_string();
    Ok(VnslCommand::Jump { to_label: iden })
}
