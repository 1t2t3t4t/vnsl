use anyhow::Ok;
use model::VnslScene;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use thiserror::Error;

pub mod model;

mod statement;

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
    let rules = scene.into_inner();
    let mut scene = VnslScene::default();

    for rule in rules {
        match rule.as_rule() {
            Rule::scene => {
                let scene_name = parse_scene_name(rule);
                scene.name = scene_name;
            },
            Rule::main_statement => {
                let statements = statement::parse_statements(rule);
                scene.main_statements = statements;
            },
            Rule::labels => {

            },
            Rule::EOI => (),
            _ => unreachable!("Got unexpected rule {:?} in main loop", rule.as_rule())
        }
    }

    println!("{:#?}", scene);
    Ok(())
}

fn parse_scene_name(rule: Pair<Rule>) -> String {
    rule.into_inner()
        .next()
        .unwrap()
        .as_str()
        .to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parsing() {
        let example = include_str!("test.scene");
        super::parse(example).unwrap();
    }
}
