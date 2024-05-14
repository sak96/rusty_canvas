use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::shapes::Shapes;
use crate::store::tools::Tools;
use crate::types::events::CanvasEvent;
use crate::types::shapes::{Drawable, ShapeType};
use crate::types::tools::{Tool, ToolAction};

pub struct EventHandler {
    canvas_ref: NodeRef,
    tool: Tool,
    event: Option<CanvasEvent>,
    shape: Option<Drawable>,
}

impl PartialEq for EventHandler {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

const PADDING: f64 = 5.0;

impl EventHandler {
    pub fn new(canvas_ref: NodeRef) -> Self {
        Self {
            canvas_ref,
            tool: Tool::default(),
            event: Default::default(),
            shape: Default::default(),
        }
    }

    pub fn set_tool(&mut self, tools: &mut Tools, shapes: &mut Shapes) {
        if self.tool.ne(&tools.tool) {
            self.tool
                .handle_event(&CanvasEvent::DeselectTool, tools, &mut self.shape, shapes);
            self.tool = tools.tool.clone();
            self.tool
                .handle_event(&CanvasEvent::SelectTool, tools, &mut self.shape, shapes);
        }
    }

    fn refresh_canvas(&self, shapes: &Shapes) {
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
        let mut selections = vec![];
        for shape in &shapes.shapes {
            shape.draw(&context);
            if shapes.selected_shapes.contains(shape.get_id()) {
                let mut padded_bbox = shape.bbox();
                padded_bbox.add_padding(PADDING);
                selections.push(ShapeType::Selection.get_drawable(&padded_bbox));
            }
        }
        for shape in &selections {
            shape.draw(&context);
        }
        if let Some((first, rest)) = selections.split_first() {
            let mut group = first.bbox();
            for shape in rest {
                group.add_bbox(&shape.bbox());
                shape.draw(&context);
            }
            ShapeType::Selection.get_drawable(&group).draw(&context);
        }
        if let Some(shape) = &self.shape {
            shape.draw(&context);
        }
    }

    fn get_event_canvas_postion(canvas: &HtmlCanvasElement, event: &PointerEvent) -> (f64, f64) {
        let rect = canvas.get_bounding_client_rect();
        let x = (event.client_x() as f64 - rect.left()) * (canvas.width() as f64 / rect.width());
        let y = (event.client_y() as f64 - rect.top()) * (canvas.height() as f64 / rect.height());
        (x, y)
    }

    pub fn handle_ptr_event(
        &mut self,
        event: PointerEvent,
        tools: &mut Tools,
        shapes: &mut Shapes,
    ) {
        let canvas = self.get_canvas();
        let position = Self::get_event_canvas_postion(&canvas, &event);
        let canvas_event = match event.type_().as_str() {
            "pointerdown" => {
                canvas.set_pointer_capture(event.pointer_id()).unwrap();
                Some(CanvasEvent::PointerEventStart(position))
            }
            "pointerup" => {
                canvas.release_pointer_capture(event.pointer_id()).unwrap();
                match self.event {
                    Some(CanvasEvent::PointerEventStart(_)) => Some(CanvasEvent::Click(position)),
                    Some(CanvasEvent::DragMove((start, _))) => {
                        Some(CanvasEvent::DragEnd((start, position)))
                    }
                    _ => None,
                }
            }
            "pointermove" => match self.event {
                Some(CanvasEvent::PointerEventStart(start))
                | Some(CanvasEvent::DragMove((start, _))) => {
                    Some(CanvasEvent::DragMove((start, position)))
                }
                _ => Some(CanvasEvent::Hover(position)),
            },
            _ => None,
        };
        if let Some(canvas_event) = &canvas_event {
            if self
                .tool
                .handle_event(canvas_event, tools, &mut self.shape, shapes)
            {
                event.prevent_default();
            }
        }
        self.event = canvas_event;
    }

    fn get_canvas(&self) -> HtmlCanvasElement {
        self.canvas_ref.cast::<HtmlCanvasElement>().unwrap()
    }
}

#[function_component(TheCanvas)]
pub fn the_canvas() -> Html {
    let canvas_ref = use_node_ref();
    let event_handler = use_mut_ref(|| EventHandler::new(canvas_ref.clone()));
    let (shapes, shapes_dispatch) = use_store::<Shapes>();
    let tool_dispatch = use_dispatch::<Tools>();
    let on_pointer_event = {
        let event_handler = event_handler.clone();
        let tool_dispatch = tool_dispatch.clone();
        shapes_dispatch.reduce_mut_callback_with(move |shapes, event: PointerEvent| {
            tool_dispatch.reduce_mut(|tools| {
                event_handler
                    .borrow_mut()
                    .handle_ptr_event(event, tools, shapes)
            })
        })
    };
    let onresize = {
        let shapes = shapes.clone();
        let event_handler = event_handler.clone();
        Callback::from(move |_| {
            event_handler.borrow().refresh_canvas(&shapes);
        })
    };
    {
        let event_handler = event_handler.clone();
        use_effect_with(shapes.clone(), move |_| {
            event_handler.borrow().refresh_canvas(&shapes);
        });
    };
    let current_tool = use_selector(|tools: &Tools| tools.tool.clone());
    let current_ptr = use_selector(|tools: &Tools| tools.pointer.clone());
    {
        let event_handler = event_handler.clone();
        use_effect_with(
            (current_tool.clone(), shapes_dispatch.clone()),
            move |(_tool, shapes_dispatch)| {
                shapes_dispatch.reduce_mut(|shapes| {
                    tool_dispatch.reduce_mut(|tools| {
                        event_handler.borrow_mut().set_tool(tools, shapes);
                    })
                })
            },
        );
    };

    html! {
        <canvas
            style={format!("flex: 1; cursor: {current_ptr}; touch-action: none;")}
            ref={event_handler.borrow().canvas_ref.clone()}
            onpointerup={on_pointer_event.clone()}
            onpointerdown={on_pointer_event.clone()}
            onpointermove={on_pointer_event.clone()}
            {onresize}
        />
    }
}
