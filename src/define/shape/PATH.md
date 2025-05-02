### Path: defined by path string

Create: svg.add_shape(Shape::Path(Path::new()))

Usage: use path actions to construct a path, svg.moveto/lineto/....

#### Path showcase: use absolute moveto to create a path.

```
use simple_svg::*;

let mut svg = Svg::new(30.0, 70.0);

let mut path = Path::new();

path.moveto_abs((20.0, 60.0));

let path_id = svg.add_shape(Shape::Path(path));

let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

let mut path_sstyle = Sstyle::new();
path_sstyle.stroke = Some("#7AA20D".to_string());
path_sstyle.stroke_width = Some(1.0);

let mut circle_sstyle = Sstyle::new();
circle_sstyle.fill = Some("red".to_string());

let mut group = Group::new();

group.place_widget(Widget {
    shape_id: path_id,
    style: Some(path_sstyle),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id,
    style: Some(circle_sstyle),
    at: Some((20.0, 60.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/path/moveto1.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/path/moveto1.svg)

#### Path showcase: use relative moveto to create a path.

```
use simple_svg::*;

let mut svg = Svg::new(30.0, 70.0);

let mut path = Path::new();

path.moveto_rel((20.0, 60.0));

let path_id = svg.add_shape(Shape::Path(path));

let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

let mut path_sstyle = Sstyle::new();
path_sstyle.stroke = Some("#7AA20D".to_string());
path_sstyle.stroke_width = Some(1.0);

let mut circle_sstyle = Sstyle::new();
circle_sstyle.fill = Some("red".to_string());

let mut group = Group::new();

