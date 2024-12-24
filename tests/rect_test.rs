use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn rect_svg_out_test() {
    let mut svg = Svg::new(100.0, 80.0);

    let rect_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));

    let mut rect_sstyle = Sstyle::new();
    rect_sstyle.fill = Some("#BBC42A".to_string());

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: rect_id,
        style: Some(&rect_sstyle),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/shapes/rect/rect.svg");

    assert_eq!(svg_str, contents);
}
