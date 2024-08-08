use super::VnslDataType;

#[derive(Debug, Clone, PartialEq)]
pub enum VnslCommand {
    Dialogue(VnslDialogue),
    SetCharacter(VnslSetCharacter),
    Action(VnslAction),
    Jump(VnslJump),
    Global(VnslGlobal),
}

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
