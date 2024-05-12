use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::types::ids::Id;
use crate::types::shapes::Shape;
use crate::types::version::Version;

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct Shapes {
    pub shapes: Vec<Shape>,
    pub selected_shapes: Vec<Id>,
    pub version: Version,
}
