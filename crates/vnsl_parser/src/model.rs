mod command;

pub use command::*;

#[macro_export]
macro_rules! impl_deref {
    ($type:ty, $target:ty, $member:tt) => {
        impl std::ops::Deref for $type {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.$member
            }
        }
    };
}

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

impl_deref!(VnslBlock, [VnslStatement], statements);

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
    Var(VnslVar),
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

    pub fn all_statements(&self) -> Vec<VnslStatement> {
        let mut stmts = self.main_statements.clone();
        let mut label_stmts = self
            .labels
            .iter()
            .flat_map(|l| l.block.statements.clone())
            .collect();
        stmts.append(&mut label_stmts);
        stmts
    }
}
