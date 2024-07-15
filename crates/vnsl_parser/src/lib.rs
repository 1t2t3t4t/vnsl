use anyhow::Ok;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct VnslParser;

#[derive(Debug, Clone, Copy, Error)]
pub enum ParseError {
    #[error("Parsing got empty rule")]
    EmptyRule,
}

pub fn parse(script: &str) -> anyhow::Result<()> {
    let scene = VnslParser::parse(Rule::script, script)?
        .next()
        .ok_or(ParseError::EmptyRule)?;
    for i in scene.into_inner() {
        println!("{:?}", i.as_rule())
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parsing() {
        let example = include_str!("test.scene");
        super::parse(example).unwrap();
    }
}
