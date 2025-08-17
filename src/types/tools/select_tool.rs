use super::ToolAction;
use crate::store::AppState;
use crate::store::shapes::Shapes;
use crate::types::events::CanvasEvent;
use crate::types::ids::Id;
use crate::types::shapes::{BBox, Drawable, ShapeType};

#[derive(Default, Clone)]
pub struct Select;

impl Select {
    fn get_selected(selection: &BBox, shapes: &Shapes) -> Vec<Id> {
        shapes
            .shapes
            .iter()
            .filter(|shape| shape.isin(selection))
            .map(|shape| shape.get_id().clone())
            .collect()
    }
}

impl ToolAction for Select {
    fn button_icon(&self) -> &'static str {
        "ti-marquee-2"
    }

    fn button_title(&self) -> &'static str {
        "Selection tool."
    }

    fn handle_event(
        &mut self,
        event: &CanvasEvent,
        tool_shape: &mut Option<Drawable>,
        app_state: &mut AppState,
    ) -> bool {
        match event {
            CanvasEvent::PointerEventStart(_) => {
                app_state.replace_selected(vec![]);
                tool_shape.take();
                true
            }
            CanvasEvent::DragMove((start, end)) => {
                let selection = BBox::from_corner(start, end);
                app_state.replace_selected(Self::get_selected(&selection, app_state.get_shapes()));
                tool_shape.replace(ShapeType::Selection.get_drawable(&selection));
                true
            }
            CanvasEvent::DragEnd((start, end)) => {
                let selection = BBox::from_corner(start, end);
                app_state.replace_selected(Self::get_selected(&selection, app_state.get_shapes()));
                tool_shape.take();
                true
            }
            CanvasEvent::DeselectTool => {
                tool_shape.take();
                app_state.replace_selected(vec![]);
                app_state.set_pointer("default");
                true
            }
            CanvasEvent::KeyPress(key) => {
                let mut changed = false;
                if key.eq(&"Delete") {
                    let selected_id = app_state.get_selected().to_vec();
                    app_state.replace_selected(vec![]);
                    changed = !selected_id.is_empty();
                    app_state.remove_shapes(selected_id);
                }
                changed
            }
            _ => false,
        }
    }
}
