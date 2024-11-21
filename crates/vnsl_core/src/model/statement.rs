mod expr;

use super::{VnslBlock, VnslCommand};
use crate::impl_deref;
pub use expr::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum VnslStatement {
    Command(VnslCommand),
    Choices(VnslChoices),
    Condition(VnslCondition),
    Expr(VnslExpr),
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VnslChoices {
    pub choices: Vec<VnslChoice>,
}
impl_deref!(VnslChoices, [VnslChoice], choices);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VnslChoice {
    pub id: String,
    pub text: String,
    pub block: VnslBlock,
}

impl Default for VnslChoice {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            text: String::default(),
            block: VnslBlock::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VnslCondition {
    pub if_block: VnslConditionBlock,
    pub elif_block: Vec<VnslConditionBlock>,
    pub else_block: Option<VnslBlock>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VnslConditionBlock {
    pub iden: String,
    pub block: VnslBlock,
}
