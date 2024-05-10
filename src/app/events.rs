use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::NodeRef;
use yew::PointerEvent;

use super::shapes::{Draw, Shape};
use super::tools::{Tool, ToolAction};
use super::{shapes, tools};

type Point = (f64, f64);

#[non_exhaustive]
pub enum Event {
    PointerEventStart(Point),
    Hover(Point),
    DragMove((Point, Point)),
    DragEnd((Point, Point)),
    Click(Point),
}

pub struct EventHandler {
    canvas_ref: NodeRef,
    tools: [Tool; 3],
    shapes: Vec<Shape>,
    event: Option<Event>,
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
            event: None,
        }
    }

    pub fn all_tools(&self) -> &[Tool] {
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

    fn get_event_canvas_postion(canvas: &HtmlCanvasElement, event: &PointerEvent) -> Point {
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
        let position = Self::get_event_canvas_postion(&canvas, &event);
        let event = match event.type_().as_str() {
            "pointerdown" => {
                canvas.set_pointer_capture(event.pointer_id()).unwrap();
                Some(Event::PointerEventStart(position))
            }
            "pointerup" => {
                canvas.release_pointer_capture(event.pointer_id()).unwrap();
                match self.event {
                    Some(Event::PointerEventStart(_)) => Some(Event::Click(position)),
                    Some(Event::DragMove((start, _))) => Some(Event::DragEnd((start, position))),
                    _ => None,
                }
            }
            "pointermove" => match self.event {
                Some(Event::PointerEventStart(start)) | Some(Event::DragMove((start, _))) => {
                    Some(Event::DragMove((start, position)))
                }
                _ => Some(Event::Hover(position)),
            },
            _ => None,
        };
        if let Some(event) = &event {
            if self.tools[tool_idx].handle_event(event, &mut self.shapes) {
                let context = self.redraw_canvas();
                self.tools[tool_idx].draw_extra_shapes(&context);
            }
        }
        self.event = event;
    }

    fn get_canvas(&self) -> HtmlCanvasElement {
        self.canvas_ref.cast::<HtmlCanvasElement>().unwrap()
    }
}
