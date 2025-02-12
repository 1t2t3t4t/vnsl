use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use vnsl_core::model::{VnslDataType, VnslScene};

use crate::{runstack::RunStack, Runtime};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    pub current_scene: Option<VnslScene>,
    pub current_character_id: Option<String>,
    pub run_stack: RunStack,
    pub lua_globals: HashMap<String, VnslDataType>,
}

impl From<&Runtime> for Snapshot {
    fn from(value: &Runtime) -> Self {
        Self {
            current_scene: value.get_current_scene().cloned(),
            current_character_id: value.current_character_id().cloned(),
            run_stack: value.get_run_stack().clone(),
            lua_globals: value.get_run_context().lua_runtime.export_globals(),
        }
    }
}
