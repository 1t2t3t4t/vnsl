use super::{VnslBlock, VnslDataType};
use crate::impl_deref;

#[derive(Debug, Clone, PartialEq)]
pub struct VnslDialogue {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VnslSetCharacter {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VnslJump {
    pub to_label: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VnslAction {
    pub name: String,
    pub args: Vec<VnslActionArg>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VnslActionArg {
    pub name: Option<String>,
    pub data_type: VnslDataType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VnslGlobal {
    pub name: String,
    pub value: VnslDataType,
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
