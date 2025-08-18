use std::marker::PhantomData;

use super::ToolAction;
use super::select_tool::Select;
use crate::store::AppState;
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
        tool_shape: &mut Option<Drawable>,
        app_state: &mut AppState,
    ) -> bool {
        match event {
            CanvasEvent::SelectTool => {
                app_state.set_pointer("crosshair");
                true
            }
            CanvasEvent::DeselectTool => {
                app_state.set_pointer("default");
                true
            }
            CanvasEvent::DragMove((start, end)) => {
                tool_shape.replace(T::shape_type().get_drawable(&BBox::from_corner(start, end)));
                app_state.set_redraw();
                true
            }
            CanvasEvent::DragEnd((start, end)) => {
                let shape = Shape::new(
                    &BBox::from_corner(start, end),
                    T::shape_type(),
                    app_state.get_color().clone(),
                );
                app_state.replace_selected(vec![shape.get_id().clone()]);
                app_state.add_shape(shape);
                tool_shape.take();
                app_state.set_tool(Select {}.into());
                true
            }
            _ => false,
        }
    }
}
pub type RectangleShape = ShapeTool<Rectangle>;
pub type EllipseShape = ShapeTool<Ellipse>;
