use anyhow::Ok;
use pest::iterators::Pair;
use vnsl_core::model::VnslDataType;
use crate::Rule;

pub fn parse_data_type(rule: Pair<Rule>) -> anyhow::Result<VnslDataType> {
    let mut inner = rule.into_inner();
    assert_eq!(inner.len(), 1);
    let inner = inner.next().unwrap();
    match inner.as_rule() {
        Rule::string => Ok(VnslDataType::String(parse_string(inner))),
        Rule::number => {
            let num = inner.as_str().parse::<f64>()?;
            Ok(VnslDataType::Number(num))
        }
        Rule::bool => {
            let bool_val = inner.as_str() == "true";
            Ok(VnslDataType::Bool(bool_val))
        }
        _ => unreachable!(),
    }
}

pub fn parse_string(rule: Pair<Rule>) -> String {
    rule.into_inner()
        .map(|r| r.as_str().to_string())
        .collect::<Vec<String>>()
        .join("\n")
        .trim()
        .to_string()
}
