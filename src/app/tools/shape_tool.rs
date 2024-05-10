use super::ToolAction;
use crate::app::shapes::{BBox, Draw, Shape};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct ShapeTool {
    start: Option<(f64, f64)>,
    icon: &'static str,
    title: &'static str,
    shape: Shape,
}

impl ShapeTool {
    pub fn new(icon: &'static str, title: &'static str, shape: Shape) -> Self {
        Self {
            icon,
            title,
            shape,
            start: None,
        }
    }
}

#[allow(unused_variables)]
impl ToolAction for ShapeTool {
    fn button_icon(&self) -> &'static str {
        self.icon
    }

    fn button_title(&self) -> &'static str {
        self.title
    }

    fn onmousedown(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Shape>,
    ) -> bool {
        self.start.replace(position);
        self.shape
            .resize_to_bbox(&BBox::from_corner(position, position));
        true
    }

    fn onmouseup(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Shape>,
    ) -> bool {
        if let Some(start) = self.start.take() {
            let mut shape = self.shape.clone();
            let changed = shape.resize_to_bbox(&BBox::from_corner(start, position));
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
        shapes: &mut Vec<Shape>,
    ) -> bool {
        if let Some(start) = self.start {
            self.shape
                .resize_to_bbox(&BBox::from_corner(start, position))
        } else {
            false
        }
    }

    fn draw_extra_shapes(&self, context: &CanvasRenderingContext2d) {
        if self.start.is_some() {
            self.shape.draw(context);
        }
    }
}
