use crate::{data_type, error::ParsingResult, utils, Rule};
use pest::iterators::Pair;
use vnsl_core::model::VnslGlobal;

pub fn parse_global(rule: Pair<Rule>) -> ParsingResult<VnslGlobal> {
    let mut pairs = utils::extract_inners(rule, [Rule::identifier, Rule::data_type]);
    let name = pairs[&Rule::identifier].as_str().to_string();
    let value = data_type::parse_data_type(pairs.remove(&Rule::data_type).unwrap())?;
    Ok(VnslGlobal { name, value })
}
