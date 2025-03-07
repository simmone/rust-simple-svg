use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn recursive_test() {
    let mut svg = Svg::new(400.0, 400.0);

    let mut sstyle = Sstyle::new();
    sstyle.stroke = Some("red".to_string());

    let mut group = Group::new();
    
    let recursive_closure = |x: f64, y: f64, radius: f64| {
        let circle_id = svg.add_shape(Shape::Circle(Circle::new(radius)));

        group.place_widget(Widget {
            shape_id: circle_id,
            style: Some(sstyle.clone()),
            at: Some((x, y)),
            ..Default::default()
        });
        
        if radius > 8.0 {
            recursive_closure(x + radius, y, radius / 2.0);
            recursive_closure(x - radius, y, radius / 2.0);
            recursive_closure(x, y + radius, radius / 2.0);
            recursive_closure(x, y - radius, radius / 2.0);
         }
    };
    
    recursive_closure(200.0, 200.0, 100.0);

    svg.add_default_group(group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/example/recursive.svg");

    assert_eq!(svg_str, contents);
}
