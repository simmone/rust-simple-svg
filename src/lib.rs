use crate::defines::svg::Svg;

pub mod defines;
pub mod lib_tests;

pub fn svg_out(svg: Svg) -> String {
    format!("<svg\n{}, {}", svg.width.to_string(), svg.height.to_string())
}


