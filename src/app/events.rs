use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::NodeRef;
use yew::PointerEvent;

use super::shapes::{Draw, Shape};
use super::tools::{Tool, ToolAction};
use super::{shapes, tools};

pub struct EventHandler {
    canvas_ref: NodeRef,
    tools: [Tool; 3],
    shapes: Vec<Shape>,
}

impl EventHandler {
    pub fn new(canvas_ref: NodeRef) -> Self {
        Self {
            canvas_ref,
            tools: [
                tools::select_tool::SelectTool::default().into(),
                tools::shape_tool::ShapeTool::new(
                    "ti-square",
                    "Rectangle drawing tool.",
                    shapes::Rectangle::default().into(),
                )
                .into(),
                tools::shape_tool::ShapeTool::new(
                    "ti-circle",
                    "Ellipse drawing tool.",
                    shapes::Ellipse::default().into(),
                )
                .into(),
            ],
            shapes: vec![],
        }
    }
    pub fn tools(&self) -> &[Tool] {
        &self.tools
    }
    pub fn reset_canvas(&self) {
        self.redraw_canvas();
    }
    fn redraw_canvas(&self) -> CanvasRenderingContext2d {
        let canvas = self.get_canvas();
        canvas.set_width(canvas.client_width().abs_diff(0));
        canvas.set_height(canvas.client_height().abs_diff(0));
        let context: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        for shape in &self.shapes {
            shape.draw(&context);
        }
        context
    }

    fn get_event_canvas_postion(canvas: &HtmlCanvasElement, event: &PointerEvent) -> (f64, f64) {
        let rect = canvas.get_bounding_client_rect();
        let x = (event.client_x() as f64 - rect.left()) * (canvas.width() as f64 / rect.width());
        let y = (event.client_y() as f64 - rect.top()) * (canvas.height() as f64 / rect.height());
        (x, y)
    }

    pub fn handle_ptr_event(&mut self, event: PointerEvent, mut tool_idx: usize) {
        if tool_idx >= self.tools.len() {
            tool_idx = 0;
        }
        let canvas = self.get_canvas();
        let tool = &mut self.tools[tool_idx];
        let position = Self::get_event_canvas_postion(&canvas, &event);
        let view_changed = match event.type_().as_str() {
            "pointerdown" => {
                canvas.set_pointer_capture(event.pointer_id()).unwrap();
                tool.onmousedown(position, canvas.clone(), &mut self.shapes)
            }
            "pointerup" => {
                canvas.release_pointer_capture(event.pointer_id()).unwrap();
                tool.onmouseup(position, canvas.clone(), &mut self.shapes)
            }
            "pointermove" => tool.onmousemove(position, canvas.clone(), &mut self.shapes),
            "pointerleave" => tool.onmouseleave(position, canvas.clone(), &mut self.shapes),
            _ => false,
        };
        if view_changed {
            let tool = &self.tools[tool_idx];
            let context = self.redraw_canvas();
            tool.draw_extra_shapes(&context);
        }
    }
    fn get_canvas(&self) -> HtmlCanvasElement {
        self.canvas_ref.cast::<HtmlCanvasElement>().unwrap()
    }
}
