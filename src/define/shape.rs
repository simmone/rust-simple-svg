//! Shape is basic element of svg.
//! Define some kind of shape, give it a style, place it into group at somewhere.

pub mod arrow;
pub mod circle;
pub mod ellipse;
pub mod filter;
pub mod gradient;
pub mod line;
pub mod marker;
pub mod path;
pub mod polygon;
pub mod polyline;
pub mod rect;
pub mod text;

use crate::define::shape::arrow::Arrow;
use crate::define::shape::circle::Circle;
use crate::define::shape::ellipse::Ellipse;
use crate::define::shape::filter::Filter;
use crate::define::shape::gradient::LinearGradient;
use crate::define::shape::gradient::RadialGradient;
use crate::define::shape::line::Line;
use crate::define::shape::marker::Marker;
use crate::define::shape::path::Path;
use crate::define::shape::polygon::Polygon;
use crate::define::shape::polyline::Polyline;
use crate::define::shape::rect::Rect;
use crate::define::shape::text::Text;

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

    pub fn unique(&self) -> String {
        match self {
            Shape::Rect(rect) => rect.unique(),
            Shape::Circle(circle) => circle.unique(),
            Shape::Ellipse(ellipse) => ellipse.unique(),
            Shape::Line(line) => line.unique(),
            Shape::Polygon(polygon) => polygon.unique(),
            Shape::Polyline(polyline) => polyline.unique(),
            Shape::Filter(filter) => filter.unique(),
            Shape::LinearGradient(linear_gradient) => linear_gradient.unique(),
            Shape::RadialGradient(radial_gradient) => radial_gradient.unique(),
            Shape::Path(path) => path.unique(),
            Shape::Text(text) => text.unique(),
            Shape::Marker(marker) => marker.unique(),
            Shape::Arrow(arrow) => arrow.unique(),
        }
    }
}
