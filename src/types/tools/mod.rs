use enum_dispatch::enum_dispatch;

use crate::store::shapes::Shapes;
use crate::types::events::Event;
use crate::types::shapes::Shape;

pub mod select_tool;
pub mod shape_tool;

use select_tool::SelectTool;
use shape_tool::{EllipseTool, RectangleTool};

use strum_macros::{Display, EnumIter, EnumString};

#[enum_dispatch(ToolAction)]
#[allow(clippy::enum_variant_names)]
#[derive(EnumString, EnumIter, Display, Clone)]
pub enum Tool {
    SelectTool,
    RectangleTool,
    EllipseTool,
}

impl Default for Tool {
    fn default() -> Self {
        SelectTool {}.into()
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
    fn handle_event(&mut self, event: &Event, shapes: &mut Shapes) -> Option<Shape>;
    fn deselect(&mut self, tool_shape: &mut Option<Shape>, shapes: &mut Shapes) {}
}
