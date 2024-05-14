use serde::{Deserialize, Serialize};

use crate::types::ids::Id;
use crate::types::shapes::Shape;
use crate::types::version::Version;

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Shapes {
    pub shapes: Vec<Shape>,
    pub selected_shapes: Vec<Id>,
    pub version: Version,
}
