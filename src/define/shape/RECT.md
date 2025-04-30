### Rect: rectangle, defined by width and height.

Create: svg.add_shape(Shape::Rect(Rect::new(width, height)))

#### Rect showcase: create a rectangle, fill color.

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

let contents = include_str!("../../../showcase/shapes/rect/rect.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/shapes/rect/rect.svg)

#### Rect showcase: create a rectangle, fill color, place into center.

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
    at: Some((50.0, 50.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/shapes/rect/rect_y.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/shapes/rect/rect_y.svg)