use pretty_assertions::assert_eq;
use simple_svg::defines::svg::new_svg;
use simple_svg::*;

#[test]
fn empty_svg_out_test() {
    let svg = new_svg(30, 20);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/basic/empty.svg");

    assert_eq!(svg_str, contents);
}
