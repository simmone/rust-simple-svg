use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn path_qcurve1_test() {
    let mut svg = Svg::new(220.0, 120.0);

    let mut path = Path::new();
    path.moveto_abs((10.0, 60.0));
    path.qcurve_abs((60.0, 10.0), (110.0, 60.0));
    path.qcurve_abs((160.0, 110.0), (210.0, 60.0));

    let qcurve_id = svg.add_shape(Shape::Path(path));

    let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

    let mut group = Group::new();

    let mut qcurve_sstyle = Sstyle::new();
    qcurve_sstyle.stroke_width = Some(3.0);
    qcurve_sstyle.stroke = Some("#333333".to_string());
    group.place_widget(Widget {
        shape_id: qcurve_id,
        style: Some(qcurve_sstyle),
        ..Default::default()
    });

    let mut circle_sstyle = Sstyle::new();
    circle_sstyle.fill = Some("red".to_string());

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((10.0, 60.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((60.0, 10.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((110.0, 60.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((160.0, 110.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((210.0, 60.0)),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/path/qcurve1.svg");

    assert_eq!(svg_str, contents);
}

#[test]
fn path_qcurve2_test() {
    let mut svg = Svg::new(220.0, 120.0);

    let mut path = Path::new();
    path.moveto_abs((10.0, 60.0));
    path.qcurve_rel((50.0, -50.0), (100.0, 0.0));
    path.qcurve_rel((50.0, 50.0), (100.0, 0.0));

    let qcurve_id = svg.add_shape(Shape::Path(path));

    let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

    let mut group = Group::new();

    let mut qcurve_sstyle = Sstyle::new();
    qcurve_sstyle.stroke_width = Some(3.0);
    qcurve_sstyle.stroke = Some("#333333".to_string());
    group.place_widget(Widget {
        shape_id: qcurve_id,
        style: Some(qcurve_sstyle),
        ..Default::default()
    });

    let mut circle_sstyle = Sstyle::new();
    circle_sstyle.fill = Some("red".to_string());

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((10.0, 60.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((60.0, 10.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((110.0, 60.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((160.0, 110.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((210.0, 60.0)),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/path/qcurve2.svg");

    assert_eq!(svg_str, contents);
}
