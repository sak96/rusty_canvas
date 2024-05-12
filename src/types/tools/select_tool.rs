use super::ToolAction;
use crate::store::shapes::Shapes;
use crate::types::events::Event;
use crate::types::shapes::{BBox, Selection, Shape};

#[derive(Default)]
pub struct SelectTool;

impl SelectTool {
    fn update_selection(selection: &BBox, shapes: &mut Shapes) {
        shapes.selected_shapes.clear();
        for shape in &shapes.shapes {
            if shape.bbox().in_(selection) {
                shapes.selected_shapes.push(shape.get_id().clone());
            }
        }
    }
}

impl ToolAction for SelectTool {
    fn button_icon(&self) -> &'static str {
        "ti-marquee"
    }

    fn button_title(&self) -> &'static str {
        "Selection tool."
    }

    fn handle_event(&mut self, event: &Event, shapes: &mut Shapes) -> Option<Shape> {
        match event {
            Event::PointerEventStart(_) => {
                shapes.selected_shapes.clear();
                None
            }
            Event::DragMove((start, end)) => {
                let selection = BBox::from_corner(start, end);
                Self::update_selection(&selection, shapes);
                let mut shape: Shape = Selection::default().into();
                shape.resize_to_bbox(&selection);
                shapes.version.increment();
                Some(shape)
            }
            Event::DragEnd((start, end)) => {
                let selection = BBox::from_corner(start, end);
                Self::update_selection(&selection, shapes);
                None
            }
            _ => None,
        }
    }
}
