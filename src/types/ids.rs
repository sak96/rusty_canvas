use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Id(String);

impl Default for Id {
    fn default() -> Self {
        Self(blob_uuid::random_blob())
    }
}
