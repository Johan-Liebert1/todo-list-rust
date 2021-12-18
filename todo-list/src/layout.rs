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
    pub children: Vec<LayoutBox>,
    pub position: Position,
}

impl LayoutBox {
    pub fn new(width: i32, height: i32, layout_type: LayoutTypes, position: Position) -> Self {
        LayoutBox {
            width,
            height,
            layout_type,
            position,
            children: Vec::new(),
        }
    }
}
