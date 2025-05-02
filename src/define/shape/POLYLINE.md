### Polyline: defined by vector of points.

Create: svg.add_shape(Shape::Polyline(Polyline::new(vec![(x, y), (x, y)...])))

#### Polyline showcase: create a polyline.

```
use simple_svg::*;

let mut svg = Svg::new(130.0, 130.0);

let polyline_id = svg.add_shape(Shape::Polyline(Polyline::new(vec![
    (0.0, 0.0),
    (40.0, 0.0),
    (40.0, 40.0),
    (80.0, 40.0),
    (80.0, 80.0),
    (120.0, 80.0),
    (120.0, 120.0),
])));

let mut polyline_sstyle = Sstyle::new();
polyline_sstyle.fill = Some("blue".to_string());
polyline_sstyle.stroke = Some("#BBC42A".to_string());
polyline_sstyle.stroke_width = Some(5.0);

let mut group = Group::new();
group.place_widget(Widget {
    shape_id: polyline_id,
    style: Some(polyline_sstyle),
    at: Some((5.0, 5.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/shapes/polyline/polyline.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/shapes/polyline/polyline.svg)
