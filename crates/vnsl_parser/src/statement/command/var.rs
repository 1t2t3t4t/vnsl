use pest::iterators::Pair;

use crate::{data_type, model::VnslVar, utils, Rule};

pub fn parse_var(rule: Pair<Rule>) -> anyhow::Result<VnslVar> {
    let mut pairs = utils::extract_inners(rule, [Rule::identifier, Rule::data_type]);
    let name = pairs[&Rule::identifier].as_str().to_string();
    let value = data_type::parse_data_type(pairs.remove(&Rule::data_type).unwrap())?;
    Ok(VnslVar { name, value })
}
