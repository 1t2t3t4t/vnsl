use crate::{data_type, utils, Rule};
use pest::iterators::Pair;
use vnsl_core::model::VnslDialogue;

pub fn parse_dialogue(rule: Pair<Rule>) -> anyhow::Result<VnslDialogue> {
    let inner = utils::extract_inner_as_rule(rule, Rule::string);
    let text = data_type::parse_string(inner);
    Ok(VnslDialogue { text })
}
