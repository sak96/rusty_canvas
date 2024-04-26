use super::Tool;
use crate::app::shapes::{BBox, Draw};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct ShapeTool<T: Draw + Default> {
    start: Option<(f64, f64)>,
    icon: &'static str,
    title: &'static str,
    shape: Box<T>,
}

impl<T: Draw + Default> ShapeTool<T> {
    pub fn new(icon: &'static str, title: &'static str) -> Self {
        Self {
            icon,
            title,
            shape: Default::default(),
            start: None,
        }
    }
}

#[allow(unused_variables)]
impl<T: Draw + Default + 'static> Tool for ShapeTool<T> {
    fn button_icon(&self) -> &'static str {
        self.icon
    }

    fn button_title(&self) -> &'static str {
        self.title
    }

    fn onmousedown(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) -> bool {
        self.start.replace(position);
        self.shape
            .resize_to_bbox(&BBox::from_corner(position, position));
        true
    }

    fn onmouseup(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) -> bool {
        if let Some(start) = self.start.take() {
            let mut shape = Box::<T>::default();
            let changed = shape.resize_to_bbox(&BBox::from_corner(start, position));
            shapes.push(shape);
            changed
        } else {
            false
        }
    }

    fn onmousemove(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) -> bool {
        if let Some(start) = self.start {
            self.shape
                .resize_to_bbox(&BBox::from_corner(start, position))
        } else {
            false
        }
    }

    fn draw_extra_shapes(&self, context: &CanvasRenderingContext2d) {
        if self.start.is_some() {
            self.shape.draw(context);
        }
    }
}
