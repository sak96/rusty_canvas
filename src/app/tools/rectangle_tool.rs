use super::Tool;
use crate::app::shapes::{BBox, Draw, Rectangle};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Default)]
pub struct RectangleTool {
    bbox: Option<(f64, f64)>,
    shape: Box<Rectangle>,
}

#[allow(unused_variables)]
impl Tool for RectangleTool {
    fn button_icon(&self) -> &'static str {
        "\u{2B1B}"
    }

    fn button_title(&self) -> &'static str {
        "Rectangle drawing tool."
    }

    fn onmousedown(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) -> bool {
        self.bbox.replace(position);
        self.shape
            .resize_to_bbox(BBox::from_corner(position, position));
        true
    }

    fn onmouseup(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) -> bool {
        if let Some(start) = self.bbox.take() {
            let mut shape = Box::<Rectangle>::default();
            let changed = shape.resize_to_bbox(BBox::from_corner(start, position));
            shapes.push(shape);
            changed
        } else {
            false
        }
    }

    fn onmousemove(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) -> bool {
        if let Some(start) = self.bbox {
            self.shape
                .resize_to_bbox(BBox::from_corner(start, position))
        } else {
            false
        }
    }

    fn draw_extra_shapes(&self, interface: &CanvasRenderingContext2d) {
        self.shape.draw(interface);
    }
}
