use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

#[derive(PartialEq)]
pub struct BBox {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

impl BBox {
    pub fn from_corner((x1, y1): (f64, f64), (x, y): (f64, f64)) -> Self {
        Self {
            left: x.min(x1),
            top: y.min(y1),
            width: (x - x1).abs(),
            height: (y - y1).abs(),
        }
    }
}

pub trait Draw {
    fn bbox(&self) -> BBox;
    fn draw(&self, interface: &CanvasRenderingContext2d);
    fn resize_to_bbox(&mut self, bbox: BBox) -> bool;
}

#[derive(Default)]
pub struct Rectangle {
    left: f64,
    top: f64,
    width: f64,
    height: f64,
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

    fn draw(&self, interface: &CanvasRenderingContext2d) {
        interface.set_stroke_style(&JsValue::from_str("green"));
        interface.stroke_rect(self.left, self.top, self.width, self.height);
    }

    fn resize_to_bbox(&mut self, bbox: BBox) -> bool {
        if self.bbox() != bbox {
            self.left = bbox.left;
            self.top = bbox.top;
            self.width = bbox.width;
            self.height = bbox.height;
            true
        } else {
            false
        }
    }
}
