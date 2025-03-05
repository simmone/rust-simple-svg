use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn line_test() {
    let mut svg = Svg::new(110.0, 110.0);

    let line_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (100.0, 100.0))));

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

#[test]
fn dual_line_test() {
    let mut svg = Svg::new(800.0, 600.0);
    svg.set_background("#eeeeee".to_string());

    let line_h_id = svg.add_shape(Shape::Line(Line::new((100.0, 300.0), (700.0, 300.0))));
    let line_v_id = svg.add_shape(Shape::Line(Line::new((400.0, 100.0), (400.0, 500.0))));

    let mut line_h_sstyle = Sstyle::new();
    line_h_sstyle.stroke = Some("#5c3d19".to_string());
    line_h_sstyle.stroke_width = Some(2.0);

    let mut line_v_sstyle = Sstyle::new();
    line_v_sstyle.stroke = Some("#5c3d19".to_string());
    line_v_sstyle.stroke_width = Some(2.0);

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: line_h_id,
        style: Some(line_h_sstyle),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: line_v_id,
        style: Some(line_v_sstyle),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/shapes/line/dual.svg");

    assert_eq!(svg_str, contents);
}

