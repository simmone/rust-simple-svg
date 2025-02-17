use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn path_moveto_abs_test() {
    let mut svg = Svg::new(30.0, 70.0);
    
    let mut path = Path::new();
    
    path.moveto_abs((20.0, 60.0));

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
fn path_moveto_rel_test() {
    let mut svg = Svg::new(30.0, 70.0);
    
    let mut path = Path::new();
    
    path.moveto_rel((20.0, 60.0));

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


