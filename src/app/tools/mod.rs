use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::shapes::Shape;
pub mod select_tool;
pub mod shape_tool;

#[allow(unused_variables)]
pub trait Tool {
    fn button_icon(&self) -> &'static str;
    fn button_title(&self) -> &'static str;
    fn onmousedown(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Shape>,
    ) -> bool {
        false
    }
    fn onmouseup(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Shape>,
    ) -> bool {
        false
    }
    fn onmousemove(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Shape>,
    ) -> bool {
        false
    }
    fn onmouseleave(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Shape>,
    ) -> bool {
        self.onmouseup(position, canvas, shapes)
    }
    fn draw_extra_shapes(&self, context: &CanvasRenderingContext2d) {}
}
