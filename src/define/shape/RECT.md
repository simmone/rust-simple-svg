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

#### Rect showcase: create a rectangle, fill color, place into center, show the left right corner.

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

#### Rect showcase: create a radius corner rectangle

```
use simple_svg::*;

let mut svg = Svg::new(100.0, 100.0);

let mut rect = Rect::new(100.0, 100.0);
rect.radius_x = Some(5.0);
rect.radius_y = Some(10.0);

let rect_id = svg.add_shape(Shape::Rect(rect));

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

let contents = include_str!("../../../showcase/shapes/rect/rect_radius.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/shapes/rect/rect_radius.svg)

#### Rect showcase: create mutiple rectangle, fill different color, pile up.

```
use simple_svg::*;

let mut svg = Svg::new(150.0, 150.0);

let blue_rect_id = svg.add_shape(Shape::Rect(Rect::new(150.0, 150.0)));
let green_rect_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));
let red_rect_id = svg.add_shape(Shape::Rect(Rect::new(50.0, 50.0)));

let mut blue_rect_sstyle = Sstyle::new();
blue_rect_sstyle.fill = Some("blue".to_string());

let mut green_rect_sstyle = Sstyle::new();
green_rect_sstyle.fill = Some("green".to_string());

let mut red_rect_sstyle = Sstyle::new();
red_rect_sstyle.fill = Some("red".to_string());

let mut group = Group::new();

group.place_widget(Widget {
    shape_id: blue_rect_id,
    style: Some(blue_rect_sstyle),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: green_rect_id,
    style: Some(green_rect_sstyle),
    at: Some((25.0, 25.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: red_rect_id,
    style: Some(red_rect_sstyle),
    at: Some((50.0, 50.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/shapes/rect/m_rect.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/shapes/rect/m_rect.svg)

#### Rect showcase: create rectangle once, reuse it, place it at different place.

```
use simple_svg::*;

let mut svg = Svg::new(190.0, 190.0);

let blue_rect_id = svg.add_shape(Shape::Rect(Rect::new(50.0, 50.0)));

let mut blue_rect_sstyle = Sstyle::new();
blue_rect_sstyle.fill = Some("blue".to_string());

let mut group = Group::new();

group.place_widget(Widget {
    shape_id: blue_rect_id.clone(),
    style: Some(blue_rect_sstyle.clone()),
    at: Some((10.0, 10.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: blue_rect_id.clone(),
    style: Some(blue_rect_sstyle.clone()),
    at: Some((70.0, 70.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: blue_rect_id.clone(),
    style: Some(blue_rect_sstyle.clone()),
    at: Some((130.0, 130.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/shapes/rect/rect_reuse.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/shapes/rect/rect_reuse.svg)