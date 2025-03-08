use pretty_assertions::assert_eq;
use std::fs::File;
use std::io::prelude::*;
use simple_svg::*;

fn recursive_it(x: f64, y: f64, radius: f64) -> usize {
    let mut count = 1;

    let mut loop_radius = radius;
    let mut loop_x = x;
    let mut loop_y = y;

    print!("{}, {}, {}, {}\n", loop_x, loop_y, loop_radius, count);
    
    while loop_radius > 8.0 {
        loop_radius = loop_radius / 2.0;

        count = count + 1;

        print!("{}, {}, {}, {}\n", loop_x, loop_y, loop_radius, count);
        
        loop_left_x_radius = loop_x - loop_radius;
        
        while 
    }
        
    count
}

#[test]
fn howto_recursive_test() {
    let x = 200.0;
    let y = 200.0;
    let radius = 100.0;
    
    assert_eq!(0, recursive_it(x, y, radius));
}

#[test]
fn recursive_test() -> std::io::Result<()> {
    let mut svg = Svg::new(400.0, 400.0);

    let mut sstyle = Sstyle::new();
    sstyle.stroke = Some("red".to_string());

    let mut group = Group::new();
    
    let mut paint_branch = 
        |mut x: f64, mut y: f64, mut radius: f64, x_func: &dyn Fn(f64, f64) -> f64, y_func: &dyn Fn(f64, f64) -> f64| {
            loop {
                let circle_id = svg.add_shape(Shape::Circle(Circle::new(radius)));

                group.place_widget(Widget {
                    shape_id: circle_id,
                    style: Some(sstyle.clone()),
                    at: Some((x, y)),
                    ..Default::default()
                });

                if radius > 8.0 {
                    x = x_func(x, radius);
                
                    y = y_func(y, radius);
                
                    radius = radius / 2.0;
                } else {
                    break;
                }
            }
        };

    let x_right_func = |x, radius| { x + radius };
    let y_same_func = |y, _| { y };
    paint_branch(200.0, 200.0, 100.0, &x_right_func, &y_same_func);

    let x_left_func = |x, radius| { x - radius };
    paint_branch(200.0, 200.0, 100.0, &x_left_func, &y_same_func);

    let x_same_func = |x, _| { x };
    let y_right_func = |y, radius| { y + radius };
    paint_branch(200.0, 200.0, 100.0, &x_same_func, &y_right_func);

    let y_left_func = |y, radius| { y - radius };
    paint_branch(200.0, 200.0, 100.0, &x_same_func, &y_left_func);

    svg.add_default_group(group);

    let svg_str = svg_out(svg);
    
    let mut file = File::create("test.svg")?;
    
    file.write(svg_str.as_bytes())?;
    
    Ok(())
    
//    let contents = include_str!("../showcase/example/recursive.svg");

//    assert_eq!(svg_str, contents);
}
