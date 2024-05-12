use super::ToolAction;
use crate::store::shapes::{BBox, Shape};
use crate::utils::events::Event;
use web_sys::CanvasRenderingContext2d;

pub struct ShapeTool {
    icon: &'static str,
    title: &'static str,
    shape: Shape,
}

impl ShapeTool {
    pub fn new(icon: &'static str, title: &'static str, shape: Shape) -> Self {
        Self { icon, title, shape }
    }
}

impl ToolAction for ShapeTool {
    fn button_icon(&self) -> &'static str {
        self.icon
    }

    fn button_title(&self) -> &'static str {
        self.title
    }

    fn draw_extra_shapes(&self, context: &CanvasRenderingContext2d) {
        let bbox = self.shape.bbox();
        if bbox.width != 0.0 || bbox.height != 0.0 {
            self.shape.draw(context);
        }
    }

    fn handle_event(&mut self, event: &Event, shapes: &mut Vec<Shape>) -> bool {
        match event {
            Event::DragMove((start, end)) => {
                self.shape.resize_to_bbox(&BBox::from_corner(start, end));
                true
            }
            Event::DragEnd((start, end)) => {
                let mut shape = self.shape.clone();
                let changed = shape.resize_to_bbox(&BBox::from_corner(start, end));
                self.shape.resize_to_bbox(&BBox::default());
                shapes.push(shape);
                changed
            }
            _ => false,
        }
    }
}
