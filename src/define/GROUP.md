### Group: group shapes or other groups, reuse it. Group is a reusable set.

Create: Group::new()

Place shape or group in it: group.place_widget(shape_id:..., style:..., at:..., ..Default::default())

#### Group showcase: create a flare group, reuse it.

```
use simple_svg::*;

let mut svg = Svg::new(220.0, 280.0);

let line1_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (30.0, 30.0))));
let line2_id = svg.add_shape(Shape::Line(Line::new((0.0, 15.0), (30.0, 15.0))));
let line3_id = svg.add_shape(Shape::Line(Line::new((15.0, 0.0), (15.0, 30.0))));
let line4_id = svg.add_shape(Shape::Line(Line::new((30.0, 0.0), (0.0, 30.0))));

let mut line_sstyle = Sstyle::new();
line_sstyle.stroke = Some("#765373".to_string());
line_sstyle.stroke_width = Some(5.0);

let mut group = Group::new();

group.place_widget(Widget {
    shape_id: line1_id,
    style: Some(line_sstyle.clone()),
    at: Some((5.0, 5.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: line2_id,
    style: Some(line_sstyle.clone()),
    at: Some((5.0, 5.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: line3_id,
    style: Some(line_sstyle.clone()),
    at: Some((5.0, 5.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: line4_id,
    style: Some(line_sstyle.clone()),
    at: Some((5.0, 5.0)),
    ..Default::default()
});

let group_id = svg.add_group(group);

let mut default_group = Group::new();

default_group.place_widget(Widget {
    shape_id: group_id.clone(),
    at: Some((50.0, 50.0)),
    ..Default::default()
});

default_group.place_widget(Widget {
    shape_id: group_id.clone(),
    at: Some((100.0, 100.0)),
    ..Default::default()
});

default_group.place_widget(Widget {
    shape_id: group_id.clone(),
    at: Some((80.0, 200.0)),
    ..Default::default()
});

default_group.place_widget(Widget {
    shape_id: group_id.clone(),
    at: Some((150.0, 100.0)),
    ..Default::default()
});

svg.add_default_group(default_group);

let svg_str = svg_out(svg);

let contents = include_str!("../../showcase/group/group1.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../showcase/group/group1.svg)

#### Group showcase: create a pattern, reuse it.

```
use simple_svg::*;

let mut svg = Svg::new(100.0, 100.0);

let rect_id = svg.add_shape(Shape::Rect(Rect::new(50.0, 50.0)));

let line1_id = svg.add_shape(Shape::Line(Line::new((10.0, 0.0), (0.0, 50.0))));
let line2_id = svg.add_shape(Shape::Line(Line::new((0.0, 0.0), (10.0, 50.0))));

let mut line_sstyle = Sstyle::new();
line_sstyle.stroke = Some("black".to_string());
line_sstyle.stroke_width = Some(1.0);

let mut cross_line_group = Group::new();

cross_line_group.place_widget(Widget {
    shape_id: line1_id,
    style: Some(line_sstyle.clone()),
    ..Default::default()
});

cross_line_group.place_widget(Widget {
    shape_id: line2_id,
    style: Some(line_sstyle.clone()),
    ..Default::default()
});

let cross_line_group_id = svg.add_group(cross_line_group);

let mut rect_sstyle = Sstyle::new();
rect_sstyle.fill = Some("orange".to_string());
rect_sstyle.stroke = Some("red".to_string());
rect_sstyle.stroke_width = Some(2.0);

let mut pattern_group = Group::new();

pattern_group.place_widget(Widget {
    shape_id: rect_id,
    style: Some(rect_sstyle),
    ..Default::default()
});

pattern_group.place_widget(Widget {
    shape_id: cross_line_group_id.clone(),
    at: Some((0.0, 0.0)),
    ..Default::default()
});

pattern_group.place_widget(Widget {
    shape_id: cross_line_group_id.clone(),
    at: Some((10.0, 0.0)),
    ..Default::default()
});

pattern_group.place_widget(Widget {
    shape_id: cross_line_group_id.clone(),
    at: Some((20.0, 0.0)),
    ..Default::default()
});

