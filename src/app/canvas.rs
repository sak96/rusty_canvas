use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use super::shapes::{BBox, Draw, Rectangle};

fn refresh_canvas(canvas: HtmlCanvasElement, shapes: &[Box<dyn Draw>]) {
    canvas.set_width(canvas.client_width().abs_diff(0));
    canvas.set_height(canvas.client_height().abs_diff(0));
    let interface: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    interface.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    for shape in shapes {
        shape.draw(&interface);
    }
}

fn get_event_canvas_postion(canvas: HtmlCanvasElement, event: MouseEvent) -> (f64, f64) {
    let rect = canvas.get_bounding_client_rect();
    let x = (event.client_x() as f64 - rect.left()) * (canvas.width() as f64 / rect.width());
    let y = (event.client_y() as f64 - rect.top()) * (canvas.height() as f64 / rect.height());
    (x, y)
}

#[function_component(DrawingCanvas)]
pub fn drawing_canvas() -> Html {
    let canvas_ref = use_node_ref();
    let shapes = use_mut_ref(Vec::<Box<dyn Draw>>::new);
    let new_shape_start = use_mut_ref(Option::<(f64, f64)>::default);
    let onmousedown = {
        let canvas_ref = canvas_ref.clone();
        let shapes = shapes.clone();
        let new_shape_start = new_shape_start.clone();
        Callback::from(move |event: MouseEvent| {
            let canvas: HtmlCanvasElement = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let (left, top) = get_event_canvas_postion(canvas.clone(), event);
            new_shape_start.borrow_mut().replace((left, top));
            shapes
                .borrow_mut()
                .push(Box::new(Rectangle::new(left, top, 0.0, 0.0)));
            refresh_canvas(canvas, &shapes.borrow());
        })
    };
    let onmousemove = {
        let canvas_ref = canvas_ref.clone();
        let shapes = shapes.clone();
        let new_shape_start = new_shape_start.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(start) = *new_shape_start.borrow() {
                let canvas: HtmlCanvasElement = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                let end = get_event_canvas_postion(canvas.clone(), event);
                let mut shapes = shapes.borrow_mut();
                if let Some(shape) = shapes.last_mut() {
                    shape.resize_to_bbox(BBox::from_corner(start, end));
                    refresh_canvas(canvas, &shapes);
                }
            }
        })
    };
    let onmouseup = {
        let canvas_ref = canvas_ref.clone();
        let shapes = shapes.clone();
        let new_shape_start = new_shape_start.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(start) = new_shape_start.borrow_mut().take() {
                let canvas: HtmlCanvasElement = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                let end = get_event_canvas_postion(canvas.clone(), event);
                let mut shapes = shapes.borrow_mut();
                if let Some(shape) = shapes.last_mut() {
                    shape.resize_to_bbox(BBox::from_corner(start, end));
                    refresh_canvas(canvas, &shapes);
                }
            }
        })
    };
    let onresize = {
        let canvas_ref = canvas_ref.clone();
        let shapes = shapes.clone();
        Callback::from(move |_| {
            let canvas: HtmlCanvasElement = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            refresh_canvas(canvas, &shapes.borrow());
        })
    };
    {
        let canvas_ref = canvas_ref.clone();
        let shapes = shapes.clone();
        use_effect(move || {
            let canvas: HtmlCanvasElement = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            refresh_canvas(canvas, &shapes.borrow());
        })
    }

    html!(
        <canvas
            style="flex: 1"
            ref={canvas_ref}
            {onmouseup}
            {onmousemove}
            {onmousedown}
            {onresize}
            />
    )
}
