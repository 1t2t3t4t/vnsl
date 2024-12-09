use anyhow::Ok;
use pest::iterators::Pair;
use vnsl_core::model::VnslGoTo;

use crate::{utils::extract_inner_as_rule, Rule};

pub fn parse_goto(rule: Pair<Rule>) -> anyhow::Result<VnslGoTo> {
    let inner = extract_inner_as_rule(rule, Rule::identifier);
    Ok(VnslGoTo {
        scene_id: inner.as_str().to_string(),
    })
}
