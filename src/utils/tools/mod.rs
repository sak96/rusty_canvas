use enum_dispatch::enum_dispatch;
use web_sys::CanvasRenderingContext2d;

use crate::types::events::Event;
use crate::types::shapes::{Ellipse, Rectangle, Shape};
pub mod select_tool;
pub mod shape_tool;

use select_tool::SelectTool;
use shape_tool::ShapeTool;

#[enum_dispatch(ToolAction)]
pub enum Tool {
    ShapeTool,
    SelectTool,
}

#[allow(unused_variables)]
#[enum_dispatch]
pub trait ToolAction {
    fn button_icon(&self) -> &'static str;
    fn button_title(&self) -> &'static str;
    fn draw_extra_shapes(&self, context: &CanvasRenderingContext2d) {}
    fn handle_event(&mut self, event: &Event, shapes: &mut Vec<Shape>) -> bool;
}

pub struct ToolBar {
    tools: [Tool; 3],
    tool_idx: usize,
}

impl ToolBar {
    pub fn new() -> Self {
        Self {
            tools: [
                select_tool::SelectTool::default().into(),
                shape_tool::ShapeTool::new(
                    "ti-square",
                    "Rectangle drawing tool.",
                    Rectangle::default().into(),
                )
                .into(),
                shape_tool::ShapeTool::new(
                    "ti-circle",
                    "Ellipse drawing tool.",
                    Ellipse::default().into(),
                )
                .into(),
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

    pub fn tool(&self) -> &Tool {
        &self.tools[self.tool_idx]
    }

    pub fn handle_event(&mut self, event: &Event, shapes: &mut Vec<Shape>) -> bool {
        self.tools[self.tool_idx].handle_event(event, shapes)
    }
}
