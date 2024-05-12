use enum_dispatch::enum_dispatch;

use crate::store::shapes::Shapes;
use crate::types::events::Event;
use crate::types::shapes::Shape;
pub mod select_tool;
pub mod shape_tool;

use select_tool::SelectTool;
use shape_tool::{EllipseTool, RectangleTool};

#[enum_dispatch(ToolAction)]
#[allow(clippy::enum_variant_names)]
pub enum Tool {
    RectangleTool,
    EllipseTool,
    SelectTool,
}

#[allow(unused_variables)]
#[enum_dispatch]
pub trait ToolAction {
    fn button_icon(&self) -> &'static str;
    fn button_title(&self) -> &'static str;
    fn handle_event(&mut self, event: &Event, shapes: &mut Shapes) -> Option<Shape>;
}

pub struct ToolBar {
    tools: [Tool; 3],
    tool_idx: usize,
}

impl ToolBar {
    pub fn new() -> Self {
        Self {
            tools: [
                SelectTool {}.into(),
                RectangleTool::default().into(),
                EllipseTool::default().into(),
            ],
            tool_idx: 0,
        }
    }

    pub fn all_tools(&self) -> &[Tool] {
        &self.tools
    }

    pub fn get_tool_idx(&self) -> usize {
        self.tool_idx
    }

    pub fn set_tool_idx(&mut self, mut tool_idx: usize) {
        if tool_idx >= self.tools.len() {
            tool_idx = 0;
        }
        self.tool_idx = tool_idx
    }

    pub fn handle_event(&mut self, event: &Event, shapes: &mut Shapes) -> Option<Shape> {
        self.tools[self.tool_idx].handle_event(event, shapes)
    }
}
