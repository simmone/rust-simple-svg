use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn path_lineto_test() {
    let mut svg = Svg::new(110.0, 160.0);

    let mut path = Path::new();

    path.moveto_abs((5.0, 5.0));
    path.lineto_hor(100.0);
    path.lineto_ver(100.0);
    path.lineto_rel((-50.0, 50.0));
    path.lineto_rel((-50.0, -50.0));
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
