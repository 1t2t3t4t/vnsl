use pest::iterators::Pair;

use crate::{data_type, model::VnslDialogue, utils, Rule};

pub fn parse_dialogue(rule: Pair<Rule>) -> anyhow::Result<VnslDialogue> {
    let inner = utils::extract_inner(rule, Rule::string);
    let text = data_type::parse_string(inner);
    Ok(VnslDialogue { text })
}
