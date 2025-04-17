//! A library to generate SVG format file
//!
//! This is rewrite from Racket library: racket-simple-svg

#[doc(hidden)]
pub use crate::define::group::Group;
#[doc(hidden)]
pub use crate::define::shape::arrow::Arrow;
#[doc(hidden)]
pub use crate::define::shape::circle::Circle;
#[doc(hidden)]
pub use crate::define::shape::ellipse::Ellipse;
#[doc(hidden)]
pub use crate::define::shape::filter::Filter;
#[doc(hidden)]
pub use crate::define::shape::gradient::LinearGradient;
#[doc(hidden)]
pub use crate::define::shape::gradient::RadialGradient;
#[doc(hidden)]
pub use crate::define::shape::line::Line;
#[doc(hidden)]
pub use crate::define::shape::marker::{Marker, MarkerType};
#[doc(hidden)]
pub use crate::define::shape::path::{ArcDirection, Path};
#[doc(hidden)]
pub use crate::define::shape::polygon::Polygon;
#[doc(hidden)]
pub use crate::define::shape::polyline::Polyline;
#[doc(hidden)]
pub use crate::define::shape::rect::Rect;
#[doc(hidden)]
pub use crate::define::shape::text::{Text, TextDecoration};
#[doc(hidden)]
pub use crate::define::shape::Shape;
#[doc(hidden)]
pub use crate::define::sstyle::{FillRule, LineCap, LineJoin, Sstyle};
#[doc(hidden)]
pub use crate::define::svg::Svg;
#[doc(hidden)]
pub use crate::define::widget::Widget;
#[doc(hidden)]
pub use crate::gadget::table::Table;

use crate::define::svg::BACKGROUND_GROUP_ID;
use crate::define::svg::DEFAULT_GROUP_ID;

#[doc(hidden)]
pub mod define;
#[doc(hidden)]
pub mod gadget;

pub fn svg_out(mut svg: Svg) -> String {
    let mut svg_out_str = String::new();

    svg_out_str.push_str("<svg\n");
    svg_out_str.push_str("    version=\"1.1\"\n");
    svg_out_str.push_str("    xmlns=\"http://www.w3.org/2000/svg\"\n");
    svg_out_str.push_str("    xmlns:xlink=\"http://www.w3.org/1999/xlink\"\n");
    svg_out_str.push_str(&format!(
        "    width=\"{}\" height=\"{}\"\n",
        svg.width.to_string(),
        svg.height.to_string()
    ));

    if svg.view_box.is_some() {
        svg_out_str.push_str(&format!(
            "    viewBox=\"{} {} {} {}\"\n",
            svg.view_box.as_ref().unwrap().min_x,
            svg.view_box.as_ref().unwrap().min_y,
            svg.view_box.as_ref().unwrap().width,
            svg.view_box.as_ref().unwrap().height
        ))
    }

    svg_out_str.push_str("    >\n");

    if svg.background != None {
        svg.group_show_list
            .push((BACKGROUND_GROUP_ID.to_string(), (0.0, 0.0)));
    }

    if svg.group_define_map.contains_key(DEFAULT_GROUP_ID) {
        let widget_list = &svg
            .group_define_map
            .get(DEFAULT_GROUP_ID)
            .unwrap()
            .widget_list;

        if widget_list.len() > 0 {
            svg.group_show_list
                .push((DEFAULT_GROUP_ID.to_string(), (0.0, 0.0)));
        }
    }

    svg_out_str.push_str(&svg.flush_data());

    svg_out_str.push_str("</svg>\n");

    svg_out_str
}
