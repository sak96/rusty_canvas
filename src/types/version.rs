use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Version {
    version: usize,
}

impl Version {
    pub fn increment(&mut self) {
        self.version = self.version.overflowing_add(1).0
    }
}
