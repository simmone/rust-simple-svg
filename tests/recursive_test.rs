use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn recursive_test() {
    let mut svg = Svg::new(400.0, 400.0);

    let mut sstyle = Sstyle::new();
    sstyle.stroke = Some("red".to_string());

    let mut group = Group::new();
    
    let mut paint_branch = 
        |mut x: f64, mut y: f64, mut radius: f64, x_func: &dyn Fn(f64, f64) -> f64, y_func: &dyn Fn(f64, f64) -> f64| {
            loop {
                if radius <= 8.0 { break; }
        
                let circle_id = svg.add_shape(Shape::Circle(Circle::new(radius)));

                group.place_widget(Widget {
                    shape_id: circle_id,
                    style: Some(sstyle.clone()),
                    at: Some((x, y)),
                    ..Default::default()
                });
                
                x = x_func(x, radius);
                
                y = y_func(y, radius);
                
                radius = radius / 2.0;
            }
        };

    let x_func = |x, radius| { x + radius };

    let y_func = |y, _| { y };

    paint_branch(200.0, 200.0, 100.0, &x_func, &y_func);

    svg.add_default_group(group);

    let svg_str = svg_out(svg);
    
    print!("{}\n", svg_str);
    
    assert_eq!(1, 2);

//    let contents = include_str!("../showcase/example/recursive.svg");

//    assert_eq!(svg_str, contents);
}
