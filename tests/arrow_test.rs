use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn arrow1_test() {
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

#[test]
fn arrow2_test() {
    let mut svg = Svg::new(400.0, 400.0);

    let arrow10_id = svg.add_shape(Shape::Arrow(Arrow::new(
        (200.0, 200.0),
        (200.0, 350.0),
        10.0,
        10.0,
        20.0,
    )));

    let arrow11_id = svg.add_shape(Shape::Arrow(Arrow::new(
        (200.0, 200.0),
        (150.0, 300.0),
        10.0,
        10.0,
        20.0,
    )));

    let arrow12_id = svg.add_shape(Shape::Arrow(Arrow::new(
        (200.0, 200.0),
        (100.0, 250.0),
        10.0,
        10.0,
        20.0,
    )));

    let arrow20_id = svg.add_shape(Shape::Arrow(Arrow::new(
        (200.0, 200.0),
        (50.0, 200.0),
        10.0,
        10.0,
        20.0,
    )));

    let arrow21_id = svg.add_shape(Shape::Arrow(Arrow::new(
        (200.0, 200.0),
        (100.0, 150.0),
        10.0,
        10.0,
        20.0,
    )));

    let arrow22_id = svg.add_shape(Shape::Arrow(Arrow::new(
        (200.0, 200.0),
        (150.0, 100.0),
        10.0,
        10.0,
        20.0,
    )));

    let arrow30_id = svg.add_shape(Shape::Arrow(Arrow::new(
        (200.0, 200.0),
        (200.0, 50.0),
        10.0,
        10.0,
        20.0,
    )));

    let arrow31_id = svg.add_shape(Shape::Arrow(Arrow::new(
        (200.0, 200.0),
        (250.0, 100.0),
        10.0,
        10.0,
        20.0,
    )));

    let arrow32_id = svg.add_shape(Shape::Arrow(Arrow::new(
        (200.0, 200.0),
        (300.0, 150.0),
        10.0,
        10.0,
        20.0,
    )));

    let arrow40_id = svg.add_shape(Shape::Arrow(Arrow::new(
        (200.0, 200.0),
        (350.0, 200.0),
        10.0,
        10.0,
        20.0,
    )));

    let arrow41_id = svg.add_shape(Shape::Arrow(Arrow::new(
        (200.0, 200.0),
        (300.0, 250.0),
        10.0,
        10.0,
        20.0,
    )));

    let arrow42_id = svg.add_shape(Shape::Arrow(Arrow::new(
        (200.0, 200.0),
        (250.0, 300.0),
        10.0,
        10.0,
        20.0,
    )));

    let mut arrow_sstyle = Sstyle::new();
    arrow_sstyle.stroke = Some("green".to_string());
    arrow_sstyle.stroke_width = Some(2.0);
    arrow_sstyle.fill = Some("#ED6E46".to_string());

    let mut group = Group::new();

    group.place_widget(Widget {
        shape_id: arrow10_id,
        style: Some(arrow_sstyle.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: arrow11_id,
        style: Some(arrow_sstyle.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: arrow12_id,
        style: Some(arrow_sstyle.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: arrow20_id,
        style: Some(arrow_sstyle.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: arrow21_id,
        style: Some(arrow_sstyle.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: arrow22_id,
        style: Some(arrow_sstyle.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: arrow30_id,
        style: Some(arrow_sstyle.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: arrow31_id,
        style: Some(arrow_sstyle.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: arrow32_id,
        style: Some(arrow_sstyle.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: arrow40_id,
        style: Some(arrow_sstyle.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: arrow41_id,
        style: Some(arrow_sstyle.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: arrow42_id,
        style: Some(arrow_sstyle.clone()),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/arrow/arrow2.svg");

    assert_eq!(svg_str, contents);
}

