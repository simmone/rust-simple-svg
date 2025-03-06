use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn logo_test() {
    let mut svg = Svg::new(519.875, 519.824);

    let background_circle_id = svg.add_shape(Shape::Circle(Circle::new(253.093)));

    let mut blue_piece_path = Path::new();
    blue_piece_path.raw(["M455.398,412.197c33.792-43.021,53.946-97.262,53.946-156.211",
                   	     "c0-139.779-113.313-253.093-253.093-253.093c-30.406,0-59.558,5.367-86.566,15.197",
                         "C272.435,71.989,408.349,247.839,455.398,412.197z"].join("").to_string());

    let mut left_red_piece_path = Path::new();
    left_red_piece_path.raw(["M220.003,164.337c-39.481-42.533-83.695-76.312-130.523-98.715",
	                           "C36.573,112.011,3.159,180.092,3.159,255.986c0,63.814,23.626,122.104,62.597,166.623",
	                           "C100.111,319.392,164.697,219.907,220.003,164.337z"].join("").to_string());

    let mut bottom_red_piece_path = Path::new();
    bottom_red_piece_path.raw(["M266.638,221.727c-54.792,59.051-109.392,162.422-129.152,257.794"
	                             "c35.419,18.857,75.84,29.559,118.766,29.559c44.132,0,85.618-11.306,121.74-31.163"
                               "C357.171,381.712,317.868,293.604,266.638,221.727z"].join("").to_string());

    let path_id = svg.add_shape(Shape::Path(path));

    let mut path_sstyle = Sstyle::new();
    path_sstyle.stroke = Some("#7AA20D".to_string());
    path_sstyle.fill = Some("#7AA20D".to_string());
    path_sstyle.stroke_width = Some(9.0);
    path_sstyle.stroke_linejoin = Some(LineJoin::Round);

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
    
    print!("{}\n", svg_str);

    let contents = include_str!("../showcase/example/five_circles.svg");

    assert_eq!(svg_str, contents);
}
