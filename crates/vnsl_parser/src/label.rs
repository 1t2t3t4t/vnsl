use crate::{
    block,
    error::{unexpected_rule, ParsingResult},
    Rule,
};
use pest::iterators::Pair;
use vnsl_core::model::VnslLabel;

pub fn parse_label(rule: Pair<Rule>) -> ParsingResult<VnslLabel> {
    let inner = rule.into_inner();
    let mut label = VnslLabel::default();
    for rule in inner {
        match rule.as_rule() {
            Rule::identifier => {
                label.name = rule.as_str().to_string();
            }
            Rule::block => {
                label.block = block::parse_block(rule)?;
            }
            _ => {
                return unexpected_rule(&rule, &[Rule::identifier, Rule::block], "label");
            }
        }
    }
    Ok(label)
}