pattern_group.place_widget(Widget {
    shape_id: cross_line_group_id.clone(),
    at: Some((30.0, 0.0)),
    ..Default::default()
});

pattern_group.place_widget(Widget {
    shape_id: cross_line_group_id.clone(),
    at: Some((40.0, 0.0)),
    ..Default::default()
});

let pattern_group_id = svg.add_group(pattern_group);

let mut default_group = Group::new();

default_group.place_widget(Widget {
    shape_id: pattern_group_id.clone(),
    at: Some((0.0, 0.0)),
    ..Default::default()
});

default_group.place_widget(Widget {
    shape_id: pattern_group_id.clone(),
    at: Some((50.0, 0.0)),
    ..Default::default()
});

default_group.place_widget(Widget {
    shape_id: pattern_group_id.clone(),
    at: Some((0.0, 50.0)),
    ..Default::default()
});

default_group.place_widget(Widget {
    shape_id: pattern_group_id.clone(),
    at: Some((50.0, 50.0)),
    ..Default::default()
});

svg.add_default_group(default_group);

let svg_str = svg_out(svg);

let contents = include_str!("../../showcase/group/group2.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../showcase/group/group2.svg)

#### Group showcase: create a five circle group, reuse it four times.

```
use simple_svg::*;

let mut svg = Svg::new(1000.0, 600.0);

let circle_id = svg.add_shape(Shape::Circle(Circle::new(60.0)));

let mut circle1_sstyle = Sstyle::new();
circle1_sstyle.stroke = Some("rgb(11, 112, 191)".to_string());

let mut circle2_sstyle = Sstyle::new();
circle2_sstyle.stroke = Some("rgb(240, 183, 0)".to_string());

let mut circle3_sstyle = Sstyle::new();
circle3_sstyle.stroke = Some("rgb(0, 0, 0)".to_string());

let mut circle4_sstyle = Sstyle::new();
circle4_sstyle.stroke = Some("rgb(13, 146, 38)".to_string());

let mut circle5_sstyle = Sstyle::new();
circle5_sstyle.stroke = Some("rgb(214, 0, 23)".to_string());

let mut five_circle_group = Group::new();

five_circle_group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle1_sstyle),
    at: Some((120.0, 120.0)),
    ..Default::default()
});

five_circle_group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle2_sstyle),
    at: Some((180.0, 180.0)),
    ..Default::default()
});

five_circle_group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle3_sstyle),
    at: Some((260.0, 120.0)),
    ..Default::default()
});

five_circle_group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle4_sstyle),
    at: Some((320.0, 180.0)),
    ..Default::default()
});

five_circle_group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle5_sstyle),
    at: Some((400.0, 120.0)),
    ..Default::default()
});

let five_circle_group_id = svg.add_group(five_circle_group);

let mut default_group = Group::new();

let filter_id = svg.add_shape(Shape::Filter(Filter::new()));

let mut group_sstyle = Sstyle::new();
group_sstyle.stroke_width = Some(12.0);

default_group.place_widget(Widget {
    shape_id: five_circle_group_id.clone(),
    style: Some(group_sstyle.clone()),
    filter_id: Some(filter_id.clone()),
    at: Some((0.0, 0.0)),
    ..Default::default()
});

default_group.place_widget(Widget {
    shape_id: five_circle_group_id.clone(),
    style: Some(group_sstyle.clone()),
    filter_id: Some(filter_id.clone()),
    at: Some((0.0, 300.0)),
    ..Default::default()
});

default_group.place_widget(Widget {
    shape_id: five_circle_group_id.clone(),
    style: Some(group_sstyle.clone()),
    filter_id: Some(filter_id.clone()),
    at: Some((500.0, 0.0)),
    ..Default::default()
});

default_group.place_widget(Widget {
    shape_id: five_circle_group_id.clone(),
    style: Some(group_sstyle.clone()),
    filter_id: Some(filter_id.clone()),
    at: Some((500.0, 300.0)),
    ..Default::default()
});

svg.add_default_group(default_group);

let svg_str = svg_out(svg);

let contents = include_str!("../../showcase/group/group3.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../showcase/group/group3.svg)
