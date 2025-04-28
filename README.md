# Svg Image Writer for Rust

The project provides abilities to generate basic svg shape/group image.

thanks to Joni's tutorial: [SVG Pocket Guide](http://svgpocketguide.com/)

## Getting Started

This library's target is to generate combined shapes progmatically.

## Usage(check showcases to see the detail)

Basic steps:
1. create canvas: Svg::new(with, height);
2. create shape and add to canvas: svg.add_shape(Shape::Rect(Rect::new(...;
3. create a style for this shape: Sstyle::new();
4. set style: sstyle.fill = Some(#BBC42A".to_string());
4. create a group: Group::new();
5. place the shape in the group in somewhere, and set its style: group.place_widget(Widget(shape_id:..., at:..., style:..., ..Default::default()));
6. set group as default group;
7. use svg_out to generate svg content;

![](showcase/shapes/rect/rect.svg)
![](../../../showcase/shapes/rect/rect.svg)

## License

MIT See [LICENSE](../../../LICENSE) file.

## Showcases

### Rectangle

```
use simple_svg::*;

let mut svg = Svg::new(100.0, 100.0);

let rect_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));

let mut rect_sstyle = Sstyle::new();
rect_sstyle.fill = Some("#BBC42A".to_string());

let mut group = Group::new();
group.place_widget(Widget {
    shape_id: rect_id,
    style: Some(rect_sstyle),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../showcase/shapes/rect/rect.svg");

assert_eq!(svg_str, contents);
```

![](showcase/shapes/rect/rect.svg)
![](../../../showcase/shapes/rect/rect.svg)


### Recursive svg image:

```
use simple_svg::*;

fn get_circles(x: f64, y: f64, radius: f64, mut circles: &mut Vec<(f64, f64, f64)>) {
    circles.push((x, y, radius));

    if radius > 8.0 {
        get_circles(x + radius, y, radius / 2.0, &mut circles);
        get_circles(x - radius, y, radius / 2.0, &mut circles);
        get_circles(x, y + radius, radius / 2.0, &mut circles);
        get_circles(x, y - radius, radius / 2.0, &mut circles);
    }
}

fn recursive_test() {
    let mut svg = Svg::new(400.0, 400.0);

    let mut sstyle = Sstyle::new();
    sstyle.stroke_width = Some(1.0);
    sstyle.stroke = Some("red".to_string());

    let mut circles: Vec<(f64, f64, f64)> = vec![];

    get_circles(200.0, 200.0, 100.0, &mut circles);

    let mut group = Group::new();

    for circle in circles {
        let circle_id = svg.add_shape(Shape::Circle(Circle::new(circle.2)));

        group.place_widget(Widget {
            shape_id: circle_id.to_string(),
            at: Some((circle.0, circle.1)),
            ..Default::default()
        });
    }

    let group_id = svg.add_group(group);

    let mut default_group = Group::new();

    default_group.place_widget(Widget {
        shape_id: group_id,
        style: Some(sstyle.clone()),
        ..Default::default()
    });

    svg.add_default_group(default_group);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/example/recursive.svg");

    assert_eq!(svg_str, contents);
}
```

![](showcase/example/recursive.svg)
![](../../../showcase/example/recursive.svg)

