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
use crate::defines::svg::DEFAULT_GROUP_ID;

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

    if svg.background.is_some() {
        let rect_id = svg.add_shape(Shape::Rect(Rect::new(svg.width, svg.height)));

        let mut background_sstyle = Sstyle::new();
        background_sstyle.fill = Some(svg.background.as_ref().unwrap().clone());

        let mut group = Group::new();
        group.place_widget(Widget {
            shape_id: rect_id,
            style: Some(background_sstyle),
            ..Default::default()
        });

        svg.add_name_group(BACKGROUND_GROUP_ID.to_string(), group);
    }

    if svg.group_define_map.contains_key(DEFAULT_GROUP_ID) {
        let mut widget_list = svg.group_define_map.get(DEFAULT_GROUP_ID).unwrap();
        if widget_list.length > 0 {
            svg.group_show_list.push(
        }
    }

    svg_out_str.push_str(&svg.flush_data());

    svg_out_str.push_str("</svg>\n");

    svg_out_str
}

              (let ([default_not_null (> (length (GROUP-widget_list (hash-ref (SVG-group_define_map (*SVG*)) DEFAULT_GROUP_ID))) 0)])
                (set-SVG-group_show_list!
                 (*SVG*)
                 (append
                  (if background
                      (list (cons BACKGROUND_GROUP_ID (cons 0 0)))
                      '())
                  (if default_not_null
                      (list (cons DEFAULT_GROUP_ID (cons 0 0)))
                      '())))))

