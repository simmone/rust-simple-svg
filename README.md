# Svg Image File Writer for Rust

The project provides abilities to generate basic svg shape/group image.

thanks to Joni's tutorial: [SVG Pocket Guide](http://svgpocketguide.com/)

## Getting Started

This library's target is to generate combined shapes progmatically.

## Examples and Usage

Basic steps(ie: create a rectangle):
1. create canvas: let mut svg = Svg::new(with, height);
2. create shape: svg.add_shape(Shape::Rect(Rect::new(...;
3. create a style: Sstyle::new();
4. set style: sstyle.fill = Some(...;
4. create a group: Group::new();
5. place the shape in the group and set style: group.place_widget(Widget(shape_id:..., style:..., ..Default::default()))
6. set group as default group;
7. use svg_out to generate svg content;

```
use std::fs::File;
use std::io::prelude::*;
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

let mut file = File::create("target/doc/simple_svg/rect.svg").unwrap();

file.write(svg_str.as_bytes()).unwrap();

```

![](showcase/shapes/rect/rect.svg)

![](target/doc/simple_svg/rect.svg)
