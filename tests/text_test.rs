use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn text_basic_test() {
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

    let contents = include_str!("../showcase/text/text1.svg");

    assert_eq!(svg_str, contents);
}
