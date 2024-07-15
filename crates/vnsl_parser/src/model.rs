#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VnslScene {
    pub name: String,
    pub main_statements: Vec<VnslStatement>,
    pub labels: Vec<VnslLabel>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VnslLabel {
    pub name: String,
    pub statements: Vec<VnslStatement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VnslStatement {
    Command(VnslCommand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VnslCommand {
    Dialogue { text: String },
    Action { identifier: String },
    None,
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
