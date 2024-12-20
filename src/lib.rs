pub use crate::defines::group::Group;
pub use crate::defines::rect::Rect;
pub use crate::defines::shape::Shape;
pub use crate::defines::sstyle::Sstyle;
pub use crate::defines::svg::Svg;
pub use crate::defines::widget::Widget;

pub mod defines;

pub fn svg_out(svg: Svg) -> String {
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
    svg_out_str.push_str("</svg>\n");

    svg_out_str
}
