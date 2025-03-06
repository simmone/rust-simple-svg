use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn path_raw_test() {
    let mut svg = Svg::new(263.0, 188.0);

    let mut path = Path::new();

    path.raw(["             M248.761,92c0,9.801-7.93,17.731-17.71,17.731c-0.319,0-0.617,0-0.935-0.021\n",
              "             c-10.035,37.291-51.174,65.206-100.414,65.206 c-49.261,0-90.443-27.979-100.435-65.334\n",
              "             c-0.765,0.106-1.531,0.149-2.317,0.149c-9.78,0-17.71-7.93-17.71-17.731\n",
              "             c0-9.78,7.93-17.71,17.71-17.71c0.787,0,1.552,0.042,2.317,0.149\n",
              "             C39.238,37.084,80.419,9.083,129.702,9.083c49.24,0,90.379,27.937,100.414,65.228h0.021\n",
              "             c0.298-0.021,0.617-0.021,0.914-0.021C240.831,74.29,248.761,82.22,248.761,92z\n"].join("").to_string());

    let path_id = svg.add_shape(Shape::Path(path));

    let mut path_sstyle = Sstyle::new();
    path_sstyle.stroke = Some("#7AA20D".to_string());
    path_sstyle.fill = Some("#7AA20D".to_string());
    path_sstyle.stroke_width = Some(9.0);
    path_sstyle.stroke_linejoin = Some(LineJoin::Round);

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: path_id,
        style: Some(path_sstyle),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);
    
    print!("{}\n", svg_str);

    let contents = include_str!("../showcase/path/raw_path.svg");

    assert_eq!(svg_str, contents);
}
