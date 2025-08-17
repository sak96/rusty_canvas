pub type Point = (f64, f64);

#[non_exhaustive]
pub enum CanvasEvent {
    PointerEventStart(Point),
    /// currently not used any where
    #[allow(dead_code)]
    Hover(Point),
    DragMove((Point, Point)),
    DragEnd((Point, Point)),
    /// currently not used any where
    #[allow(dead_code)]
    Click(Point),
    KeyPress(String),
    DeselectTool,
    SelectTool,
}
