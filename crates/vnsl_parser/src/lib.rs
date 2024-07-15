use anyhow::Ok;
use model::VnslScene;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use thiserror::Error;

pub mod model;

mod block;
mod command;
mod data_type;
mod label;
mod statement;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct VnslParser;

#[derive(Debug, Clone, Copy, Error)]
pub enum ParseError {
    #[error("Parsing got empty rule")]
    EmptyRule,
}

pub fn parse(script: &str) -> anyhow::Result<VnslScene> {
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
            }
            Rule::main_statement => {
                let statements = statement::parse_statements(rule)?;
                scene.main_statements = statements;
            }
            Rule::labels => {
                let label = label::parse_label(rule)?;
                scene.labels.push(label);
            }
            Rule::EOI => (),
            _ => unreachable!("Got unexpected rule {:?} in main loop", rule.as_rule()),
        }
    }

    Ok(scene)
}

fn parse_scene_name(rule: Pair<Rule>) -> String {
    rule.into_inner().next().unwrap().as_str().to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_parsing() {
        snapshot("spec");
    }

    fn snapshot(name: &str) {
        let script = fs::read_to_string(format!("./snapshot/{}.vnsl", name)).unwrap();
        let result = super::parse(&script).unwrap();
        
        let scn_str = format!("{:#?}", result);
        
        let expect_path = format!("./snapshot/{}.result", name);
        if fs::metadata(&expect_path).is_ok() {
            let expect = fs::read_to_string(expect_path).unwrap();
            assert_eq!(expect, scn_str);
        } else {
            fs::write(expect_path, scn_str).unwrap();
            assert!(false, "Recording snapshot for {}", name);
        }
    }
}
