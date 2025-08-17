use super::ToolAction;
use crate::store::AppState;
use crate::store::shapes::Shapes;
use crate::types::events::{CanvasEvent, Point};
use crate::types::ids::Id;
use crate::types::shapes::Drawable;

#[derive(Default, Clone)]
pub struct Erase();

impl Erase {
    const MARGIN: f64 = 10.0;
    fn get_selected(point: &Point, shapes: &Shapes) -> Vec<Id> {
        shapes
            .shapes
            .iter()
            .filter(|shape| shape.contains(point, Self::MARGIN))
            .map(|shape| shape.get_id().clone())
            .collect()
    }
}

impl ToolAction for Erase {
    fn button_icon(&self) -> &'static str {
        "ti-eraser"
    }

    fn button_title(&self) -> &'static str {
        "Eraser Tool."
    }

    fn handle_event(
        &mut self,
        event: &CanvasEvent,
        _tool_shape: &mut Option<Drawable>,
        app_state: &mut AppState,
    ) -> bool {
        match event {
            CanvasEvent::DragEnd((_, point)) | CanvasEvent::Click(point) => {
                let shapes = Self::get_selected(point, app_state.get_shapes());
                let changed = !shapes.is_empty();
                app_state.remove_shapes(shapes);
                changed
            }
            _ => false,
        }
    }
}
