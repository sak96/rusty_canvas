use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

#[derive(PartialEq, Debug)]
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

    fn right(&self) -> f64 {
        self.left + self.width
    }

    fn bottom(&self) -> f64 {
        self.top + self.height
    }

    #[must_use]
    pub fn in_(&self, bbox: &BBox) -> bool {
        self.left >= bbox.left
            && self.top >= bbox.top
            && self.right() <= bbox.right()
            && self.bottom() <= bbox.bottom()
    }

    pub fn add_padding(&mut self, padding: f64) {
        self.left -= padding;
        self.top -= padding;
        self.width += padding + padding;
        self.height += padding + padding;
    }

    pub fn add_bbox(&mut self, bbox: &BBox) {
        let left = self.left.min(bbox.left);
        let top = self.top.min(bbox.top);
        self.width = self.right().max(bbox.right()) - left;
        self.height = self.bottom().max(bbox.bottom()) - top;
        self.left = left;
        self.top = top;
    }
}

pub trait Draw {
    fn bbox(&self) -> BBox;
    fn draw(&self, context: &CanvasRenderingContext2d);
    fn _resize_to_bbox(&mut self, bbox: &BBox);

    fn resize_to_bbox(&mut self, bbox: &BBox) -> bool {
        if &self.bbox() != bbox {
            self._resize_to_bbox(bbox);
            true
        } else {
            false
        }
    }
}

#[derive(Default)]
pub struct Rectangle {
    left: f64,
    top: f64,
    width: f64,
    height: f64,
    pub is_selection: bool,
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

    fn draw(&self, context: &CanvasRenderingContext2d) {
        if self.is_selection {
            context.set_stroke_style(&JsValue::from_str("blue"));
        } else {
            context.set_stroke_style(&JsValue::from_str("green"));
        }
        context.stroke_rect(self.left, self.top, self.width, self.height);
    }

    fn _resize_to_bbox(&mut self, bbox: &BBox) {
        self.left = bbox.left;
        self.top = bbox.top;
        self.width = bbox.width;
        self.height = bbox.height;
    }
}

#[derive(Default)]
pub struct Ellipse {
    center_x: f64,
    center_y: f64,
    radius_x: f64,
    radius_y: f64,
}

impl Draw for Ellipse {
    fn bbox(&self) -> BBox {
        BBox {
            left: self.center_x - self.radius_x,
            top: self.center_y - self.radius_y,
            width: self.radius_x + self.radius_x,
            height: self.radius_y + self.radius_y,
        }
    }

    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.set_stroke_style(&JsValue::from_str("red"));
        context.begin_path();
        context
            .ellipse(
                self.center_x,
                self.center_y,
                self.radius_x,
                self.radius_y,
                0.0,
                0.0,
                std::f64::consts::TAU,
            )
            .unwrap();
        context.stroke();
    }

    fn _resize_to_bbox(&mut self, bbox: &BBox) {
        self.radius_x = bbox.width / 2.0;
        self.radius_y = bbox.height / 2.0;
        self.center_x = bbox.left + self.radius_x;
        self.center_y = bbox.top + self.radius_y;
    }
}
