use pest::iterators::Pair;

use crate::{model::VnslJump, Rule};

pub fn parse_jump(rule: Pair<Rule>) -> anyhow::Result<VnslJump> {
    let mut inner = rule.into_inner();
    assert_eq!(inner.len(), 1);
    let iden = inner.next().unwrap().as_str().to_string();
    Ok(VnslJump { to_label: iden })
}
