use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::PointerEvent;

use super::{
    shapes::{Draw, Shape},
    tools::{Tool, ToolAction},
};

pub fn refresh_canvas(canvas: HtmlCanvasElement, shapes: &[Shape], tool: Option<&Tool>) {
    canvas.set_width(canvas.client_width().abs_diff(0));
    canvas.set_height(canvas.client_height().abs_diff(0));
    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    for shape in shapes {
        shape.draw(&context);
    }
    if let Some(tool) = tool {
        tool.draw_extra_shapes(&context);
    }
}

pub fn get_event_canvas_postion(canvas: &HtmlCanvasElement, event: &PointerEvent) -> (f64, f64) {
    let rect = canvas.get_bounding_client_rect();
    let x = (event.client_x() as f64 - rect.left()) * (canvas.width() as f64 / rect.width());
    let y = (event.client_y() as f64 - rect.top()) * (canvas.height() as f64 / rect.height());
    (x, y)
}
