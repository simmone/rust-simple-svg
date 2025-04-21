use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn table2_test() {
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

    let contents = include_str!("../showcase/gadget/table/table2.svg");

    assert_eq!(svg_str, contents);
}
