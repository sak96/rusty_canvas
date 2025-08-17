use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::types::{ids::Id, shapes::Shape, tools::Tool};

use self::shapes::Shapes;

pub mod shapes;
pub mod tools;

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct AppState {
    shapes: Shapes,
    tools: tools::Tools,
    pointer: String,
}

impl AppState {
    pub fn get_pointer(&self) -> &str {
        &self.pointer
    }
    pub fn set_pointer(&mut self, pointer: &str) {
        self.pointer = pointer.to_string();
    }

    pub fn get_tool(&self) -> &Tool {
        &self.tools.tool
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.tools.tool = tool
    }

    pub fn get_shapes(&self) -> &Shapes {
        &self.shapes
    }

    pub fn get_selected(&mut self) -> &[Id] {
        &self.shapes.selected_shapes
    }

    pub fn replace_selected(&mut self, new_shapes: Vec<Id>) {
        self.shapes.selected_shapes = new_shapes;
        self.shapes.version.increment();
    }

    pub fn remove_shapes(&mut self, shapes: Vec<Id>) {
        self.shapes.shapes = self
            .shapes
            .shapes
            .drain(..)
            .filter(|x| !shapes.contains(x.get_id()))
            .collect();
        self.shapes.version.increment();
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.shapes.push(shape);
        self.shapes.version.increment();
    }

    pub fn set_redraw(&mut self) {
        self.shapes.version.increment();
    }
}
