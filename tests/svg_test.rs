use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn check_add_shape() {
    let mut svg: Svg = Svg::new(640.0, 480.0);

    let rect1 = Rect::new(30.0, 20.0);
    let shape1 = Shape::Rect(rect1);
    let _rect_id = svg.add_shape(shape1);
    assert_eq!(svg.shape_id_count, 1);
    match svg.shape_define_map.get("s1").unwrap() {
        Shape::Rect(s1) => {
            assert_eq!(s1.width, 30.0);
        }
        Shape::Circle(_) => {}
        Shape::Ellipse(_) => {}
        Shape::Line(_) => {}
        Shape::Polygon(_) => {}
        Shape::Polyline(_) => {}
        Shape::Filter(_) => {}
        Shape::LinearGradient(_) => {}
        Shape::RadialGradient(_) => {}
    }

    let rect2 = Rect::new(10.0, 5.0);
    let shape2 = Shape::Rect(rect2);
    let _rect_id = svg.add_shape(shape2);
    assert_eq!(svg.shape_id_count, 2);
    match svg.shape_define_map.get("s2").unwrap() {
        Shape::Rect(s2) => {
            assert_eq!(s2.width, 10.0);
        }
        Shape::Circle(_) => {}
        Shape::Ellipse(_) => {}
        Shape::Line(_) => {}
        Shape::Polygon(_) => {}
        Shape::Polyline(_) => {}
        Shape::Filter(_) => {}
        Shape::LinearGradient(_) => {}
        Shape::RadialGradient(_) => {}
    }
}

#[test]
fn check_add_group() {
    let mut svg: Svg = Svg::new(640.0, 480.0);
    let shape_id = svg.add_shape(Shape::Rect(Rect::new(30.0, 20.0)));

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: shape_id,
        ..Default::default()
    });

    svg.add_group(group);

    assert_eq!(svg.group_define_map.len(), 1);
    assert_eq!(
        svg.group_define_map.get("g1").unwrap().widget_list[0].shape_id,
        "s1".to_string()
    );
}

#[test]
fn check_flush_data() {
    let mut svg: Svg = Svg::new(100.0, 100.0);
    let shape1_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));

    let mut rect_sstyle = Sstyle::new();
    rect_sstyle.fill = Some("#BBC42A".to_string());

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: shape1_id,
        style: Some(rect_sstyle.clone()),
        ..Default::default()
    });

    let shape2_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));
    group.place_widget(Widget {
        shape_id: shape2_id,
        style: Some(rect_sstyle.clone()),
        ..Default::default()
    });

    let shape3_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));
    group.place_widget(Widget {
        shape_id: shape3_id,
        style: Some(rect_sstyle.clone()),
        ..Default::default()
    });

    let shape4_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));
    group.place_widget(Widget {
        shape_id: shape4_id,
        style: Some(rect_sstyle.clone()),
        ..Default::default()
    });

    let shape5_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));
    group.place_widget(Widget {
        shape_id: shape5_id,
        style: Some(rect_sstyle.clone()),
        ..Default::default()
    });

    svg.add_default_group(group);

    let mut expected_str = String::new();
    expected_str.push_str("  <defs>\n");
    expected_str.push_str("    <rect id=\"s1\" width=\"100\" height=\"100\" />\n");
    expected_str.push_str("    <rect id=\"s2\" width=\"100\" height=\"100\" />\n");
    expected_str.push_str("    <rect id=\"s3\" width=\"100\" height=\"100\" />\n");
    expected_str.push_str("    <rect id=\"s4\" width=\"100\" height=\"100\" />\n");
    expected_str.push_str("    <rect id=\"s5\" width=\"100\" height=\"100\" />\n");
    expected_str.push_str("  </defs>\n\n");
    expected_str.push_str("  <use xlink:href=\"#s1\" fill=\"#BBC42A\" />\n");
    expected_str.push_str("  <use xlink:href=\"#s2\" fill=\"#BBC42A\" />\n");
    expected_str.push_str("  <use xlink:href=\"#s3\" fill=\"#BBC42A\" />\n");
    expected_str.push_str("  <use xlink:href=\"#s4\" fill=\"#BBC42A\" />\n");
    expected_str.push_str("  <use xlink:href=\"#s5\" fill=\"#BBC42A\" />\n");

    assert_eq!(svg.flush_data(), expected_str);
}
