use crate::types::shapes::Shape;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct Shapes(pub Vec<Shape>);
