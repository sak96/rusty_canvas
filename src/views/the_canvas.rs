use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use yew::NodeRef;
use yew::PointerEvent;
use yewdux::prelude::*;

use crate::store::shapes::Shapes;
use crate::types::events::Event;
use crate::types::shapes::{Selection, Shape};
use crate::types::tools::ToolBar;

pub struct EventHandler {
    canvas_ref: NodeRef,
    tools: ToolBar,
    event: Option<Event>,
    shape: Option<Shape>,
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
            tools: ToolBar::new(),
            event: Default::default(),
            shape: Default::default(),
        }
    }

    pub fn toolbar(&self) -> &ToolBar {
        &self.tools
    }

    pub fn toolbar_mut(&mut self) -> &mut ToolBar {
        &mut self.tools
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
                let mut sel_shape: Shape = Selection::default().into();
                sel_shape.resize_to_bbox(&padded_bbox);
                selections.push(sel_shape);
            }
        }
        for shape in &selections {
            shape.draw(&context);
        }
        match selections.split_first() {
            Some((first, [])) => first.draw(&context),
            Some((first, rest)) => {
                let mut group = first.bbox();
                for shape in rest {
                    group.add_bbox(&shape.bbox());
                    shape.draw(&context);
                }
                let mut sel_shape: Shape = Selection::default().into();
                sel_shape.resize_to_bbox(&group);
                sel_shape.draw(&context);
            }
            None => {}
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

    pub fn handle_ptr_event(&mut self, event: PointerEvent, shapes: &mut Shapes) {
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
            self.shape = self.tools.handle_event(event, shapes);
        }
        self.event = event;
    }

    fn get_canvas(&self) -> HtmlCanvasElement {
        self.canvas_ref.cast::<HtmlCanvasElement>().unwrap()
    }
}

#[derive(Properties, PartialEq)]
pub struct TheCanvasProps {
    pub event_handler: Rc<RefCell<EventHandler>>,
}

#[function_component(TheCanvas)]
pub fn the_canvas(TheCanvasProps { event_handler }: &TheCanvasProps) -> Html {
    let (shapes, shapes_dispatch) = use_store::<Shapes>();
    let on_pointer_event = {
        let event_handler = event_handler.clone();
        shapes_dispatch.reduce_mut_callback_with(move |shapes, event: PointerEvent| {
            event_handler.borrow_mut().handle_ptr_event(event, shapes)
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

    html! {
        <canvas
            style="flex: 1"
            ref={event_handler.borrow().canvas_ref.clone()}
            onpointerup={on_pointer_event.clone()}
            onpointerdown={on_pointer_event.clone()}
            onpointermove={on_pointer_event.clone()}
            {onresize}
        />
    }
}
