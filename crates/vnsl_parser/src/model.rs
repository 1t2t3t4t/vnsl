#[derive(Debug, Clone, PartialEq, Default)]
pub struct VnslScene {
    pub name: String,
    pub main_statements: Vec<VnslStatement>,
    pub labels: Vec<VnslLabel>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VnslLabel {
    pub name: String,
    pub block: VnslBlock,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VnslBlock {
    pub statements: Vec<VnslStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VnslStatement {
    Command(VnslCommand),
    Choices(VnslChoices),
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
pub enum VnslCommand {
    Dialogue { text: String },
    Action(VnslAction),
    None,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VnslChoices {
    pub choices: Vec<VnslChoice>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VnslChoice {
    pub text: String,
    pub block: VnslBlock,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VnslDataType {
    String(String),
    Number(f64),
    Bool(bool),
}

impl VnslScene {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            main_statements: vec![],
            labels: vec![],
        }
    }
}
