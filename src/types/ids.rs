use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Id(String);

impl Default for Id {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}
