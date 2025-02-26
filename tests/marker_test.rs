use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn marker_normal_test() {
    let mut svg = Svg::new(200.0, 120.0);

    let marker_id = svg.add_shape(Shape::Marker(Marker::new(MarkerType::Triangle1)));

    let line_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (100.0, 0.0))));

    let mut line_sstyle = Sstyle::new();
    line_sstyle.stroke = Some("#000000".to_string());
    line_sstyle.stroke_width = Some(2.0);

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: line_id,
        style: Some(line_sstyle),
        at: Some((50.0, 50.0)),
        marker_end_id: Some(marker_id),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/marker/marker1.svg");

    assert_eq!(svg_str, contents);
}

#[test]
fn marker_all_direction_test() {
    let mut svg = Svg::new(300.0, 300.0);

    let marker_id = svg.add_shape(Shape::Marker(Marker::new(MarkerType::Triangle2)));

    let line1_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (100.0, 0.0))));
    let line2_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (70.0, 70.0))));
    let line3_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (0.0, 100.0))));
    let line4_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (-70.0, 70.0))));
    let line5_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (-100.0, 0.0))));
    let line6_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (-70.0, -70.0))));
    let line7_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (0.0, -100.0))));
    let line8_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (70.0, -70.0))));

    let mut line_sstyle = Sstyle::new();
    line_sstyle.stroke = Some("#000000".to_string());
    line_sstyle.stroke_width = Some(2.0);

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: line1_id,
        style: Some(line_sstyle.clone()),
        at: Some((180.0, 150.0)),
        marker_start_id: Some(marker_id.clone()),
        marker_end_id: Some(marker_id.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: line2_id,
        style: Some(line_sstyle.clone()),
        at: Some((180.0, 180.0)),
        marker_start_id: Some(marker_id.clone()),
        marker_end_id: Some(marker_id.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: line3_id,
        style: Some(line_sstyle.clone()),
        at: Some((150.0, 180.0)),
        marker_start_id: Some(marker_id.clone()),
        marker_end_id: Some(marker_id.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: line4_id,
        style: Some(line_sstyle.clone()),
        at: Some((120.0, 180.0)),
        marker_start_id: Some(marker_id.clone()),
        marker_end_id: Some(marker_id.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: line5_id,
        style: Some(line_sstyle.clone()),
        at: Some((120.0, 150.0)),
        marker_start_id: Some(marker_id.clone()),
        marker_end_id: Some(marker_id.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: line6_id,
        style: Some(line_sstyle.clone()),
        at: Some((120.0, 120.0)),
        marker_start_id: Some(marker_id.clone()),
        marker_end_id: Some(marker_id.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: line7_id,
        style: Some(line_sstyle.clone()),
        at: Some((150.0, 120.0)),
        marker_start_id: Some(marker_id.clone()),
        marker_end_id: Some(marker_id.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: line8_id,
        style: Some(line_sstyle.clone()),
        at: Some((180.0, 120.0)),
        marker_start_id: Some(marker_id.clone()),
        marker_end_id: Some(marker_id.clone()),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/marker/marker2.svg");

    assert_eq!(svg_str, contents);
}

#[test]
fn marker_all_test() {
    let mut svg = Svg::new(200.0, 200.0);

    let arrow_marker_id = svg.add_shape(Shape::Marker(Marker::new(MarkerType::Triangle1)));

    let circle_marker_id = svg.add_shape(Shape::Marker(Marker::new(MarkerType::Circle)));

    let polyline_id = svg.add_shape(Shape::Polyline(Polyline::new(vec![
        (0.0, 0.0),
        (0.0, 100.0),
        (100.0, 100.0),
    ])));

    let mut polyline_sstyle = Sstyle::new();
    polyline_sstyle.stroke = Some("#000000".to_string());
    polyline_sstyle.stroke_width = Some(2.0);

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: polyline_id,
        style: Some(polyline_sstyle),
        at: Some((50.0, 50.0)),
        marker_start_id: Some(arrow_marker_id.clone()),
        marker_mid_id: Some(circle_marker_id),
        marker_end_id: Some(arrow_marker_id.clone()),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/marker/marker3.svg");

    assert_eq!(svg_str, contents);
}

#[test]
fn marker_curve_test() {
    let mut svg = Svg::new(300.0, 300.0);

    let mut ccurve1_path = Path::new();
    ccurve1_path.moveto_abs((100.0, 75.0));
    ccurve1_path.ccurve_abs((125.0, 50.0), (150.0, 50.0), (175.0, 75.0));
    let ccurve1_id = svg.add_shape(Shape::Path(ccurve1_path));

    let mut ccurve2_path = Path::new();
    ccurve2_path.moveto_abs((175.0, 125.0));
    ccurve2_path.ccurve_abs((150.0, 150.0), (125.0, 150.0), (100.0, 125.0));
    let ccurve2_id = svg.add_shape(Shape::Path(ccurve2_path));

    let arrow_marker_id = svg.add_shape(Shape::Marker(Marker::new(MarkerType::Triangle1)));

    let mut ccurve1_sstyle = Sstyle::new();
    ccurve1_sstyle.stroke = Some("crimson".to_string());
    ccurve1_sstyle.stroke_width = Some(10.0);

    let mut ccurve2_sstyle = ccurve1_sstyle.clone();
    ccurve2_sstyle.stroke = Some("olivedrab".to_string());

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: ccurve1_id,
        style: Some(ccurve1_sstyle),
        marker_end_id: Some(arrow_marker_id.clone()),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: ccurve2_id,
        style: Some(ccurve2_sstyle),
        marker_end_id: Some(arrow_marker_id.clone()),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/marker/marker4.svg");

    assert_eq!(svg_str, contents);
}
