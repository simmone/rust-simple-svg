### Svg: canvas, defined by width, height, background, view_box.

Create: Svg::new(width, height)

#### Svg showcase: create a empty svg.

```
use simple_svg::*;

let svg = Svg::new(30.0, 20.0);

let svg_str = svg_out(svg);

let contents = include_str!("../../showcase/basic/empty.svg");

assert_eq!(svg_str, contents);

```

![](../../../../../showcase/basic/empty.svg)

#### Svg showcase: create a svg with a background color.

```
use simple_svg::*;

let mut svg = Svg::new(100.0, 100.0);
svg.set_background("#BBC42A".to_string());

let svg_str = svg_out(svg);

let contents = include_str!("../../showcase/basic/background.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../showcase/basic/background.svg)

#### Svg showcase: create a svg with a background color, place a rectangle in center.

```
use simple_svg::*;

let mut svg = Svg::new(100.0, 100.0);
svg.set_background("#BBC42A".to_string());

let rect_id = svg.add_shape(Shape::Rect(Rect::new(50.0, 50.0)));

let mut rect_sstyle = Sstyle::new();
rect_sstyle.fill = Some("#FFFFFF".to_string());

let mut group = Group::new();
group.place_widget(Widget {
    shape_id: rect_id,
    style: Some(rect_sstyle),
    at: Some((25.0, 25.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../showcase/basic/rect_in_background.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../showcase/basic/rect_in_background.svg)

#### Svg showcase: create a svg, set viewBox, show a part of rectangle.

```
use simple_svg::*;

let mut svg = Svg::new(100.0, 100.0);
svg.set_viewbox(50.0, 0.0, 100.0, 100.0);

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

let contents = include_str!("../../showcase/basic/viewBox.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../showcase/basic/viewBox.svg)

