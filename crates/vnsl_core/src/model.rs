mod command;
mod statement;

use crate::impl_deref;

pub use command::*;
pub use statement::*;

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
