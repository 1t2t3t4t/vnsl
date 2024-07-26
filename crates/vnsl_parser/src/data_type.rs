use crate::Rule;
use anyhow::{Error, Ok};
use pest::iterators::Pair;
use thiserror::Error;
use vnsl_core::model::VnslDataType;

#[derive(Debug, Error)]
pub enum ParseDataTypeError {
    #[error("Cannot parse number {0} with error {1}")]
    CannotParseNumber(String, Error),
}

pub fn parse_data_type(rule: Pair<Rule>) -> anyhow::Result<VnslDataType> {
    let mut inner = rule.into_inner();
    assert_eq!(inner.len(), 1);
    let inner = inner.next().unwrap();
    match inner.as_rule() {
        Rule::string => Ok(VnslDataType::String(parse_string(inner))),
        Rule::number => {
            let num_str = inner.as_str();
            let num = match num_str.trim().parse::<f64>() {
                std::result::Result::Ok(double) => Ok(double),
                Err(err) => Err(ParseDataTypeError::CannotParseNumber(
                    num_str.to_string(),
                    err.into(),
                )
                .into()),
            };
            Ok(VnslDataType::Number(num?))
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
