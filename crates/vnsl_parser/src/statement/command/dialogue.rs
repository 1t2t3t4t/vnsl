use crate::{
    data_type,
    error::{unexpected_rule, ParsingError, ParsingErrorKind, ParsingResult},
    Rule,
};
use pest::iterators::Pair;
use vnsl_core::model::{VnslDialogue, VnslSetCharacter};

pub fn parse_dialogue(rule: Pair<Rule>) -> ParsingResult<VnslDialogue> {
    let rule_clone = rule.clone();
    let inners = rule.into_inner();
    let mut set_char = None;
    let mut text = None;

    for inner in inners {
        match inner.as_rule() {
            Rule::string => {
                text = Some(data_type::parse_string(inner));
            }
            Rule::char_identifier => {
                set_char = Some(VnslSetCharacter {
                    id: inner.as_str().to_string(),
                })
            }
            _ => {
                return unexpected_rule(&inner, &[Rule::string, Rule::char_identifier], "dialogue");
            }
        }
    }

    Ok(VnslDialogue {
        text: text.ok_or_else(|| {
            ParsingError::new(
                &rule_clone,
                ParsingErrorKind::MissingRequired("dialogue text string".to_string()),
            )
        })?,
        set_char,
    })
}
