mod expr;

use super::{VnslBlock, VnslCommand};
use crate::impl_deref;
pub use expr::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VnslStatement {
    Command(VnslCommand),
    Choices(VnslChoices),
    Expr(VnslExpr),
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VnslChoices {
    pub choices: Vec<VnslChoice>,
}
impl_deref!(VnslChoices, [VnslChoice], choices);

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VnslChoice {
    pub text: String,
    pub block: VnslBlock,
}
