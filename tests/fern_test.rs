use pretty_assertions::assert_eq;
//use std::fs::File;
//use std::io::prelude::*;
use num::complex::Complex;
use simple_svg::*;
use std::collections::HashMap;
use std::f64::consts::PI;

const CANVAS_WIDTH: f64 = 600.0;
const CANVAS_HEIGHT: f64 = 600.0;
const START_POINT: (f64, f64) = (300.0, 50.0);
const START_LENGTH: f64 = 120.0;
const START_DEG: f64 = 100.0; // 100°
const START_WIDTH: f64 = 3.0;
const STEP_WIDTH: f64 = 0.86;
const COLOR: &str = "#5a5";
const MIN_LENGTH: f64 = 0.5;
const CENTRAL_REDUCTION: f64 = 0.75;
const LATERAL_REDUCTION: f64 = 0.35;
const LATERAL_DEG: f64 = 80.0; // 80°
const BEND: f64 = 5.0; // 5°
const PRECISION: f64 = 0.0;

fn make_polar(magnitude: f64, angle: f64) -> Complex<f64> {
    let complex_number = Complex::new(0.0, 1.0);

    (angle.cos() * magnitude) + (angle.sin() * magnitude * complex_number)
}

#[test]
fn make_polar_test() {
    assert_eq!(
        make_polar(10.0, PI * 0.5),
        Complex::new(6.123233995736766e-16, 10.0)
    );

    assert_eq!(
        make_polar(10.0, PI * 0.25),
        Complex::new(7.0710678118654755, 7.071067811865475)
    );
}

fn get_end_point(start_point: (f64, f64), length: f64, deg: f64, precision: f64) -> (f64, f64) {
    let end = make_polar(length, PI * 2.0 * (deg / 360.0));

    (
        ((start_point.0 + end.re) * (10.0f64.powf(precision))).round() / (10.0f64.powf(precision)),
        ((start_point.1 + end.im) * (10.0f64.powf(precision))).round() / (10.0f64.powf(precision)),
    )
}

fn recursive_points(
    loop_start_point: (f64, f64),
    length: f64,
    deg: f64,
    width: f64,
    lines: &mut Vec<(String, (f64, f64), (f64, f64))>,
) {
    if (CENTRAL_REDUCTION * length) >= MIN_LENGTH {
        let loop_end_point = get_end_point(loop_start_point, length, deg, PRECISION);

        let truncted_width = format!("{:.2}", width);

        lines.push((
            truncted_width,
            (loop_start_point.0, CANVAS_HEIGHT - loop_start_point.1),
            (loop_end_point.0, CANVAS_HEIGHT - loop_end_point.1),
        ));

        // central branch
        recursive_points(
            loop_end_point,
            length * CENTRAL_REDUCTION,
            deg - BEND,
            width * STEP_WIDTH,
            lines,
        );

        // left branch
        recursive_points(
            loop_end_point,
            length * LATERAL_REDUCTION,
            deg + LATERAL_DEG - BEND,
            width * STEP_WIDTH,
            lines,
        );

        // right branch
        recursive_points(
            loop_end_point,
            length * LATERAL_REDUCTION,
            deg - LATERAL_DEG - BEND,
            width * STEP_WIDTH,
            lines,
        );
    }
}

#[test]
fn recursive_points_test() {
    let mut lines: Vec<(String, (f64, f64), (f64, f64))> = vec![];

    recursive_points(
        START_POINT,
        START_LENGTH,
        START_DEG,
        START_WIDTH,
        &mut lines,
    );

    assert_eq!(lines.len(), 4939);
}

#[test]
//fn fern_test() -> std::io::Result<()> {
fn fern_test() {
    let mut svg = Svg::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    svg.precision = 4;

    let mut default_group = Group::new();

    let mut lines: Vec<(String, (f64, f64), (f64, f64))> = vec![];

    recursive_points(
        START_POINT,
        START_LENGTH,
        START_DEG,
        START_WIDTH,
        &mut lines,
    );

    let mut width_group_map = HashMap::new();

    for line in lines {
        if !width_group_map.contains_key(&line.0) {
            let mut new_vec = vec![];

            new_vec.push((line.1, line.2));

            width_group_map.insert(line.0, new_vec);
        } else {
            let exists_vec = width_group_map.get_mut(&line.0).unwrap();

            exists_vec.push((line.1, line.2));
        }
    }

    assert_eq!(width_group_map.len(), 19);

    let mut widths: Vec<String> = width_group_map.clone().into_keys().collect();
    widths.sort_by(|a, b| {
        let a_num = a.parse::<f64>().unwrap();
        let b_num = b.parse::<f64>().unwrap();

        let a_int = (a_num * 100.0).round() as usize;
        let b_int = (b_num * 100.0).round() as usize;

        b_int.cmp(&a_int)
    });

    for width in widths {
        let mut width_sstyle = Sstyle::new();
        width_sstyle.stroke = Some(COLOR.to_string());
        width_sstyle.stroke_width = Some(width.parse::<f64>().unwrap());

        let mut width_group = Group::new();

        for point_pair in width_group_map.get(&width).unwrap() {
            let line_id = svg.add_shape(Shape::Line(Line::new(point_pair.0, point_pair.1)));

            width_group.place_widget(Widget {
                shape_id: line_id,
                ..Default::default()
            });
        }

        let width_group_id = svg.add_group(width_group);

        default_group.place_widget(Widget {
            shape_id: width_group_id.clone(),
            style: Some(width_sstyle.clone()),
            ..Default::default()
        });
    }

    svg.add_default_group(default_group);

    let svg_str = svg_out(svg);

    //    let mut file = File::create("fern.svg")?;

    //    file.write(svg_str.as_bytes())?;

    //    Ok(())

    let contents = include_str!("../showcase/example/fern.svg");

    assert_eq!(svg_str, contents);
}
