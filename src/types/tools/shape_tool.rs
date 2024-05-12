use std::marker::PhantomData;

use super::ToolAction;
use crate::store::shapes::Shapes;
use crate::types::events::Event;
use crate::types::shapes::{BBox, Ellipse, Rectangle, Shape};

pub trait ShapeToolDetails {
    fn button_icon(&self) -> &'static str;
    fn button_title(&self) -> &'static str;
}

#[derive(Default, Clone)]
pub struct ShapeTool<T> {
    marker: PhantomData<T>,
}

impl<T> ToolAction for ShapeTool<T>
where
    T: Into<Shape> + ShapeToolDetails + Default,
{
    fn button_icon(&self) -> &'static str {
        ShapeToolDetails::button_icon(&T::default())
    }

    fn button_title(&self) -> &'static str {
        ShapeToolDetails::button_title(&T::default())
    }

    fn handle_event(&mut self, event: &Event, shapes: &mut Shapes) -> Option<Shape> {
        match event {
            Event::DragMove((start, end)) => {
                let mut shape = T::default().into();
                shape.resize_to_bbox(&BBox::from_corner(start, end));
                shapes.version.increment();
                Some(shape)
            }
            Event::DragEnd((start, end)) => {
                let mut shape = T::default().into();
                shape.resize_to_bbox(&BBox::from_corner(start, end));
                shapes.shapes.push(shape);
                None
            }
            _ => None,
        }
    }
}
pub type RectangleTool = ShapeTool<Rectangle>;
pub type EllipseTool = ShapeTool<Ellipse>;
