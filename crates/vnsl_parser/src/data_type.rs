use pest::iterators::Pair;

use crate::Rule;

pub fn parse_string(rule: Pair<Rule>) -> String {
    rule.into_inner().next().unwrap().as_str().to_string()
}
