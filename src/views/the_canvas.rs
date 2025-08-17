use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::AppState;
use crate::store::shapes::Shapes;
use crate::types::events::CanvasEvent;
use crate::types::shapes::{Drawable, ShapeCache, ShapeType};
use crate::types::tools::{Tool, ToolAction};

pub struct EventHandler {
    canvas_ref: NodeRef,
    tool: Tool,
    event: Option<CanvasEvent>,
    shape: Option<Drawable>,
    shape_cache: ShapeCache,
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
            shape_cache: Default::default(),
        }
    }

    pub fn set_tool(&mut self, dispatch: Dispatch<AppState>, tool: &Tool) {
        if self.tool.ne(tool) {
            dispatch.reduce_mut(|app| {
                self.tool
                    .handle_event(&CanvasEvent::DeselectTool, &mut self.shape, app);
                self.tool = tool.clone();
                self.tool
                    .handle_event(&CanvasEvent::SelectTool, &mut self.shape, app);
            });
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
            self.shape_cache.draw_from_cache(shape, &context);
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

    pub fn handle_keyboard_event(&mut self, dispatch: Dispatch<AppState>, event: KeyboardEvent) {
        let canvas_event = CanvasEvent::KeyPress(event.key());
        dispatch.reduce_mut(|app| {
            if self.tool.handle_event(&canvas_event, &mut self.shape, app) {
                event.prevent_default();
            }
        });
        self.event = Some(canvas_event);
    }

    pub fn handle_ptr_event(&mut self, dispatch: Dispatch<AppState>, event: PointerEvent) {
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
            dispatch.reduce_mut(|app| {
                if self.tool.handle_event(canvas_event, &mut self.shape, app) {
                    event.prevent_default();
                }
            })
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

    // TODO: can this clone be avoided
    let shapes = use_selector_eq(
        |app: &AppState| app.get_shapes().clone(),
        |old, new| old.version.eq(&new.version),
    );
    let current_tool = use_selector(|app: &AppState| app.get_tool().clone());
    let current_ptr = use_selector(|app: &AppState| app.get_pointer().to_owned());

    let dispatch = use_dispatch::<AppState>();

    let on_pointer_event = {
        let event_handler = event_handler.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |event: PointerEvent| {
            // Focus canvas on mouse events; required with tabindex for key events to work
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let _ = canvas.focus();
            }
            event_handler
                .borrow_mut()
                .handle_ptr_event(dispatch.clone(), event)
        })
    };
    let onresize = {
        let shapes = shapes.clone();
        let event_handler = event_handler.clone();
        Callback::from(move |_| {
            event_handler.borrow_mut().refresh_canvas(&shapes);
        })
    };

    let on_key_down = {
        let event_handler = event_handler.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |event: KeyboardEvent| {
            event_handler
                .borrow_mut()
                .handle_keyboard_event(dispatch.clone(), event);
        })
    };

    {
        let event_handler = event_handler.clone();
        use_effect_with(shapes.clone(), move |_| {
            event_handler.borrow().refresh_canvas(&shapes);
        });
    };
    {
        let event_handler = event_handler.clone();
        let dispatch = dispatch.clone();
        use_effect_with(current_tool, move |tool| {
            event_handler.borrow_mut().set_tool(dispatch, tool);
        });
    };

    html! {
        <canvas
            style={format!("flex: 1; cursor: {current_ptr}; touch-action: none;")}
            ref={event_handler.borrow().canvas_ref.clone()}
            tabindex="0"
            onkeydown={on_key_down}
            onpointerup={on_pointer_event.clone()}
            onpointerdown={on_pointer_event.clone()}
            onpointermove={on_pointer_event.clone()}
            {onresize}
        />
    }
}
