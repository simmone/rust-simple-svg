use pretty_assertions::assert_eq;
use simple_svg::*;

#[test]
fn rect_svg_out_test() {
    let svg = Svg::new(30.0, 20.0);
    
    let rect = Shape::Rect(Rect::new(100.0, 100.0));

    let rect_id = svg.add_shape(rect);
    
    let rect_sstyle = Sstyle::new();
    rect_sstyle.fill = Some("#BBC42A".to_string());

    svg.place_widget(rect_id, 

    let svg_str = svg_out(svg);

    let contents = include_str!("../showcase/shapes/rect/rect.svg");

    assert_eq!(svg_str, contents);
}

            (lambda ()
              (let ([rec_id (svg-def-shape (new-rect 100 100))]
                    [_sstyle (sstyle-new)])
                (set-SSTYLE-fill! _sstyle "#BBC42A")
                (svg-place-widget rec_id #:style _sstyle))))])

