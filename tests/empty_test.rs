use pretty_assertions::assert_eq;
use simple_svg::defines::svg::build_svg;
use simple_svg::svg_out;

#[test]
fn void_svg_out_test() {
    let svg = svg_out(build_svg(20, 20));

    let contents = include_str!("../showcase/basic/empty.svg");

    assert_eq!(svg, contents);
}
