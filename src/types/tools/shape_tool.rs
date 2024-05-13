use std::marker::PhantomData;

use super::select_tool::Select;
use super::ToolAction;
use crate::store::shapes::Shapes;
use crate::store::tools::Tools;
use crate::types::events::CanvasEvent;
use crate::types::shapes::{BBox, Ellipse, Rectangle, Shape};

pub trait ShapeToolDetails {
    fn button_icon(&self) -> &'static str;
    fn button_title(&self) -> &'static str;
}

#[derive(Default, Clone)]
pub struct ShapeTool<T> {
    marker: PhantomData<T>,
}

impl<T> ToolAction for ShapeTool<T>
where
    T: Into<Shape> + ShapeToolDetails + Default,
{
    fn button_icon(&self) -> &'static str {
        ShapeToolDetails::button_icon(&T::default())
    }

    fn button_title(&self) -> &'static str {
        ShapeToolDetails::button_title(&T::default())
    }

    fn handle_event(
        &mut self,
        event: &CanvasEvent,
        tools: &mut Tools,
        tool_shape: &mut Option<Shape>,
        shapes: &mut Shapes,
    ) -> bool {
        match event {
            CanvasEvent::SelectTool => {
                tools.pointer = "crosshair".into();
                false
            }
            CanvasEvent::DeselectTool => {
                tools.pointer = "default".into();
                false
            }
            CanvasEvent::DragMove((start, end)) => {
                let mut shape = T::default().into();
                shape.resize_to_bbox(&BBox::from_corner(start, end));
                shapes.version.increment();
                tools.pointer = "crosshair".into();
                tool_shape.replace(shape);
                true
            }
            CanvasEvent::DragEnd((start, end)) => {
                let mut shape = T::default().into();
                shape.resize_to_bbox(&BBox::from_corner(start, end));
                shapes.selected_shapes.clear();
                shapes.selected_shapes.push(shape.get_id().clone());
                shapes.shapes.push(shape);
                tool_shape.take();
                tools.tool = Select {}.into();
                true
            }
            _ => false,
        }
    }
}
pub type RectangleShape = ShapeTool<Rectangle>;
pub type EllipseShape = ShapeTool<Ellipse>;
