use crate::{
    error::{unexpected_rule, wrap_parsing_result, ParsingResult},
    statement, Rule,
};
use pest::iterators::Pair;
use vnsl_core::model::VnslBlock;

pub fn parse_block(rule: Pair<Rule>) -> ParsingResult<VnslBlock> {
    let inner = rule.into_inner();
    let mut block = VnslBlock::default();
    for rule in inner {
        match rule.as_rule() {
            Rule::stmt => {
                let statement = wrap_parsing_result(rule, statement::parse_statement)?;
                block.statements.push(statement);
            }
            _ => {
                return unexpected_rule(&rule, &[Rule::stmt], "block");
            }
        }
    }
    Ok(block)
}
