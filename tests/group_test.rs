use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn group_basic_test() {
    let mut svg = Svg::new(220.0, 280.0);

    let line1_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (30.0, 30.0))));
    let line2_id = svg.add_shape(Shape::Line(Line::new((0.0, 15.0), (30.0, 15.0))));
    let line3_id = svg.add_shape(Shape::Line(Line::new((15.0, 0.0), (15.0, 30.0))));
    let line4_id = svg.add_shape(Shape::Line(Line::new((30.0, 0.0), (0.0, 30.0))));

    let mut line_sstyle = Sstyle::new();
    line_sstyle.stroke = Some("#765373".to_string());
    line_sstyle.stroke_width = Some(5.0);

    let mut group = Group::new();

    group.place_widget(Widget {
        shape_id: line1_id,
        style: Some(line_sstyle.clone()),
        at: Some((5.0, 5.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: line2_id,
        style: Some(line_sstyle.clone()),
        at: Some((5.0, 5.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: line3_id,
        style: Some(line_sstyle.clone()),
        at: Some((5.0, 5.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: line4_id,
        style: Some(line_sstyle.clone()),
        at: Some((5.0, 5.0)),
        ..Default::default()
    });

    let group_id = svg.add_group(group);

    let mut default_group = Group::new();

    default_group.place_widget(Widget {
        shape_id: group_id.clone(),
        at: Some((50.0, 50.0)),
        ..Default::default()
    });

    default_group.place_widget(Widget {
        shape_id: group_id.clone(),
        at: Some((100.0, 100.0)),
        ..Default::default()
    });

    default_group.place_widget(Widget {
        shape_id: group_id.clone(),
        at: Some((80.0, 200.0)),
        ..Default::default()
    });

    default_group.place_widget(Widget {
        shape_id: group_id.clone(),
        at: Some((150.0, 100.0)),
        ..Default::default()
    });

    svg.add_default_group(default_group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/group/group1.svg");

    assert_eq!(svg_str, contents);
}

#[test]
fn group_pattern_test() {
    let mut svg = Svg::new(100.0, 100.0);

    let rect_id = svg.add_shape(Shape::Rect(Rect::new(50.0, 50.0)));

    let line1_id = svg.add_shape(Shape::Line(Line::new((10.0, 0.0), (0.0, 50.0))));
    let line2_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (10.0, 50.0))));

    let mut line_sstyle = Sstyle::new();
    line_sstyle.stroke = Some("black".to_string());
    line_sstyle.stroke_width = Some(1.0);

    let mut cross_line_group = Group::new();

    cross_line_group.place_widget(Widget {
        shape_id: line1_id,
        style: Some(line_sstyle.clone()),
        ..Default::default()
    });

    cross_line_group.place_widget(Widget {
        shape_id: line2_id,
        style: Some(line_sstyle.clone()),
        ..Default::default()
    });

    let cross_line_group_id = svg.add_group(cross_line_group);

    let mut rect_sstyle = Sstyle::new();
    rect_sstyle.fill = Some("orange".to_string());
    rect_sstyle.stroke = Some("red".to_string());
    rect_sstyle.stroke_width = Some(2.0);

    let mut pattern_group = Group::new();

    pattern_group.place_widget(Widget {
        shape_id: rect_id,
        style: Some(rect_sstyle),
        ..Default::default()
    });

    pattern_group.place_widget(Widget {
        shape_id: cross_line_group_id.clone(),
        at: Some((0.0, 0.0)),
        ..Default::default()
    });

    pattern_group.place_widget(Widget {
        shape_id: cross_line_group_id.clone(),
        at: Some((10.0, 0.0)),
        ..Default::default()
    });

    pattern_group.place_widget(Widget {
        shape_id: cross_line_group_id.clone(),
        at: Some((20.0, 0.0)),
        ..Default::default()
    });

    pattern_group.place_widget(Widget {
        shape_id: cross_line_group_id.clone(),
        at: Some((30.0, 0.0)),
        ..Default::default()
    });

    pattern_group.place_widget(Widget {
        shape_id: cross_line_group_id.clone(),
        at: Some((40.0, 0.0)),
        ..Default::default()
    });

    let pattern_group_id = svg.add_group(pattern_group);

    let mut default_group = Group::new();

    default_group.place_widget(Widget {
        shape_id: pattern_group_id.clone(),
        at: Some((0.0, 0.0)),
        ..Default::default()
    });

    default_group.place_widget(Widget {
        shape_id: pattern_group_id.clone(),
        at: Some((50.0, 0.0)),
        ..Default::default()
    });

    default_group.place_widget(Widget {
        shape_id: pattern_group_id.clone(),
        at: Some((0.0, 50.0)),
        ..Default::default()
    });

    default_group.place_widget(Widget {
        shape_id: pattern_group_id.clone(),
        at: Some((50.0, 50.0)),
        ..Default::default()
    });

    svg.add_default_group(default_group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/group/group2.svg");

    assert_eq!(svg_str, contents);
}
