use web_sys::HtmlCanvasElement;

use super::shapes::Draw;

#[allow(unused_variables)]
pub trait Tool {
    fn button_icon(&self) -> &'static str;
    fn button_title(&self) -> &'static str;
    fn on_mouse_down(&mut self, position: (f64,f64), canvas: HtmlCanvasElement, shapes: &mut Vec<Box<dyn Draw>>) {}
    fn on_mouse_up(&mut self, position: (f64,f64), canvas: HtmlCanvasElement, shapes: &mut Vec<Box<dyn Draw>>) {}
    fn on_mouse_move(&mut self, position: (f64,f64), canvas: HtmlCanvasElement, shapes: &mut Vec<Box<dyn Draw>>) {}
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
}
