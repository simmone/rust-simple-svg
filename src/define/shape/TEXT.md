### Text: defined by text string

Create: svg.add_shape(Shape::Text(Text::new(string)))

#### Text showcase: create a text.

```
use simple_svg::*;

let mut svg = Svg::new(310.0, 70.0);

let mut text = Text::new("城春草木深".to_string());
text.font_size = Some(50.0);

let text_id = svg.add_shape(Shape::Text(text));

let mut text_sstyle = Sstyle::new();
text_sstyle.fill = Some("#ED6E46".to_string());

let mut group = Group::new();
group.place_widget(Widget {
    shape_id: text_id,
    style: Some(text_sstyle),
    at: Some((30.0, 50.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/text/text1.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/text/text1.svg)

#### Text showcase: create a text, give it rotate effect by a vector of degree.

```
use simple_svg::*;

let mut svg = Svg::new(350.0, 120.0);

let mut text = Text::new("城春草木深".to_string());
text.font_size = Some(50.0);
text.rotate = Some(vec![10.0, 20.0, 30.0, 40.0, 50.0]);
text.text_length = Some(300.0);

let text_id = svg.add_shape(Shape::Text(text));

let mut text_sstyle = Sstyle::new();
text_sstyle.fill = Some("#ED6E46".to_string());

let mut group = Group::new();
group.place_widget(Widget {
    shape_id: text_id,
    style: Some(text_sstyle),
    at: Some((30.0, 60.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/text/text2.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/text/text2.svg)

#### Text showcase: create a text with decoration.

```
use simple_svg::*;

let mut svg = Svg::new(310.0, 280.0);

let mut text1 = Text::new("国破山河在".to_string());
text1.font_size = Some(50.0);
text1.text_decoration = Some(TextDecoration::OverLine);
let text1_id = svg.add_shape(Shape::Text(text1));

let mut text2 = Text::new("国破山河在".to_string());
text2.font_size = Some(50.0);
text2.text_decoration = Some(TextDecoration::UnderLine);
let text2_id = svg.add_shape(Shape::Text(text2));

let mut text3 = Text::new("国破山河在".to_string());
text3.font_size = Some(50.0);
text3.text_decoration = Some(TextDecoration::LineThrough);
let text3_id = svg.add_shape(Shape::Text(text3));

let mut text_sstyle = Sstyle::new();
text_sstyle.fill = Some("#ED6E46".to_string());

let mut group = Group::new();
group.place_widget(Widget {
    shape_id: text1_id,
    style: Some(text_sstyle.clone()),
    at: Some((30.0, 60.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: text2_id,
    style: Some(text_sstyle.clone()),
    at: Some((30.0, 160.0)),
    ..Default::default()
});

group.place_widget(Widget {
    shape_id: text3_id,
    style: Some(text_sstyle.clone()),
    at: Some((30.0, 260.0)),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/text/text3.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/text/text3.svg)

#### Text showcase: create a text, follow with a path.

```
use simple_svg::*;

let mut svg = Svg::new(500.0, 100.0);

let mut path = Path::new();
path.moveto_abs((10.0, 60.0));
path.qcurve_abs((110.0, 10.0), (210.0, 60.0));
path.qcurve_abs((310.0, 110.0), (410.0, 60.0));
let qcurve_id = svg.add_shape(Shape::Path(path));

let mut text = Text::new("国破山河在 城春草木深 感时花溅泪 恨别鸟惊心".to_string());
text.path = Some(qcurve_id);
text.path_start_offset = Some(5.0);
let text_id = svg.add_shape(Shape::Text(text));
let mut text_sstyle = Sstyle::new();
text_sstyle.fill = Some("#ED6E46".to_string());

let mut group = Group::new();
group.place_widget(Widget {
    shape_id: text_id,
    style: Some(text_sstyle),
    ..Default::default()
});

svg.add_default_group(group);

let svg_str = svg_out(svg);

let contents = include_str!("../../../showcase/text/text4.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../../showcase/text/text4.svg)
