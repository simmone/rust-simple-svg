use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn table1_test() {
    let mut svg = Svg::new(250.0, 200.0);

    let mut table = Table::new();

    let table_group = table.to_group(&vec![["1", "2"], ["3", "4"]]);

    let table_id = svg.add_group(table_group);

    let mut default_group = Group::new();

    default_group.place_widget(Widget {
        shape_id: table_id.clone(),
        at: Some((50.0, 50.0)),
        ..Default::default()
    });

    svg.add_default_group(default_group);

    let svg_str = svg_out(svg);

    println!("{svg_str}");

    let contents = include_str!("../showcase/gadget/table/table1.svg");

    assert_eq!(svg_str, contents);
}
