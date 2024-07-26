use pest::iterators::Pair;
use thiserror::Error;

use crate::Rule;

macro_rules! impl_parsing_error {
    ($($key:ident: $type:ty: $tag:expr),+) => {
        #[derive(Debug, Error)]
        pub struct ParsingError {
            $(pub $key: $type,)+
            source: anyhow::Error,
        }

        impl std::fmt::Display for ParsingError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "Parsing failed:")?;
                $(
                    writeln!(f, "{}: {:?}", $tag, self.$key)?;
                )+
                writeln!(f, "Source Error:")?;
                writeln!(f, "msg: {}", self.source)?;
                write!(f, "source: {:#?}", self.source)?;
                Ok(())
            }
        }

        pub trait IntoParsingError {
            fn into_parsing_err(self, $($key: $type,)+) -> ParsingError;
        }

        impl<T> IntoParsingError for T
        where
            T: Into<anyhow::Error>,
        {
            fn into_parsing_err(self, $($key: $type,)+) -> ParsingError {
                ParsingError {
                    $($key,)+
                    source: self.into(),
                }
            }
        }

        pub trait IntoParsingResult {
            type OkType;
            fn into_parsing_result(self, $($key: $type,)+) -> ParsingResult<Self::OkType>;
        }

        impl<T, E> IntoParsingResult for Result<T, E>
        where
            E: Into<anyhow::Error>,
        {
            type OkType = T;

            fn into_parsing_result(self, $($key: $type,)+) -> ParsingResult<Self::OkType> {
                self.map_err(|err| err.into_parsing_err($($key,)+))
            }
        }
    };
}

pub type ParsingResult<T> = Result<T, ParsingError>;

impl_parsing_error!(
    rule: Rule: "Rule",
    code: String: "Raw code string"
);

pub fn wrap_parsing_result<T>(
    rule_pair: Pair<Rule>,
    ops_fn: impl FnOnce(Pair<Rule>) -> anyhow::Result<T>,
) -> ParsingResult<T> {
    let rule = rule_pair.as_rule();
    let code = rule_pair.as_str().to_string();
    ops_fn(rule_pair).into_parsing_result(rule, code)
}
