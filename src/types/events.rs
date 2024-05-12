pub type Point = (f64, f64);

#[non_exhaustive]
pub enum Event {
    PointerEventStart(Point),
    Hover(Point),
    DragMove((Point, Point)),
    DragEnd((Point, Point)),
    Click(Point),
}
