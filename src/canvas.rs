use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

#[function_component(DrawingCanvas)]
pub fn drawing_canvas() -> Html {
    let canvas_ref = use_node_ref();
    let onclick = {
        let canvas_ref = canvas_ref.clone();
        Callback::from(move |_| {
            let canvas: HtmlCanvasElement = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let interface: CanvasRenderingContext2d = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            interface.set_stroke_style(&JsValue::from_str("green"));
            interface.stroke_rect(1.0, 1.0, 99.0, 99.0);
        })
    };
    html!(
        <canvas style="display: block;width: 100vw; height: 100vh" onclick={onclick} ref={ canvas_ref.clone() } />
    )
}
