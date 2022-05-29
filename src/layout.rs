#[allow(dead_code)]
pub enum LayoutTypes {
    Horizontal,
    Vertical,
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct LayoutBox {
    pub width: i32,
    pub height: i32,
    pub layout_type: LayoutTypes,
    pub position: Position,
}
