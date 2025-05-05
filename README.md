# Svg Image Writer for Rust

The project provides abilities to generate basic svg shape/group image.

thanks to Joni's tutorial: [SVG Pocket Guide](http://svgpocketguide.com/)

## Getting Started

This library's target is to generate combined shapes progmatically.

## Usage

Basic steps:
1. create canvas, specify width and height: Svg::new(with, height);
2. create shape and add to canvas: svg.add_shape(Shape::Rect(Rect::new(...;
3. create a style for this shape/group: Sstyle::new();
4. create a group: Group::new();
5. place the shape/group id in the group in somewhere, some style: group.place_widget(Widget(shape_id:..., at:..., style:..., ..Default::default()));
6. set group as default group;
7. use svg_out to generate svg content;

Basic component usage please check the rustdoc.

## Showcase: Five circles

```
use simple_svg::*;

/// define canvas size
let mut svg = Svg::new(500.0, 300.0);

/// create a circle with radius is 60.0
let circle_id = svg.add_shape(Shape::Circle(Circle::new(60.0)));

/// create a dropdown blur filter
let filter_id = svg.add_shape(Shape::Filter(Filter::new()));

/// create a style, set circle's stroke width is 12.0
let mut circle_sstyle = Sstyle::new();
circle_sstyle.stroke_width = Some(12.0);

/// each circle have same stroke width, different color
let mut circle1_sstyle = circle_sstyle.clone();
circle1_sstyle.stroke = Some("rgb(11, 112, 191)".to_string());

let mut circle2_sstyle = circle_sstyle.clone();
circle2_sstyle.stroke = Some("rgb(240, 183, 0)".to_string());

let mut circle3_sstyle = circle_sstyle.clone();
circle3_sstyle.stroke = Some("rgb(0, 0, 0)".to_string());

let mut circle4_sstyle = circle_sstyle.clone();
circle4_sstyle.stroke = Some("rgb(13, 146, 38)".to_string());

let mut circle5_sstyle = circle_sstyle.clone();
circle5_sstyle.stroke = Some("rgb(214, 0, 23)".to_string());

/// create a group to place each circle with style and filter, at different point
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

/// add this group to default group
svg.add_default_group(group);

let svg_str = svg_out(svg);

println!("{}", svg_str);

let contents = include_str!("../showcase/example/five_circles.svg");

assert_eq!(svg_str, contents);
```

![](../../../showcase/example/five_circles.svg)

## Showcase: Recursive circles

Start a fixed size circle, then paint four circles with half radious length at up, down, left, right directions, and so on, till radius length is minimal enough.

[Code](https://github.com/simmone/rust-simple-svg/blob/master/tests/recursive_test.rs)

![](../../../showcase/example/recursive.svg)

## Showcase: Recursive fern

Thanks to the author: Matteo d'Addio matteo.daddio@live.it)

[Code](https://github.com/simmone/rust-simple-svg/blob/master/tests/fern_test.rs)

![](../../../showcase/example/fern.svg)

## License

MIT See [LICENSE](../../../LICENSE) file.
