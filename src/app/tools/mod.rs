use web_sys::HtmlCanvasElement;

use super::shapes::Draw;
pub mod rectangle_tool;

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
    fn onmouseleave(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) {
        self.onmouseup(position, canvas, shapes)
    }
}
