use pest::iterators::Pair;

use crate::{model::VnslSetCharacter, utils, Rule};

pub fn parse_set_character(rule: Pair<Rule>) -> anyhow::Result<VnslSetCharacter> {
    let id = utils::extract_inner(rule, Rule::identifier)
        .as_str()
        .to_string();
    Ok(VnslSetCharacter { id })
}
