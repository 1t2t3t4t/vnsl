mod expr;

use super::{VnslBlock, VnslCommand};
use crate::impl_deref;
pub use expr::*;

#[derive(Debug, Clone, PartialEq)]
pub enum VnslStatement {
    Command(VnslCommand),
    Choices(VnslChoices),
    Expr(VnslExpr),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VnslChoices {
    pub choices: Vec<VnslChoice>,
}
impl_deref!(VnslChoices, [VnslChoice], choices);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VnslChoice {
    pub text: String,
    pub block: VnslBlock,
}
