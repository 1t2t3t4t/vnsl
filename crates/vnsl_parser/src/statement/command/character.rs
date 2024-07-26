use pest::iterators::Pair;
use vnsl_core::model::VnslSetCharacter;
use crate::{Rule, utils};

pub fn parse_set_character(rule: Pair<Rule>) -> anyhow::Result<VnslSetCharacter> {
    let id = utils::extract_inner(rule, Rule::identifier)
        .as_str()
        .to_string();
    Ok(VnslSetCharacter { id })
}
