use super::Tool;
use crate::app::shapes::{BBox, Draw, Rectangle};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Default)]
pub struct SelectTool {
    start: Option<(f64, f64)>,
    shape: Rectangle,
    shapes: Vec<Rectangle>,
}

const PADDING: f64 = 5.0;

impl SelectTool {
    fn handle_selection(
        &self,
        selection: &BBox,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) -> Vec<Rectangle> {
        let mut selections = vec![];
        for shape in shapes {
            let mut bbox = shape.bbox();
            if bbox.in_(selection) {
                bbox.add_padding(PADDING);
                let mut rect = Rectangle::default();
                rect.resize_to_bbox(&bbox);
                rect.is_selection = true;
                selections.push(rect);
            }
        }
        selections
    }
}

#[allow(unused_variables)]
impl Tool for SelectTool {
    fn button_icon(&self) -> &'static str {
        "ti-marquee"
    }

    fn button_title(&self) -> &'static str {
        "Selection tool."
    }

    fn onmousedown(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) -> bool {
        self.shape.is_selection = true;
        self.start.replace(position);
        let selected_area = BBox::from_corner(position, position);
        self.shape.resize_to_bbox(&selected_area);
        true
    }

    fn onmouseup(
        &mut self,
        position: (f64, f64),
        canvas: HtmlCanvasElement,
        shapes: &mut Vec<Box<dyn Draw>>,
    ) -> bool {
        if let Some(start) = self.start.take() {
            let selection = BBox::from_corner(start, position);
            self.shapes = self.handle_selection(&selection, shapes);
            if let Some(first) = self.shapes.first() {
                let mut bbox = first.bbox();
                for shape in &self.shapes {
                    bbox.add_bbox(&shape.bbox())
                }
                bbox.add_padding(2.0 * PADDING);
                self.shape.resize_to_bbox(&bbox);
            }
            true
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
            let selection = BBox::from_corner(start, position);
            self.shape.resize_to_bbox(&selection);
            self.shapes = self.handle_selection(&selection, shapes);
            true
        } else {
            false
        }
    }

    fn draw_extra_shapes(&self, context: &CanvasRenderingContext2d) {
        // if either some thing is selected or selection is happening
        if !self.shapes.is_empty() || self.start.is_some() {
            self.shape.draw(context);
            for shape in &self.shapes {
                shape.draw(context);
            }
        }
    }
}
