use crate::defines::rect::Rect;

pub enum Shape {
    Rect(Rect),
}

impl Shape {
    pub fn format(&self, shape_id: String) -> String {
        match self {
            Shape::Rect(rect) => rect.format(shape_id),
        }
    }
}
