### Table: paint a two dimension's vector content.

Create: Table::new()

Then table.to_group(), svg.add_group(table_group)

#### Table showcase: show a Vec<Vec<&str>>

```
use simple_svg::*;

let mut svg = Svg::new(250.0, 200.0);

let table = Table::new();

let table_group = table.to_group(
    &mut svg,
    &vec![
        vec!["1", "2", "3"],
        vec!["4", "5", "6"],
        vec!["7", "8", "9"],
    ],
);

let table_id = svg.add_group(table_group);

let mut default_group = Group::new();

default_group.place_widget(Widget {
    shape_id: table_id.clone(),
    at: Some((50.0, 50.0)),
    ..Default::default()
});

svg.add_default_group(default_group);

let svg_str = svg_out(svg);

eprintln!("{svg_str}");

let contents = include_str!("../../showcase/gadget/table/table1.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../showcase/gadget/table/table1.svg)

#### Table showcase: show a Vec<Vec<&str>>, set col width, row height, cell margin left and margin top.

```
use simple_svg::*;

let mut svg = Svg::new(250.0, 200.0);

let mut table = Table::new();
table.set_table_col_width(vec![1], 80.0);
table.set_table_row_height(vec![1], 50.0);
table.set_table_col_margin_left(vec![1], 35.0);
table.set_table_row_margin_top(vec![1], 30.0);

let table_group = table.to_group(
    &mut svg,
    &vec![
        vec!["1", "2", "3"],
        vec!["4", "5", "6"],
        vec!["7", "8", "9"],
    ],
);

let table_id = svg.add_group(table_group);

let mut default_group = Group::new();

default_group.place_widget(Widget {
    shape_id: table_id.clone(),
    at: Some((50.0, 50.0)),
    ..Default::default()
});

svg.add_default_group(default_group);

let svg_str = svg_out(svg);

let contents = include_str!("../../showcase/gadget/table/table2.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../showcase/gadget/table/table2.svg)

#### Table showcase: show a Vec<Vec<&str>>, initial parameters, set cell font size and color.

```
use simple_svg::*;

let mut svg = Svg::new(400.0, 300.0);

let mut table = Table::new();
table.col_width = 100.0;
table.row_height = 60.0;
table.color = "green".to_string();
table.font_color = "blue".to_string();
table.cell_margin_top = 44.0;
table.cell_margin_left = 40.0;

table.set_table_cell_font_size(vec![(0, 0), (1, 1), (2, 2)], 40.0);
table.set_table_cell_font_color(vec![(0, 0), (1, 1), (2, 2)], "red".to_string());

let table_group = table.to_group(
    &mut svg,
    &vec![
        vec!["1", "2", "3"],
        vec!["4", "5", "6"],
        vec!["7", "8", "9"],
    ],
);

let table_id = svg.add_group(table_group);

let mut default_group = Group::new();

default_group.place_widget(Widget {
    shape_id: table_id.clone(),
    at: Some((50.0, 50.0)),
    ..Default::default()
});

svg.add_default_group(default_group);

let svg_str = svg_out(svg);

let contents = include_str!("../../showcase/gadget/table/table3.svg");

assert_eq!(svg_str, contents);
```

![](../../../../../showcase/gadget/table/table3.svg)

