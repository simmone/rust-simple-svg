use simple_svg::*;

#[test]
fn path_absolute_moveto_format_test() {
    let mut svg = Svg::new(30.0, 70.0);

    let mut path = Path::new();

    path.moveto_abs((20.00001, 60.00001));

    let path_id = svg.add_shape(Shape::Path(path));

    let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

    let mut path_sstyle = Sstyle::new();
    path_sstyle.stroke = Some("#7AA20D".to_string());
    path_sstyle.stroke_width = Some(1.0);

    let mut circle_sstyle = Sstyle::new();
    circle_sstyle.fill = Some("red".to_string());

    let mut group = Group::new();

    group.place_widget(Widget {
        shape_id: path_id,
        style: Some(path_sstyle),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id,
        style: Some(circle_sstyle),
        at: Some((20.0, 60.0)),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/path/moveto1.svg");

    assert_eq!(svg_str, contents);
}

#[test]
fn path_relative_moveto_format_test() {
    let mut svg = Svg::new(30.0, 70.0);

    let mut path = Path::new();

    path.moveto_rel((20.00001, 60.00001));

    let path_id = svg.add_shape(Shape::Path(path));

    let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

    let mut path_sstyle = Sstyle::new();
    path_sstyle.stroke = Some("#7AA20D".to_string());
    path_sstyle.stroke_width = Some(1.0);

    let mut circle_sstyle = Sstyle::new();
    circle_sstyle.fill = Some("red".to_string());

    let mut group = Group::new();

    group.place_widget(Widget {
        shape_id: path_id,
        style: Some(path_sstyle),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id,
        style: Some(circle_sstyle),
        at: Some((20.0, 60.0)),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/path/moveto2.svg");

    assert_eq!(svg_str, contents);
}

#[test]
fn path_lineto_format_test() {
    let mut svg = Svg::new(110.0, 160.0);

    let mut path = Path::new();

    path.moveto_abs((5.0, 5.0));
    path.lineto_hor(100.00001);
    path.lineto_ver(100.00001);
    path.lineto_rel((-50.00001, 50.00001));
    path.lineto_rel((-50.00001, -50.00001));
    path.close();

    let path_id = svg.add_shape(Shape::Path(path));

    let mut path_sstyle = Sstyle::new();
    path_sstyle.stroke = Some("#7AA20D".to_string());
    path_sstyle.stroke_width = Some(5.0);
    path_sstyle.stroke_linejoin = Some(LineJoin::Round);

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: path_id,
        style: Some(path_sstyle),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/path/lineto.svg");

    assert_eq!(svg_str, contents);
}

#[test]
fn path_arc_format_test() {
    let mut svg = Svg::new(300.0, 130.0);

    let mut arc1 = Path::new();
    arc1.moveto_abs((130.0, 45.0));
    arc1.arc_abs(
        (170.00001, 85.00001),
        (80.00001, 40.00001),
        ArcDirection::LeftBig,
    );
    let arc1_id = svg.add_shape(Shape::Path(arc1));

    let mut arc2 = Path::new();
    arc2.moveto_abs((130.0, 45.0));
    arc2.arc_abs((170.0, 85.0), (80.0, 40.0), ArcDirection::LeftSmall);
    let arc2_id = svg.add_shape(Shape::Path(arc2));

    let mut arc3 = Path::new();
    arc3.moveto_abs((130.0, 45.0));
    arc3.arc_abs((170.0, 85.0), (80.0, 40.0), ArcDirection::RightBig);
    let arc3_id = svg.add_shape(Shape::Path(arc3));

    let mut arc4 = Path::new();
    arc4.moveto_abs((130.0, 45.0));
    arc4.arc_abs((170.0, 85.0), (80.0, 40.0), ArcDirection::RightSmall);
    let arc4_id = svg.add_shape(Shape::Path(arc4));

    let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

    let mut group = Group::new();

    let mut arc_sstyle = Sstyle::new();
    arc_sstyle.stroke_width = Some(3.0);

    let mut arc1_sstyle = arc_sstyle.clone();
    arc1_sstyle.stroke = Some("#ccccff".to_string());
    group.place_widget(Widget {
        shape_id: arc1_id,
        style: Some(arc1_sstyle),
        ..Default::default()
    });

    let mut arc2_sstyle = arc_sstyle.clone();
    arc2_sstyle.stroke = Some("green".to_string());
    group.place_widget(Widget {
        shape_id: arc2_id,
        style: Some(arc2_sstyle),
        ..Default::default()
    });

    let mut arc3_sstyle = arc_sstyle.clone();
    arc3_sstyle.stroke = Some("blue".to_string());
    group.place_widget(Widget {
        shape_id: arc3_id,
        style: Some(arc3_sstyle),
        ..Default::default()
    });

    let mut arc4_sstyle = arc_sstyle.clone();
    arc4_sstyle.stroke = Some("yellow".to_string());
    group.place_widget(Widget {
        shape_id: arc4_id,
        style: Some(arc4_sstyle),
        ..Default::default()
    });

    let mut circle_sstyle = Sstyle::new();
    circle_sstyle.fill = Some("red".to_string());

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((130.0, 45.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((170.0, 85.0)),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/path/arc.svg");

    assert_eq!(svg_str, contents);
}

#[test]
fn path_qcurve_absolute_format_test() {
    let mut svg = Svg::new(220.0, 120.0);

    let mut path = Path::new();
    path.moveto_abs((10.0, 60.0));
    path.qcurve_abs((60.00001, 10.00001), (110.00001, 60.00001));
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
fn path_qcurve_relative_format_test() {
    let mut svg = Svg::new(220.0, 120.0);

    let mut path = Path::new();
    path.moveto_abs((10.0, 60.0));
    path.qcurve_rel((50.00001, -50.00001), (100.00001, 0.00001));
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

#[test]
fn path_ccurve_absolute_format_test() {
    let mut svg = Svg::new(200.0, 120.0);

    let mut path = Path::new();
    path.moveto_abs((10.0, 60.0));
    path.ccurve_abs(
        (30.00001, 15.00001),
        (80.00001, 15.00001),
        (100.00001, 60.00001),
    );
    path.ccurve_abs((120.0, 105.0), (170.0, 105.0), (190.0, 60.0));
    let ccurve_id = svg.add_shape(Shape::Path(path));

    let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

    let mut group = Group::new();

    let mut ccurve_sstyle = Sstyle::new();
    ccurve_sstyle.stroke_width = Some(3.0);
    ccurve_sstyle.stroke = Some("#333333".to_string());
    group.place_widget(Widget {
        shape_id: ccurve_id,
        style: Some(ccurve_sstyle),
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
        at: Some((30.0, 15.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((80.0, 15.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((100.0, 60.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((120.0, 105.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((170.0, 105.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((190.0, 60.0)),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/path/ccurve1.svg");

    assert_eq!(svg_str, contents);
}

#[test]
fn path_ccurve_relative_format_test() {
    let mut svg = Svg::new(200.0, 120.0);

    let mut path = Path::new();
    path.moveto_abs((10.0, 60.0));
    path.ccurve_rel(
        (20.00001, -45.00001),
        (70.00001, -45.00001),
        (90.00001, 0.00001),
    );
    path.ccurve_rel((20.0, 45.0), (70.0, 45.0), (90.0, 0.0));

    let ccurve_id = svg.add_shape(Shape::Path(path));

    let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

    let mut group = Group::new();

    let mut ccurve_sstyle = Sstyle::new();
    ccurve_sstyle.stroke_width = Some(3.0);
    ccurve_sstyle.stroke = Some("#333333".to_string());
    group.place_widget(Widget {
        shape_id: ccurve_id,
        style: Some(ccurve_sstyle),
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
        at: Some((30.0, 15.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((80.0, 15.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((100.0, 60.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((120.0, 105.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((170.0, 105.0)),
        ..Default::default()
    });

    group.place_widget(Widget {
        shape_id: circle_id.clone(),
        style: Some(circle_sstyle.clone()),
        at: Some((190.0, 60.0)),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/path/ccurve2.svg");

    assert_eq!(svg_str, contents);
}
