use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn background_test() {
    let mut svg = Svg::new(100.0, 100.0);
    svg.set_background("#BBC42A".to_string());

    let svg_str = svg_out(svg);

    println!("{}", svg_str);

    let contents = include_str!("../showcase/basic/background.svg");

    assert_eq!(svg_str, contents);
}
