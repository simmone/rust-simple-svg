use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn filter_test() {
    let mut svg = Svg::new(140.0, 140.0);

    let circle_id = svg.add_shape(Shape::Circle(Circle::new(50.0)));

    let mut circle_sstyle = Sstyle::new();
    circle_sstyle.stroke = Some("red".to_string());
    circle_sstyle.stroke_width = Some(12.0);

    let filter_id = svg.add_shape(Shape::Filter(Filter::new()));

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: circle_id,
        style: Some(circle_sstyle),
        filter_id: Some(filter_id),
        at: Some((70.0, 70.0)),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/filter/filter_blur_dropdown.svg");

    assert_eq!(svg_str, contents);
}
