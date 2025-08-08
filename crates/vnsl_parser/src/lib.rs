mod block;
mod data_type;
mod error;
mod label;
mod statement;
mod utils;

use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;
use vnsl_core::model::VnslScene;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct VnslParser;

#[derive(Debug, Clone, Error)]
pub enum ParseError {
    #[error("Parsing got empty rule")]
    EmptyRule,
    #[error("Parsing got invalid scene")]
    NoSceneName,
    #[error("Unsupported rule {0:?} in line: {1}")]
    UnsupportedRule(Rule, String),
}

pub fn parse_scene_name(script: &str) -> anyhow::Result<String> {
    let mut scene = VnslParser::parse(Rule::scene, script)?;
    anyhow::Ok(
        scene
            .next()
            .ok_or(ParseError::NoSceneName)?
            .into_inner()
            .as_str()
            .to_string(),
    )
}

pub fn parse_scene(script: &str) -> anyhow::Result<VnslScene> {
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

    anyhow::Ok(scene)
}

#[cfg(test)]
mod tests {
    use crate::parse_scene_name;
    use pretty_assertions::assert_str_eq;
    use std::fs;

    const FORCE_RECORD: bool = false;

    #[test]
    fn test_parse_scene_name() {
        let res = parse_scene_name(r#"scene TestName"#);
        assert_eq!(&res.expect("successfully parse"), "TestName");
    }

    #[test]
    fn test_parsing_snapshot() {
        let dir = fs::read_dir("./snapshot").unwrap();
        let mut results = vec![];
        for entry in dir {
            let os_name = entry.unwrap().file_name();
            let file_name = os_name.to_str().unwrap();
            if file_name.ends_with(".vnsl") {
                let spec_name = file_name.trim_end_matches(".vnsl");
                results.push(snapshot(spec_name));
            }
        }

        assert!(results.iter().all(|b| b == &true))
    }

    fn snapshot(name: &str) -> bool {
        println!("Testing {name}");

        let script = fs::read_to_string(format!("./snapshot/{}.vnsl", name)).unwrap();

        let scn_str = match super::parse_scene(&script) {
            Ok(result) => format!("{:#?}", result).replace("\r\n", "\n"),
            Err(err) => format!("{err:?}").trim().to_string(),
        };

        let expect_path = format!("./snapshot/{}.result", name);
        if fs::metadata(&expect_path).is_ok() && !FORCE_RECORD {
            let expect = fs::read_to_string(expect_path)
                .unwrap()
                .replace("\r\n", "\n");
            let expect = expect.trim();
            assert_str_eq!(expect, scn_str);
            true
        } else {
            fs::write(expect_path, scn_str).unwrap();
            println!("Recording snapshot for {}", name);
            false
        }
    }
}
