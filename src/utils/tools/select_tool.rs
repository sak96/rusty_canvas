use super::ToolAction;
use crate::store::shapes::{BBox, Draw, Rectangle, Shape};
use crate::utils::events::Event;
use web_sys::CanvasRenderingContext2d;

#[derive(Default)]
pub struct SelectTool {
    selected_area: Rectangle,
    shapes: Vec<Rectangle>,
}

const PADDING: f64 = 5.0;

impl SelectTool {
    fn update_selection(&mut self, selection: &BBox, shapes: &mut Vec<Shape>) {
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
        self.selected_area.resize_to_bbox(selection);
        self.shapes = selections;
    }

    fn finalize_selection(&mut self, selection: &BBox, shapes: &mut Vec<Shape>) {
        self.update_selection(selection, shapes);
        match self.shapes.split_first() {
            None => {
                self.selected_area.resize_to_bbox(&BBox::default());
            }
            Some((first, rest)) => {
                let mut bbox = first.bbox();
                for shape in rest {
                    bbox.add_bbox(&shape.bbox())
                }
                bbox.add_padding(2.0 * PADDING);
                self.selected_area.resize_to_bbox(&bbox);
            }
        }
    }
}

impl ToolAction for SelectTool {
    fn button_icon(&self) -> &'static str {
        "ti-marquee"
    }

    fn button_title(&self) -> &'static str {
        "Selection tool."
    }

    fn draw_extra_shapes(&self, context: &CanvasRenderingContext2d) {
        let bbox = self.selected_area.bbox();
        if bbox.width != 0.0 || bbox.height != 0.0 {
            self.selected_area.draw(context);
            for shape in &self.shapes {
                shape.draw(context);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, shapes: &mut Vec<Shape>) -> bool {
        match event {
            Event::PointerEventStart(start) => {
                let changed = !self.shapes.is_empty();
                self.selected_area.is_selection = true;
                self.selected_area
                    .resize_to_bbox(&BBox::from_corner(start, start));
                self.shapes.clear();
                changed
            }
            Event::DragMove((start, end)) => {
                let selection = BBox::from_corner(start, end);
                self.update_selection(&selection, shapes);
                true
            }
            Event::DragEnd((start, end)) => {
                let selection = BBox::from_corner(start, end);
                self.finalize_selection(&selection, shapes);
                true
            }
            _ => false,
        }
    }
}
