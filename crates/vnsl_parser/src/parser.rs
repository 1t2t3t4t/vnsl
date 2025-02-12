use crate::{label, statement, Rule, VnslParser};
use anyhow::Ok;
use pest::{iterators::Pair, Parser};
use thiserror::Error;
use vnsl_core::model::VnslScene;

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
            Rule::stmt => {
                let statement = statement::parse_statement(rule)?;
                scene.main_block.statements.push(statement);
            }
            Rule::label_scope => {
                let label = label::parse_label(rule)?;
                scene.labels.insert(label.name.clone(), label);
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
