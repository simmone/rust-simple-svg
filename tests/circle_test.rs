use simple_svg::*;

#[test]
fn circle_format_test() {
    let mut svg = Svg::new(100.0, 100.0);

    let circle_id = svg.add_shape(Shape::Circle(Circle::new(50.00001)));

    let mut circle_sstyle = Sstyle::new();
    circle_sstyle.fill = Some("#BBC42A".to_string());

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: circle_id,
        style: Some(circle_sstyle),
        at: Some((50.0, 50.0)),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/shapes/circle/circle.svg");

    assert_eq!(svg_str, contents);
}
