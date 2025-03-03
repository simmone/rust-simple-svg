use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn arrow_normal_test() {
    let mut svg = Svg::new(300.0, 300.0);

    let arrow_id = svg.add_shape(Shape::Arrow(Arrow::new(
        (50.0, 50.0),
        (280.0, 280.0),
        40.0,
        40.0,
        80.0,
    )));

    let mut arrow_sstyle = Sstyle::new();
    arrow_sstyle.stroke = Some("teal".to_string());
    arrow_sstyle.stroke_width = Some(5.0);
    arrow_sstyle.fill = Some("lavender".to_string());

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: arrow_id,
        style: Some(arrow_sstyle),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/arrow/arrow1.svg");

    assert_eq!(svg_str, contents);
}
