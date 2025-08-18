use std::cell::RefCell;

use hashbrown::HashMap;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use serde::{Deserialize, Serialize};

use crate::types::colors::{BackgroundColor, Color};
use crate::types::events::Point;
use crate::types::ids::Id;
use crate::types::tools::shape_tool::ShapeToolDetails;
use crate::types::version::Version;

#[derive(PartialEq, Clone, Debug, Default, Deserialize, Serialize)]
pub struct BBox {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

impl BBox {
    pub fn from_corner((x1, y1): &(f64, f64), (x, y): &(f64, f64)) -> Self {
        Self {
            left: x.min(*x1),
            top: y.min(*y1),
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

    pub fn contains(&self, point: &Point, margin: f64) -> bool {
        let (x, y) = *point;
        self.top <= y + margin
            && y - margin <= self.bottom()
            && self.left <= x + margin
            && x - margin <= self.right()
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

pub type Drawable = Box<dyn Draw>;

pub trait Draw {
    fn new(bbox: &BBox) -> Self
    where
        Self: Sized;
    fn bbox(&self) -> BBox;
    fn draw(&self, context: &CanvasRenderingContext2d);
    fn isin(&self, bbox: &BBox) -> bool {
        self.bbox().in_(bbox)
    }
    fn contains(&self, point: &Point, margin: f64) -> bool {
        self.bbox().contains(point, margin)
    }
}

#[derive(Default, Clone)]
pub struct Rectangle {
    left: f64,
    top: f64,
    width: f64,
    height: f64,
}

impl Draw for Rectangle {
    fn new(bbox: &BBox) -> Self {
        Self {
            left: bbox.left,
            top: bbox.top,
            width: bbox.width,
            height: bbox.height,
        }
    }

    fn bbox(&self) -> BBox {
        BBox {
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
        }
    }

    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.rect(self.left, self.top, self.width, self.height);
    }
}

impl ShapeToolDetails for Rectangle {
    fn shape_type() -> ShapeType {
        ShapeType::Rectangle
    }

    fn button_icon(&self) -> &'static str {
        "ti-square"
    }

    fn button_title(&self) -> &'static str {
        "Rectangle drawing tool."
    }
}

#[derive(Clone, Default)]
pub struct Selection(Rectangle);

impl Draw for Selection {
    fn new(bbox: &BBox) -> Self {
        Self(Rectangle::new(bbox))
    }

    fn bbox(&self) -> BBox {
        self.0.bbox()
    }

    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.set_stroke_style_str("blue");
        let dashes = web_sys::js_sys::Array::new();
        dashes.push(&JsValue::from_f64(5.0));
        context.set_line_dash(&dashes).unwrap();
        context.stroke_rect(self.0.left, self.0.top, self.0.width, self.0.height);
    }
}

#[derive(Default, Clone)]
pub struct Ellipse {
    center_x: f64,
    center_y: f64,
    radius_x: f64,
    radius_y: f64,
}

impl Draw for Ellipse {
    fn new(bbox: &BBox) -> Self {
        let radius_x = bbox.width / 2.0;
        let radius_y = bbox.height / 2.0;
        let center_x = bbox.left + radius_x;
        let center_y = bbox.top + radius_y;
        Self {
            radius_x,
            radius_y,
            center_x,
            center_y,
        }
    }

    fn bbox(&self) -> BBox {
        BBox {
            left: self.center_x - self.radius_x,
            top: self.center_y - self.radius_y,
            width: self.radius_x + self.radius_x,
            height: self.radius_y + self.radius_y,
        }
    }

    fn draw(&self, context: &CanvasRenderingContext2d) {
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

    fn contains(&self, point: &Point, margin: f64) -> bool {
        let dx = point.0 - self.center_x;
        let dy = point.1 - self.center_y;
        let rx = self.radius_x + margin;
        let ry = self.radius_y + margin;
        if rx <= 0.0 || ry <= 0.0 {
            return false;
        }
        (dx * dx) / (rx * rx) + (dy * dy) / (ry * ry) <= 1.0
    }
}

impl ShapeToolDetails for Ellipse {
    fn shape_type() -> ShapeType {
        ShapeType::Ellipse
    }

    fn button_icon(&self) -> &'static str {
        "ti-circle"
    }

    fn button_title(&self) -> &'static str {
        "Ellipse drawing tool."
    }
}

#[derive(Default, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub enum ShapeType {
    #[default]
    Rectangle,
    Ellipse,
    Selection,
}

impl ShapeType {
    pub fn get_drawable(&self, bbox: &BBox) -> Drawable {
        match self {
            Self::Ellipse => Box::new(Ellipse::new(bbox)),
            Self::Selection => Box::new(Selection::new(bbox)),
            Self::Rectangle => Box::new(Rectangle::new(bbox)),
        }
    }
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Shape {
    bbox: BBox,
    name: ShapeType,
    id: Id,
    version: Version,
    color: Color,
    bg_color: Option<BackgroundColor>,
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) && self.version.eq(&other.version)
    }
}

impl Shape {
    pub fn new(
        bbox: &BBox,
        drawable: ShapeType,
        color: Color,
        bg_color: Option<BackgroundColor>,
    ) -> Self {
        Self {
            bbox: bbox.clone(),
            name: drawable,
            id: Id::default(),
            version: Version::default(),
            color,
            bg_color,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color.clone();
    }

    pub fn set_bg_color(&mut self, bg_color: Option<BackgroundColor>) {
        self.bg_color = bg_color.clone();
    }

    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub fn get_version(&self) -> &Version {
        &self.version
    }

    pub fn bbox(&self) -> BBox {
        self.bbox.clone()
    }

    pub fn get_drawable(&self) -> Drawable {
        self.name.get_drawable(&self.bbox)
    }

    pub fn isin(&self, bbox: &BBox) -> bool {
        self.get_drawable().isin(bbox)
    }

    pub fn contains(&self, point: &Point, margin: f64) -> bool {
        self.get_drawable().contains(point, margin)
    }

    // pub fn resize_to_bbox(&mut self, bbox: &BBox) -> bool {
    //     if &self.bbox() != bbox {
    //         self.bbox = bbox.clone();
    //         self.version.increment();
    //         true
    //     } else {
    //         false
    //     }
    // }
}

impl Eq for Shape {}

#[derive(Default)]
pub struct ShapeCache(RefCell<HashMap<Id, (Version, Drawable)>>);

impl ShapeCache {
    pub fn draw_from_cache(&self, shape: &Shape, context: &CanvasRenderingContext2d) {
        let mut binding = self.0.borrow_mut();
        let entry = &binding
            .entry(shape.get_id().clone())
            .and_modify(|(version, drawable)| {
                if shape.version.ne(version) {
                    *version = shape.get_version().clone();
                    *drawable = shape.get_drawable();
                }
            })
            .or_insert_with(|| (shape.get_version().clone(), shape.get_drawable()))
            .1;
        if let Some(ref color) = shape.bg_color {
            context.set_fill_style_str(&color.to_string());
            entry.draw(context);
            context.fill();
        }
        context.set_stroke_style_str(&shape.color.to_string());
        context.set_line_width(1.5);
        entry.draw(context);
        context.stroke();
    }
}
