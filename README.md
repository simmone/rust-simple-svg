# Svg Image Writer for Rust

The project provides abilities to generate basic svg shape/group image.

thanks to Joni's tutorial: [SVG Pocket Guide](http://svgpocketguide.com/)

## Getting Started

This library's target is to generate combined shapes progmatically.

## Usage

Basic steps:
1. create canvas, specify width and height: Svg::new(with, height);
2. create shape and add to canvas: svg.add_shape(Shape::Rect(Rect::new(...;
3. create a style for this shape/group: Sstyle::new();
4. create a group: Group::new();
5. place the shape/group id in the group in somewhere, some style: group.place_widget(Widget(shape_id:..., at:..., style:..., ..Default::default()));
6. set group as default group;
7. use svg_out to generate svg content;

Basic component usage please check the rustdoc.

## License

MIT See [LICENSE](../../../LICENSE) file.
