use simple_svg::*;

#[test]
fn line_format_test() {
    let mut svg = Svg::new(110.0, 110.0);

    let line_id = svg.add_shape(Shape::Line(Line::new((0.00001, 0.00001), (100.00001, 100.00001))));

    let mut line_sstyle = Sstyle::new();
    line_sstyle.stroke = Some("#765373".to_string());
    line_sstyle.stroke_width = Some(10.0);

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: line_id,
        style: Some(line_sstyle),
        at: Some((5.0, 5.0)),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/shapes/line/line.svg");

    assert_eq!(svg_str, contents);
}
