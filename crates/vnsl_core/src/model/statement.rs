use crate::impl_deref;

use super::{VnslAction, VnslBlock, VnslDialogue, VnslGlobal, VnslJump, VnslSetCharacter};

#[derive(Debug, Clone, PartialEq)]
pub enum VnslStatement {
    Command(VnslCommand),
    Choices(VnslChoices),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VnslCommand {
    Dialogue(VnslDialogue),
    SetCharacter(VnslSetCharacter),
    Action(VnslAction),
    Jump(VnslJump),
    Global(VnslGlobal),
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
