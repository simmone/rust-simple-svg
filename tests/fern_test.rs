use pretty_assertions::assert_eq;
//use std::fs::File;
//use std::io::prelude::*;
use std::collections::HashMap;
use simple_svg::*;

fn get_end_point(start_point: (f64, f64), length: f64, deg: f64, precision: uszie) {
    circles.push((x, y, radius));

    if radius > 8.0 {
        get_circles(x + radius, y, radius / 2.0, &mut circles);
        get_circles(x - radius, y, radius / 2.0, &mut circles);
        get_circles(x, y + radius, radius / 2.0, &mut circles);
        get_circles(x, y - radius, radius / 2.0, &mut circles);
    }
}

#[test]
fn howto_fern_test() {
    let mut circles: Vec<(f64, f64, f64)> = vec![];

    get_circles(200.0, 200.0, 100.0, &mut circles);

    assert_eq!(circles.len(), 341);
}

#[test]
//fn fern_test() -> std::io::Result<()> {
fn fern_test() {
    let mut svg = Svg::new(400.0, 400.0);

    let mut sstyle = Sstyle::new();
    sstyle.stroke_width = Some(1.0);
    sstyle.stroke = Some("red".to_string());

    let mut group = Group::new();

    let mut circles: Vec<(f64, f64, f64)> = vec![];

    get_circles(200.0, 200.0, 100.0, &mut circles);
    
    let mut circle_id_map: HashMap<String, String> = HashMap::new();
    let mut loop_radius = 100.0;
    while loop_radius > 6.0 {
        let circle_id = svg.add_shape(Shape::Circle(Circle::new(loop_radius)));
        
        circle_id_map.insert(loop_radius.to_string(), circle_id);

        loop_radius = loop_radius / 2.0;
    }
    
    for circle in circles {
        let circle_id = circle_id_map.get(&(circle.2.to_string())).unwrap();

        group.place_widget(Widget {
            shape_id: circle_id.to_string(),
            style: Some(sstyle.clone()),
            at: Some((circle.0, circle.1)),
            ..Default::default()
        });
    }

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

//    let mut file = File::create("resursive.svg")?;

//    file.write(svg_str.as_bytes())?;

//    Ok(())

    let contents = include_str!("../showcase/example/fern.svg");

    assert_eq!(svg_str, contents);
}
