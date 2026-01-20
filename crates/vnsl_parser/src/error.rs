use pest::iterators::Pair;
use thiserror::Error;

use crate::Rule;

// Re-export for convenience - these traits may be used by consumers of the crate
#[allow(dead_code)]

/// The unified result type for all parsing operations.
pub type ParsingResult<T> = Result<T, ParsingError>;

/// A parsing error with context about where and what failed.
#[derive(Debug, Error)]
#[error("{kind}")]
pub struct ParsingError {
    /// The specific kind of parsing error.
    #[source]
    pub kind: ParsingErrorKind,
}

impl ParsingError {
    /// Create a new parsing error from a rule pair and error kind.
    pub fn new(_pair: &Pair<Rule>, kind: ParsingErrorKind) -> Self {
        Self { kind }
    }

    /// Create an unexpected rule error.
    pub fn unexpected_rule(pair: &Pair<Rule>, expected: &[Rule], context: &'static str) -> Self {
        Self::new(
            pair,
            ParsingErrorKind::UnexpectedRule {
                expected: expected.to_vec(),
                found: pair.as_rule(),
                context,
            },
        )
    }

    /// Create a missing required element error.
    pub fn missing_required(pair: &Pair<Rule>, element: impl Into<String>) -> Self {
        Self::new(pair, ParsingErrorKind::MissingRequired(element.into()))
    }

    /// Create an invalid value error.
    pub fn invalid_value(pair: &Pair<Rule>, message: impl Into<String>) -> Self {
        Self::new(pair, ParsingErrorKind::InvalidValue(message.into()))
    }
}

/// The specific kind of parsing error that occurred.
#[derive(Debug, Error)]
pub enum ParsingErrorKind {
    #[error("Unexpected rule in {context}: expected one of {expected:?}, found {found:?}")]
    UnexpectedRule {
        expected: Vec<Rule>,
        found: Rule,
        context: &'static str,
    },

    #[error("Missing required element: {0}")]
    MissingRequired(String),

    #[error("Invalid value: {0}")]
    InvalidValue(String),

    #[error("Extra element found: {0}")]
    ExtraElement(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Trait for converting errors into ParsingError with context.
#[allow(dead_code)]
pub trait IntoParsingError {
    fn into_parsing_error(self, pair: &Pair<Rule>) -> ParsingError;
}

impl<E> IntoParsingError for E
where
    E: Into<anyhow::Error>,
{
    fn into_parsing_error(self, pair: &Pair<Rule>) -> ParsingError {
        ParsingError::new(pair, ParsingErrorKind::Other(self.into()))
    }
}

/// Trait for converting Result types into ParsingResult with context.
#[allow(dead_code)]
pub trait IntoParsingResult<T> {
    fn into_parsing_result(self, pair: &Pair<Rule>) -> ParsingResult<T>;
}

impl<T, E> IntoParsingResult<T> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn into_parsing_result(self, pair: &Pair<Rule>) -> ParsingResult<T> {
        self.map_err(|e| e.into_parsing_error(pair))
    }
}

/// Wraps a parsing operation, converting any error into a ParsingError with context.
///
/// This is the preferred way to call nested parsing functions that return `anyhow::Result`
/// or any other error type, as it captures the rule context for better error messages.
#[inline(always)]
pub fn wrap_parsing_result<T, E>(
    rule_pair: Pair<Rule>,
    ops_fn: impl FnOnce(Pair<Rule>) -> Result<T, E>,
) -> ParsingResult<T>
where
    E: Into<anyhow::Error>,
{
    ops_fn(rule_pair).map_err(|e| ParsingError {
        kind: ParsingErrorKind::Other(e.into()),
    })
}

/// Helper function to create an unexpected rule error and return it as a Result.
///
/// Use this to replace `unreachable!()` calls in match arms.
#[inline]
pub fn unexpected_rule<T>(
    pair: &Pair<Rule>,
    expected: &[Rule],
    context: &'static str,
) -> ParsingResult<T> {
    Err(ParsingError::unexpected_rule(pair, expected, context))
}

/// Helper function to create an unexpected rule error from just the found rule.
///
/// Use this when you don't have the pair available but know the rule.
#[inline]
#[allow(dead_code)]
pub fn unexpected_rule_simple<T>(
    found: Rule,
    expected: &[Rule],
    context: &'static str,
) -> ParsingResult<T> {
    Err(ParsingError {
        kind: ParsingErrorKind::UnexpectedRule {
            expected: expected.to_vec(),
            found,
            context,
        },
    })
}
