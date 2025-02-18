use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn path_arc_test() {
    let mut svg = Svg::new(300.0, 130.0);

    let mut arc1 = Path::new();
    arc1.moveto_abs((130.0, 45.0));
    arc1.arc_abs((170.0, 85.0), (80.0, 40.0), ArcDirection::LeftBig);
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
