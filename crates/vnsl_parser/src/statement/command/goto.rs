use pest::iterators::Pair;
use vnsl_core::model::VnslGoTo;

use crate::{error::ParsingResult, utils::extract_inner_as_rule, Rule};

pub fn parse_goto(rule: Pair<Rule>) -> ParsingResult<VnslGoTo> {
    let inner = extract_inner_as_rule(rule, Rule::identifier);
    Ok(VnslGoTo {
        scene_id: inner.as_str().to_string(),
    })
}
