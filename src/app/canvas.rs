use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use shapes::Draw;

mod shapes {
    use wasm_bindgen::JsValue;
    use web_sys::CanvasRenderingContext2d;
    pub struct BBox {
        pub left: f64,
        pub top: f64,
        pub width: f64,
        pub height: f64,
    }

    pub trait Draw {
        fn bbox(&self) -> BBox;
        fn draw(&self, interface: CanvasRenderingContext2d);
    }

    pub struct Rectangle {
        left: f64,
        top: f64,
        width: f64,
        height: f64,
    }
    impl Rectangle {
        pub fn new(left: f64, top: f64, width: f64, height: f64) -> Self {
            Self {
                left,
                top,
                width,
                height,
            }
        }
    }

    impl Draw for Rectangle {
        fn bbox(&self) -> BBox {
            BBox {
                left: self.left,
                top: self.top,
                width: self.width,
                height: self.height,
            }
        }

        fn draw(&self, interface: CanvasRenderingContext2d) {
            interface.set_stroke_style(&JsValue::from_str("green"));
            interface.stroke_rect(self.left, self.top, self.width, self.height);
        }
    }
}
#[function_component(DrawingCanvas)]
pub fn drawing_canvas() -> Html {
    let canvas_ref = use_node_ref();
    let onclick = {
        let canvas_ref = canvas_ref.clone();
        Callback::from(move |_| {
            let rect = shapes::Rectangle::new(1.0, 1.0, 99.0, 99.0);
            let canvas: HtmlCanvasElement = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let interface: CanvasRenderingContext2d = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            rect.draw(interface);
        })
    };
    html!(
        <canvas style="flex: 1"/>
    )
}
