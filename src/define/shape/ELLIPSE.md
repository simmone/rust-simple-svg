### Ellipse: defined by radius_x, radius_y.

Create: svg.add_shape(Shape::Ellipse(Ellipse::new(radius_x, radius_y)))

#### Ellipse showcase: create a ellipse.

```
use simple_svg::*;

let mut svg = Svg::new(200.0, 100.0);

let ellipse_id = svg.add_shape(Shape::Ellipse(Ellipse::new(100.0, 50.0)));

let mut ellipse_sstyle = Sstyle::new();
ellipse_sstyle.fill = Some("#7AA20D".to_string());

let mut group = Group::new();
group.place_widget(Widget {
    shape_id: ellipse_id,
    style: Some(ellipse_sstyle),
    at: Some((100.0, 50.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/shapes/ellipse/ellipse.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/shapes/ellipse/ellipse.svg)
