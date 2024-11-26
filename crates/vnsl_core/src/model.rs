mod command;
mod statement;

#[cfg(debug_assertions)]
use std::collections::BTreeMap;

#[cfg(not(debug_assertions))]
use std::collections::HashMap;

use crate::impl_deref;

pub use command::*;
use serde::{Deserialize, Serialize};
pub use statement::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VnslScene {
    pub name: String,
    pub main_block: VnslBlock,
    #[cfg(debug_assertions)]
    pub labels: BTreeMap<String, VnslLabel>,

    #[cfg(not(debug_assertions))]
    pub labels: HashMap<String, VnslLabel>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VnslLabel {
    pub name: String,
    pub block: VnslBlock,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VnslBlock {
    pub statements: Vec<VnslStatement>,
}
impl_deref!(VnslBlock, [VnslStatement], statements);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum VnslDataType {
    String(String),
    Number(f64),
    Bool(bool),
}

impl VnslScene {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            main_block: VnslBlock::default(),
            labels: Default::default(),
        }
    }
}
