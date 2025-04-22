use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn five_star_test() {
    let mut svg = Svg::new(500.0, 300.0);

    let circle_id = svg.add_shape(Shape::Circle(Circle::new(60.0)));

    let filter_id = svg.add_shape(Shape::Filter(Filter::new()));

    let mut circle_sstyle = Sstyle::new();
    circle_sstyle.stroke_width = Some(12.0);

    let mut circle1_sstyle = circle_sstyle.clone();
    circle1_sstyle.stroke = Some("rgb(11, 112, 191)".to_string());

    let mut circle2_sstyle = circle_sstyle.clone();
    circle2_sstyle.stroke = Some("rgb(240, 183, 0)".to_string());

    let mut circle3_sstyle = circle_sstyle.clone();
    circle3_sstyle.stroke = Some("rgb(0, 0, 0)".to_string());

    let mut circle4_sstyle = circle_sstyle.clone();
    circle4_sstyle.stroke = Some("rgb(13, 146, 38)".to_string());

    let mut circle5_sstyle = circle_sstyle.clone();
    circle5_sstyle.stroke = Some("rgb(214, 0, 23)".to_string());

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle1_sstyle),
        filter_id: Some(filter_id.clone()),
        at: Some((120.0, 120.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle2_sstyle),
        filter_id: Some(filter_id.clone()),
        at: Some((180.0, 180.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle3_sstyle),
        filter_id: Some(filter_id.clone()),
        at: Some((260.0, 120.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle4_sstyle),
        filter_id: Some(filter_id.clone()),
        at: Some((320.0, 180.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle5_sstyle),
        filter_id: Some(filter_id.clone()),
        at: Some((400.0, 120.0)),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    println!("{}", svg_str);

    let contents = include_str!("../showcase/example/five_circles.svg");

    assert_eq!(svg_str, contents);
}
