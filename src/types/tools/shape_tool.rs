use std::marker::PhantomData;

use super::select_tool::Select;
use super::ToolAction;
use crate::store::shapes::Shapes;
use crate::store::tools::Tools;
use crate::types::events::CanvasEvent;
use crate::types::shapes::{BBox, Drawable, Ellipse, Rectangle, Shape, ShapeType};

pub trait ShapeToolDetails {
    fn shape_type() -> ShapeType;
    fn button_icon(&self) -> &'static str;
    fn button_title(&self) -> &'static str;
}

#[derive(Default, Clone)]
pub struct ShapeTool<T> {
    marker: PhantomData<T>,
}

impl<T> ToolAction for ShapeTool<T>
where
    T: ShapeToolDetails + Default,
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
        tool_shape: &mut Option<Drawable>,
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
                tool_shape.replace(T::shape_type().get_drawable(&BBox::from_corner(start, end)));
                shapes.version.increment();
                true
            }
            CanvasEvent::DragEnd((start, end)) => {
                let shape = Shape::new(&BBox::from_corner(start, end), T::shape_type());
                shapes.selected_shapes = vec![shape.get_id().clone()];
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
