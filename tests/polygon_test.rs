use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn polygon_test() {
    let mut svg = Svg::new(110.0, 110.0);

    let polygon_id = svg.add_shape(Shape::Polygon(Polygon::new(vec![
        (0.0, 25.0),
        (25.0, 0.0),
        (75.0, 0.0),
        (100.0, 25.0),
        (100.0, 75.0),
        (75.0, 100.0),
        (25.0, 100.0),
        (0.0, 75.0),
    ])));

    let mut polygon_sstyle = Sstyle::new();
    polygon_sstyle.fill = Some("#ED6E46".to_string());
    polygon_sstyle.stroke = Some("#765373".to_string());
    polygon_sstyle.stroke_width = Some(5.0);

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: polygon_id,
        style: Some(polygon_sstyle),
        at: Some((5.0, 5.0)),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/shapes/polygon/polygon.svg");

    assert_eq!(svg_str, contents);
}
