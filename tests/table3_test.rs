use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn table3_test() {
    let mut svg = Svg::new(400.0, 300.0);

    let mut table = Table::new();
    table.col_width = 100.0;
    table.row_height = 60.0;
    table.color = "green".to_string();
    table.cell_margin_top = 44.0;
    table.cell_margin_left = 40.0;

    table.set_table_cell_font_size(vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)], 40.0);
    table.set_table_cell_font_color(vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)], "red".to_string());

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

    let contents = include_str!("../showcase/gadget/table/table3.svg");

    assert_eq!(svg_str, contents);
}
