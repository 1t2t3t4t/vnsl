use crate::{label, statement, Rule, VnslParser};
use anyhow::Ok;
use pest::Parser;
use thiserror::Error;
use vnsl_core::model::VnslScene;

#[derive(Debug, Clone, Copy, Error)]
pub enum ParseError {
    #[error("Parsing got empty rule")]
    EmptyRule,
    #[error("Parsing got invalid scene")]
    NoSceneName,
}

pub fn parse_scene_name(script: &str) -> anyhow::Result<String> {
    let mut scene = VnslParser::parse(Rule::scene, script)?;
    Ok(scene.next().ok_or(ParseError::NoSceneName)?.into_inner().as_str().to_string())
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
                let scene_name = rule.into_inner().next().unwrap().as_str().to_string();
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

#[cfg(test)]
mod tests {
    use crate::parser::parse_scene_name;

    #[test]
    fn test_parse_scene_name() {
        let res = parse_scene_name(r#"scene TestName"#);
        assert_eq!(&res.expect("successfully parse"), "TestName");
    }
}
