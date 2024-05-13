use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use yewdux::prelude::*;

use crate::types::tools::Tool;

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct Tools {
    pub tool: Tool,
    pub pointer: String,
}

impl serde::Serialize for Tool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for Tool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)
            .unwrap_or_default()
            .clone();
        Ok(Self::from_str(&value).unwrap_or_default())
    }
}
