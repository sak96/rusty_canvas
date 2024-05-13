use enum_dispatch::enum_dispatch;

use crate::store::{shapes::Shapes, tools::Tools};
use crate::types::events::CanvasEvent;
use crate::types::shapes::Shape;

pub mod select_tool;
pub mod shape_tool;

use select_tool::Select;
use shape_tool::{EllipseShape, RectangleShape};

use strum_macros::{Display, EnumIter, EnumString};

#[enum_dispatch(ToolAction)]
#[derive(EnumString, EnumIter, Display, Clone)]
pub enum Tool {
    Select,
    RectangleShape,
    EllipseShape,
}

impl Default for Tool {
    fn default() -> Self {
        Select {}.into()
    }
}

impl PartialEq for Tool {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl Eq for Tool {}

#[allow(unused_variables)]
#[enum_dispatch]
pub trait ToolAction {
    fn button_icon(&self) -> &'static str;
    fn button_title(&self) -> &'static str;
    fn handle_event(
        &mut self,
        event: &CanvasEvent,
        tools: &mut Tools,
        tool_shape: &mut Option<Shape>,
        shapes: &mut Shapes,
    ) -> bool;
}
