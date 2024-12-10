use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn empty_svg_out_test() {
    let svg = build_svg(30.0, 20.0);
    
    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/basic/empty.svg");

    assert_eq!(svg_str, contents);
}
