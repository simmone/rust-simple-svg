use crate::define::arrow::Arrow;
use crate::define::circle::Circle;
use crate::define::ellipse::Ellipse;
use crate::define::filter::Filter;
use crate::define::gradient::LinearGradient;
use crate::define::gradient::RadialGradient;
use crate::define::line::Line;
use crate::define::marker::Marker;
use crate::define::path::Path;
use crate::define::polygon::Polygon;
use crate::define::polyline::Polyline;
use crate::define::rect::Rect;
use crate::define::text::Text;

#[derive(Clone)]
pub enum Shape {
    Rect(Rect),
    Circle(Circle),
    Ellipse(Ellipse),
    Line(Line),
    Polygon(Polygon),
    Polyline(Polyline),
    Filter(Filter),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Path(Path),
    Text(Text),
    Marker(Marker),
    Arrow(Arrow),
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
            Shape::Filter(filter) => filter.format(shape_id),
            Shape::LinearGradient(linear_gradient) => linear_gradient.format(shape_id),
            Shape::RadialGradient(radial_gradient) => radial_gradient.format(shape_id),
            Shape::Path(path) => path.format(shape_id),
            Shape::Text(text) => text.format(shape_id),
            Shape::Marker(marker) => marker.format(shape_id),
            Shape::Arrow(arrow) => arrow.format(shape_id),
        }
    }
}
