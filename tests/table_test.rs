use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn table1_test() {
    let mut svg = Svg::new(250.0, 200.0);

    let mut table = Table::new();

    let table_id = svg.add_group(table);

    let mut default_group = Group::new();

    default_table.place_widget(Widget {
        shape_id: table_id.clone(),
        at: Some((50.0, 50.0)),
        ..Default::default()
    });

    svg.add_default_table(default_table);

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/table/table1.svg");

    assert_eq!(svg_str, contents);
}
