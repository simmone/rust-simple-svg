### Gradient: LinearGradient and RadialGradient

Create: svg.add_shape(Shape::LinearGradient(LinearGradient::new(Vec<(f64, String, f64)>)))

Create: svg.add_shape(Shape::RadialGradient(RadialGradient::new(Vec<(f64, String, f64)>)))

Usage: gradient is a fill_gradient property of style.

#### LinearGradient showcase: give a lineargradient to a rectangle.

```
use simple_svg::*;

let mut svg = Svg::new(100.0, 100.0);

let rect_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));

let gradient_id = svg.add_shape(Shape::LinearGradient(LinearGradient::new(vec![
    (0.0, "#BBC42A".to_string(), 1.0),
    (100.0, "#ED6E46".to_string(), 1.0),
])));

let mut rect_sstyle = Sstyle::new();
rect_sstyle.fill_gradient = Some(gradient_id);

let mut group = Group::new();
group.place_widget(Widget {
    shape_id: rect_id,
    style: Some(rect_sstyle),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/gradient/linear_gradient.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/gradient/linear_gradient.svg)

#### RadialGradient showcase: give a radialgradient to a rectangle.

```
use simple_svg::*;

let mut svg = Svg::new(100.0, 100.0);

let rect_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));

let gradient_id = svg.add_shape(Shape::RadialGradient(RadialGradient::new(vec![
    (0.0, "#BBC42A".to_string(), 1.0),
    (100.0, "#ED6E46".to_string(), 1.0),
])));

let mut rect_sstyle = Sstyle::new();
rect_sstyle.fill_gradient = Some(gradient_id);

let mut group = Group::new();
group.place_widget(Widget {
    shape_id: rect_id,
    style: Some(rect_sstyle),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/gradient/radial_gradient.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/gradient/radial_gradient.svg)
