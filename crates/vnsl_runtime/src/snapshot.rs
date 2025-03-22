use std::collections::HashMap;

use anyhow::Result;
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

impl Snapshot {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let bytes = bincode::serde::encode_to_vec(self, bincode::config::standard())?;
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let (snapshot, _) = bincode::serde::decode_from_slice::<Snapshot, _>(
            &bytes[..],
            bincode::config::standard(),
        )?;
        Ok(snapshot)
    }
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

#[cfg(test)]
mod test {
    use vnsl_core::model::VnslScene;

    use super::Snapshot;

    #[test]
    fn test_encode_decode() -> anyhow::Result<()> {
        let mut snapshot = Snapshot::default();
        snapshot.current_character_id = Some("My name".into());
        snapshot.current_scene = Some(VnslScene::default());

        let bytes = snapshot.encode()?;
        let dec = Snapshot::decode(&bytes)?;

        assert_eq!(snapshot, dec);
        Ok(())
    }
}
