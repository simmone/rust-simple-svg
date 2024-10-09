use crate::defines::svg::Svg;

pub mod defines;

pub fn svg_out(svg: Svg) -> String {
    format!("<svg\n{}, {}", svg.width.to_string(), svg.height.to_string())
}

#[cfg(test)]
mod lib_tests;