group.place_widget(Widget {
    shape_id: path_id,
    style: Some(path_sstyle),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id,
    style: Some(circle_sstyle),
    at: Some((20.0, 60.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/path/moveto2.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/path/moveto2.svg)

#### Path showcase: use moveto, lineto, close to create a path.

```
use simple_svg::*;

let mut svg = Svg::new(110.0, 160.0);

let mut path = Path::new();

path.moveto_abs((5.0, 5.0));
path.lineto_hor(100.0);
path.lineto_ver(100.0);
path.lineto_rel((-50.0, 50.0));
path.lineto_rel((-50.0, -50.0));
path.close();

let path_id = svg.add_shape(Shape::Path(path));

let mut path_sstyle = Sstyle::new();
path_sstyle.stroke = Some("#7AA20D".to_string());
path_sstyle.stroke_width = Some(5.0);
path_sstyle.stroke_linejoin = Some(LineJoin::Round);

let mut group = Group::new();
group.place_widget(Widget {
    shape_id: path_id,
    style: Some(path_sstyle),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/path/lineto.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/path/lineto.svg)

#### Path showcase: create a arc path.

```
use simple_svg::*;

let mut svg = Svg::new(300.0, 130.0);

let mut arc1 = Path::new();
arc1.moveto_abs((130.0, 45.0));
arc1.arc_abs((170.0, 85.0), (80.0, 40.0), ArcDirection::LeftBig);
let arc1_id = svg.add_shape(Shape::Path(arc1));

let mut arc2 = Path::new();
arc2.moveto_abs((130.0, 45.0));
arc2.arc_abs((170.0, 85.0), (80.0, 40.0), ArcDirection::LeftSmall);
let arc2_id = svg.add_shape(Shape::Path(arc2));

let mut arc3 = Path::new();
arc3.moveto_abs((130.0, 45.0));
arc3.arc_abs((170.0, 85.0), (80.0, 40.0), ArcDirection::RightBig);
let arc3_id = svg.add_shape(Shape::Path(arc3));

let mut arc4 = Path::new();
arc4.moveto_abs((130.0, 45.0));
arc4.arc_abs((170.0, 85.0), (80.0, 40.0), ArcDirection::RightSmall);
let arc4_id = svg.add_shape(Shape::Path(arc4));

let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

let mut group = Group::new();

let mut arc_sstyle = Sstyle::new();
arc_sstyle.stroke_width = Some(3.0);

let mut arc1_sstyle = arc_sstyle.clone();
arc1_sstyle.stroke = Some("#ccccff".to_string());
group.place_widget(Widget {
    shape_id: arc1_id,
    style: Some(arc1_sstyle),
    ..Default::default()
});

let mut arc2_sstyle = arc_sstyle.clone();
arc2_sstyle.stroke = Some("green".to_string());
group.place_widget(Widget {
    shape_id: arc2_id,
    style: Some(arc2_sstyle),
    ..Default::default()
});

let mut arc3_sstyle = arc_sstyle.clone();
arc3_sstyle.stroke = Some("blue".to_string());
group.place_widget(Widget {
    shape_id: arc3_id,
    style: Some(arc3_sstyle),
    ..Default::default()
});

let mut arc4_sstyle = arc_sstyle.clone();
arc4_sstyle.stroke = Some("yellow".to_string());
group.place_widget(Widget {
    shape_id: arc4_id,
    style: Some(arc4_sstyle),
    ..Default::default()
});

let mut circle_sstyle = Sstyle::new();
circle_sstyle.fill = Some("red".to_string());

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((130.0, 45.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((170.0, 85.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/path/arc.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/path/arc.svg)

#### Path showcase: create a qcurve path(absolute locate).

```
use simple_svg::*;

let mut svg = Svg::new(220.0, 120.0);

let mut path = Path::new();
path.moveto_abs((10.0, 60.0));
path.qcurve_abs((60.0, 10.0), (110.0, 60.0));
path.qcurve_abs((160.0, 110.0), (210.0, 60.0));

let qcurve_id = svg.add_shape(Shape::Path(path));

let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

let mut group = Group::new();

let mut qcurve_sstyle = Sstyle::new();
qcurve_sstyle.stroke_width = Some(3.0);
qcurve_sstyle.stroke = Some("#333333".to_string());
group.place_widget(Widget {
    shape_id: qcurve_id,
    style: Some(qcurve_sstyle),
    ..Default::default()
});

let mut circle_sstyle = Sstyle::new();
circle_sstyle.fill = Some("red".to_string());

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((10.0, 60.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((60.0, 10.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((110.0, 60.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((160.0, 110.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((210.0, 60.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/path/qcurve1.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/path/qcurve1.svg)

#### Path showcase: create a qcurve path(relative locate).

```
use simple_svg::*;

let mut svg = Svg::new(220.0, 120.0);

let mut path = Path::new();
path.moveto_abs((10.0, 60.0));
path.qcurve_rel((50.0, -50.0), (100.0, 0.0));
path.qcurve_rel((50.0, 50.0), (100.0, 0.0));

let qcurve_id = svg.add_shape(Shape::Path(path));

let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

let mut group = Group::new();

let mut qcurve_sstyle = Sstyle::new();
qcurve_sstyle.stroke_width = Some(3.0);
qcurve_sstyle.stroke = Some("#333333".to_string());
group.place_widget(Widget {
    shape_id: qcurve_id,
    style: Some(qcurve_sstyle),
    ..Default::default()
});

let mut circle_sstyle = Sstyle::new();
circle_sstyle.fill = Some("red".to_string());

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((10.0, 60.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((60.0, 10.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((110.0, 60.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((160.0, 110.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((210.0, 60.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/path/qcurve2.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/path/qcurve2.svg)

#### Path showcase: create a ccurve path(absolute locate).

```
use simple_svg::*;

let mut svg = Svg::new(200.0, 120.0);

let mut path = Path::new();
path.moveto_abs((10.0, 60.0));
path.ccurve_abs((30.0, 15.0), (80.0, 15.0), (100.0, 60.0));
path.ccurve_abs((120.0, 105.0), (170.0, 105.0), (190.0, 60.0));
let ccurve_id = svg.add_shape(Shape::Path(path));

let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

let mut group = Group::new();

let mut ccurve_sstyle = Sstyle::new();
ccurve_sstyle.stroke_width = Some(3.0);
ccurve_sstyle.stroke = Some("#333333".to_string());
group.place_widget(Widget {
    shape_id: ccurve_id,
    style: Some(ccurve_sstyle),
    ..Default::default()
});

let mut circle_sstyle = Sstyle::new();
circle_sstyle.fill = Some("red".to_string());

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((10.0, 60.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((30.0, 15.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((80.0, 15.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((100.0, 60.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((120.0, 105.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((170.0, 105.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((190.0, 60.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/path/ccurve1.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/path/ccurve1.svg)

#### Path showcase: create a ccurve path(relative locate).

```
use simple_svg::*;

let mut svg = Svg::new(200.0, 120.0);

let mut path = Path::new();
path.moveto_abs((10.0, 60.0));
path.ccurve_rel((20.0, -45.0), (70.0, -45.0), (90.0, 0.0));
path.ccurve_rel((20.0, 45.0), (70.0, 45.0), (90.0, 0.0));

let ccurve_id = svg.add_shape(Shape::Path(path));

let circle_id = svg.add_shape(Shape::Circle(Circle::new(5.0)));

let mut group = Group::new();

let mut ccurve_sstyle = Sstyle::new();
ccurve_sstyle.stroke_width = Some(3.0);
ccurve_sstyle.stroke = Some("#333333".to_string());
group.place_widget(Widget {
    shape_id: ccurve_id,
    style: Some(ccurve_sstyle),
    ..Default::default()
});

let mut circle_sstyle = Sstyle::new();
circle_sstyle.fill = Some("red".to_string());

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((10.0, 60.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((30.0, 15.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((80.0, 15.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((100.0, 60.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((120.0, 105.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((170.0, 105.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: circle_id.clone(),
    style: Some(circle_sstyle.clone()),
    at: Some((190.0, 60.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/path/ccurve2.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/path/ccurve2.svg)

#### Path showcase: create a raw path.

```
use simple_svg::*;

let mut svg = Svg::new(263.0, 188.0);

let mut path = Path::new();

path.raw(
    "M248.761,92c0,9.801-7.93,17.731-17.71,17.731c-0.319,0-0.617,0-0.935-0.021".to_string(),
);
path.raw(
    "c-10.035,37.291-51.174,65.206-100.414,65.206 c-49.261,0-90.443-27.979-100.435-65.334"
        .to_string(),
);
path.raw("c-0.765,0.106-1.531,0.149-2.317,0.149c-9.78,0-17.71-7.93-17.71-17.731".to_string());
path.raw("c0-9.78,7.93-17.71,17.71-17.71c0.787,0,1.552,0.042,2.317,0.149".to_string());
path.raw(
    "C39.238,37.084,80.419,9.083,129.702,9.083c49.24,0,90.379,27.937,100.414,65.228h0.021"
        .to_string(),
);
path.raw(
    "c0.298-0.021,0.617-0.021,0.914-0.021C240.831,74.29,248.761,82.22,248.761,92z".to_string(),
);

let path_id = svg.add_shape(Shape::Path(path));

let mut path_sstyle = Sstyle::new();
path_sstyle.stroke = Some("#7AA20D".to_string());
path_sstyle.fill = Some("#7AA20D".to_string());
path_sstyle.stroke_width = Some(9.0);
path_sstyle.stroke_linejoin = Some(LineJoin::Round);

let mut group = Group::new();
group.place_widget(Widget {
    shape_id: path_id,
    style: Some(path_sstyle),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/path/raw_path.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/path/raw_path.svg)
