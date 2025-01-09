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
        svg.group_show_list.push(BACKGROUND_GROUP_ID.to_string());
    }
    
    let mut group_ids: Vec<String> = svg.group_define_map.into_keys().collect();
    group_ids.remove(&(DEFAULT_GROUP_ID.to_string()));
    group_ids.sort();
    
    for group_id in group_ids {
        svg_out_str.push_str("\n");
        svg_out_str.push_str("  <symbol id=\"{}\">\n", group_id);
        svg_out_str.push_str(svg.show_group_widgets(group_id, "    "));
        svg_out_str.push_str("  </symbol>\n");
    }

    if svg.group_define_map.contains_key(DEFAULT_GROUP_ID) {
        let widget_list = &svg.group_define_map.get(DEFAULT_GROUP_ID).unwrap().widget_list;
        if widget_list.len() > 0 {
            svg.group_show_list.push(DEFAULT_GROUP_ID.to_string());
        }
    }

    svg_out_str.push_str(&svg.flush_data());

    svg_out_str.push_str("</svg>\n");

    svg_out_str
}
