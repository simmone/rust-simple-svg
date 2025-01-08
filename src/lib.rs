//! A library to generate SVG format file
//!
//! This is rewrite from Racket library: racket-simple-svg

pub use crate::defines::group::Group;
pub use crate::defines::rect::Rect;
pub use crate::defines::shape::Shape;
pub use crate::defines::sstyle::Sstyle;
pub use crate::defines::svg::Svg;
pub use crate::defines::widget::Widget;

use crate::defines::svg::BACKGROUND_GROUP_ID;

pub mod defines;

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
    svg_out_str.push_str("    >\n");

    let mut background_sstyle = None;
    if svg.background.is_some() {
        let rect_id = svg.add_shape(Shape::Rect(Rect::new(svg.width, svg.height)));
        background_sstyle = Some(Sstyle::new());
        background_sstyle.unwrap().fill = svg.background.clone();

        let mut group = Group::new();
        group.place_widget(Widget {
            shape_id: rect_id,
            style: Some(&background_sstyle.unwrap()),
            ..Default::default()
        });

        svg.add_name_group(BACKGROUND_GROUP_ID.to_string(), group);
    }

    svg_out_str.push_str(&svg.flush_data());

    svg_out_str.push_str("</svg>\n");

    svg_out_str
}
