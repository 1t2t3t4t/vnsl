use crate::{data_type, Rule};
use pest::iterators::Pair;
use thiserror::Error;
use vnsl_core::model::{VnslDialogue, VnslSetCharacter};

#[derive(Debug, Error)]
pub enum ParseDialogueError {
    #[error("Missing string text from dialogue.")]
    MissingStringText,
}

pub fn parse_dialogue(rule: Pair<Rule>) -> anyhow::Result<VnslDialogue> {
    let inners = rule.into_inner();
    let mut set_char = None;
    let mut text = None;

    for inner in inners {
        match inner.as_rule() {
            Rule::string => {
                text = Some(data_type::parse_string(inner));
            }
            Rule::char_identifier => {
                println!("{:#?}", inner);
                set_char = Some(VnslSetCharacter {
                    id: inner.as_str().to_string(),
                })
            }
            _ => unreachable!("Unsupported rule"),
        }
    }

    Ok(VnslDialogue {
        text: text.ok_or(ParseDialogueError::MissingStringText)?,
        set_char,
    })
}
