use crate::defines::circle::Circle;
use crate::defines::ellipse::Ellipse;
use crate::defines::line::Line;
use crate::defines::polygon::Polygon;
use crate::defines::polyline::Polyline;
use crate::defines::rect::Rect;

#[derive(Clone)]
pub enum Shape {
    Rect(Rect),
    Circle(Circle),
    Ellipse(Ellipse),
    Line(Line),
    Polygon(Polygon),
    Polyline(Polyline),
}

impl Shape {
    pub fn format(&self, shape_id: String) -> String {
        match self {
            Shape::Rect(rect) => rect.format(shape_id),
            Shape::Circle(circle) => circle.format(shape_id),
            Shape::Ellipse(ellipse) => ellipse.format(shape_id),
            Shape::Line(line) => line.format(shape_id),
            Shape::Polygon(polygon) => polygon.format(shape_id),
            Shape::Polyline(polyline) => polyline.format(shape_id),
        }
    }
}
