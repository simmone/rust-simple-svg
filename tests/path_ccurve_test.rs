use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn path_ccurve1_test() {
    let mut svg = Svg::new(200.0, 120.0);

    let mut path = Path::new();
    path.moveto_abs((10.0, 60.0));
    path.ccurve_abs((30.0, 15.0), (80.0, 15.0), (100.0, 60.0));
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
fn path_ccurve2_test() {
    let mut svg = Svg::new(200.0, 120.0);

    let mut path = Path::new();
    path.moveto_abs((10.0, 60.0));
    path.ccurve_rel((20.0, -45.0), (70.0, -45.0), (90.0, 0.0));
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
