pub type Point = (f64, f64);

#[non_exhaustive]
pub enum CanvasEvent {
    PointerEventStart(Point),
    Hover(Point),
    DragMove((Point, Point)),
    DragEnd((Point, Point)),
    Click(Point),
    DeselectTool,
    SelectTool,
}
