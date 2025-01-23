use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn circle_test() {
    let mut svg = Svg::new(100.0, 100.0);

    let circle_id = svg.add_shape(Shape::Circle(Circle::new(50.0)));

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

#[test]
fn circle3_test() {
    let mut svg = Svg::new(200.0, 200.0);

    let circle_id = svg.add_shape(Shape::Circle(Circle::new(50.0)));

    let mut red_circle_sstyle = Sstyle::new();
    red_circle_sstyle.fill = Some("red".to_string());

    let mut yellow_circle_sstyle = Sstyle::new();
    yellow_circle_sstyle.fill = Some("yellow".to_string());

    let mut blue_circle_sstyle = Sstyle::new();
    blue_circle_sstyle.fill = Some("blue".to_string());

    let mut green_circle_sstyle = Sstyle::new();
    green_circle_sstyle.fill = Some("green".to_string());

    let mut group = Group::new();

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(red_circle_sstyle),
        at: Some((50.0, 50.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(yellow_circle_sstyle),
        at: Some((150.0, 50.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(blue_circle_sstyle),
        at: Some((50.0, 150.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(green_circle_sstyle),
        at: Some((150.0, 150.0)),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/shapes/circle/circle3.svg");

    assert_eq!(svg_str, contents);
}
