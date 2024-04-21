use web_sys::HtmlCanvasElement;

use super::{
    canvas::refresh_canvas,
    shapes::{BBox, Draw, Rectangle},
};

#[allow(unused_variables)]
pub trait Tool {
    fn button_icon(&self) -> &'static str;
    fn button_title(&self) -> &'static str;
    fn onmousedown(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) {
    }
    fn onmouseup(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) {
    }
    fn onmousemove(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) {
    }
}

#[derive(Default)]
pub struct RectangleTool {
    start: Option<(f64, f64)>,
}

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
    ) {
        self.start.replace(position);
        shapes.push(Box::new(Rectangle::new(position.0, position.1, 0.0, 0.0)));
        refresh_canvas(canvas, shapes);
    }

    fn onmouseup(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) {
        if let (Some(shape), Some(start)) = (shapes.last_mut(), self.start) {
            shape.resize_to_bbox(BBox::from_corner(start, position));
            refresh_canvas(canvas, shapes);
        }
        self.start = None
    }

    fn onmousemove(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) {
        if let (Some(shape), Some(start)) = (shapes.last_mut(), self.start) {
            shape.resize_to_bbox(BBox::from_corner(start, position));
            refresh_canvas(canvas, shapes);
        } else {
            self.start = None
        }
    }
}
