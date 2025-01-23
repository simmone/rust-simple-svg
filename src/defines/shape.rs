use crate::defines::rect::Rect;
use crate::defines::circle::Circle;

#[derive(Clone)]
pub enum Shape {
    Rect(Rect),
    Circle(Circle),
}

impl Shape {
    pub fn format(&self, shape_id: String) -> String {
        match self {
            Shape::Rect(rect) => rect.format(shape_id),
            Shape::Circle(circle) => circle.format(shape_id),
        }
    }
}
