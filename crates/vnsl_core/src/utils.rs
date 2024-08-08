use std::ops::Deref;

use crate::model::{VnslCommand, VnslGlobal, VnslStatement};

pub trait GlobalAccess {
    fn get_globals(&self) -> Vec<VnslGlobal>;
}

impl<T> GlobalAccess for T
where
    T: Deref<Target = [VnslStatement]>,
{
    fn get_globals(&self) -> Vec<VnslGlobal> {
        self.iter()
            .filter_map(|s| match s {
                VnslStatement::Command(cmd) => match cmd {
                    VnslCommand::Global(global) => Some(global.clone()),
                    _ => None,
                },
                _ => None,
            })
            .collect()
    }
}
