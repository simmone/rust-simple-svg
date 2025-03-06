use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn logo_test() {
    let mut svg = Svg::new(519.875, 519.824);

    let mut group = Group::new();

    let background_circle_id = svg.add_shape(Shape::Circle(Circle::new(253.093)));

    let mut background_circle_sstyle = Sstyle::new();
    background_circle_sstyle.fill = Some("white".to_string());

    group.place_widget(Widget {
        shape_id: background_circle_id.clone(),
        style: Some(background_circle_sstyle),
        at: Some((256.252, 255.986)),
        ..Default::default()
    });

    let mut blue_piece_path = Path::new();
    blue_piece_path.raw(["             M455.398,412.197c33.792-43.021,53.946-97.262,53.946-156.211\n",
                   	     "             c0-139.779-113.313-253.093-253.093-253.093c-30.406,0-59.558,5.367-86.566,15.197\n",
                         "             C272.435,71.989,408.349,247.839,455.398,412.197z"].join("").to_string());
    let blue_piece_path_id = svg.add_shape(Shape::Path(blue_piece_path));

    let mut blue_piece_path_sstyle = Sstyle::new();
    blue_piece_path_sstyle.fill = Some("#3E5BA9".to_string());

    group.place_widget(Widget {
        shape_id: blue_piece_path_id,
        style: Some(blue_piece_path_sstyle),
        ..Default::default()
    });

    let mut left_red_piece_path = Path::new();
    left_red_piece_path.raw(["             M220.003,164.337c-39.481-42.533-83.695-76.312-130.523-98.715\n",
	                           "             C36.573,112.011,3.159,180.092,3.159,255.986c0,63.814,23.626,122.104,62.597,166.623\n",
	                           "             C100.111,319.392,164.697,219.907,220.003,164.337z"].join("").to_string());
    let left_red_piece_path_id = svg.add_shape(Shape::Path(left_red_piece_path));

    let mut left_red_piece_path_sstyle = Sstyle::new();
    left_red_piece_path_sstyle.fill = Some("#9F1D20".to_string());

    group.place_widget(Widget {
        shape_id: left_red_piece_path_id,
        style: Some(left_red_piece_path_sstyle),
        ..Default::default()
    });

    let mut bottom_red_piece_path = Path::new();
    bottom_red_piece_path.raw(["             M266.638,221.727c-54.792,59.051-109.392,162.422-129.152,257.794\n",
	                             "             c35.419,18.857,75.84,29.559,118.766,29.559c44.132,0,85.618-11.306,121.74-31.163\n",
                               "             C357.171,381.712,317.868,293.604,266.638,221.727z"].join("").to_string());
    let bottom_red_piece_path_id = svg.add_shape(Shape::Path(bottom_red_piece_path));

    let mut bottom_red_piece_path_sstyle = Sstyle::new();
    bottom_red_piece_path_sstyle.fill = Some("#9F1D20".to_string());

    group.place_widget(Widget {
        shape_id: bottom_red_piece_path_id,
        style: Some(bottom_red_piece_path_sstyle),
        ..Default::default()
    });

    svg.add_default_group(group);

    let svg_str = svg_out(svg);
    
    print!("{}\n", svg_str);

    let contents = include_str!("../showcase/example/logo.svg");

    assert_eq!(svg_str, contents);
}
