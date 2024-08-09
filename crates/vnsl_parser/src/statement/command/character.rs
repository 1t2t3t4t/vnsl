use crate::{utils, Rule};
use pest::iterators::Pair;
use vnsl_core::model::VnslSetCharacter;

pub fn parse_set_character(rule: Pair<Rule>) -> anyhow::Result<VnslSetCharacter> {
    let id = utils::extract_inner_as_rule(rule, Rule::identifier)
        .as_str()
        .to_string();
    Ok(VnslSetCharacter { id })
}
