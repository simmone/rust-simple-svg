use pretty_assertions::assert_eq;
//use std::fs::File;
//use std::io::prelude::*;
use std::collections::HashMap;
use num::complex::Complex;
use std::f64::consts::PI;
use simple_svg::*;

const CANVAS_WIDTH: f64 = 600.0;
const CANVAS_HEIGHT: f64 = 600.0;
const START_POINT: (f64, f64) = (300.0, 50.0);
const START_LENGTH: f64 = 120.0;
const START_DEG: f64 = 100.0; // 100°
const START_WIDTH: f64 = 3.0;
const STEP_WIDTH: f64 = 0.86;
const COLOR: &str = "#5A5";
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
    assert_eq!(make_polar(10.0, PI * 0.5), Complex::new(6.123233995736766e-16, 10.0));

    assert_eq!(make_polar(10.0, PI * 0.25), Complex::new(7.0710678118654755, 7.071067811865475));
}

fn get_end_point(start_point: (f64, f64), length: f64, deg: f64, precision: f64) -> (f64, f64) {
    let end = make_polar(length, PI * 2.0 * (deg / 360.0));
    
    (
        ((start_point.0 + end.re) * (10.0f64.powf(precision))).round() / (10.0f64.powf(precision)),
        ((start_point.1 + end.im) * (10.0f64.powf(precision))).round() / (10.0f64.powf(precision))
    )
}

fn recursive_points(loop_start_point: (f64, f64), length: f64, deg: f64, width: f64, style_map: HashMap<K, V>) {
    if (CENTRAL_REDUCTION * length) >= MIN_LENGTH {
        let loop_end_point = get_end_point(loop_start_point, length, deg, PRECISION);
        
        let truncted_length = format!("{:.2}", length);
        
        let mut width_points = style_map.get(&truncted_length).unwrap_or(vec![]);

        width_points.push(
            (
                (loop_start_point.0, CANVAS_HEIGHT - loop_start_point.1),
                (loop_end_point.0, CANVAS_HEIGHT - loop_end_point.1)
            ));

        style_map.insert(truncted_length, width_points);
        
        // central branch
        recursive_points(
            loop_end_point,
            length * CENTRAL_REDUCTION,
            deg - BEND,
            width * STEP_WIDTH);

        // left branch
        recursive_points(
            loop_end_point,
            length * LATERAL_REDUCTION,
            deg + LATERAL_DEG - BEND,
            width * STEP_WIDTH);

        // right branch
        recursive_points(
            loop_end_point,
            length * LATERAL_REDUCTION,
            deg - LATERAL_DEG - BEND,
            width * STEP_WIDTH);
    }
}

#[test]
fn recursive_points_test() {
    let mut style_map = HashMap::new();
    
    recursive_points(START_POINT, START_LENGTH, START_DEG, START_WIDTH, style_map);
    
    assert_eq!(style_map.leng(), 19);
    assert_eq!(style_map.get(3.0).unwarp().len(), 1);
    assert_eq!(style_map.get(0.77).unwrap().len(), 835);
    assert_eq!(style_map.get(1.04).unwrap().len(), 939);
    assert_eq!(style_map.get(1.21).unwrap().len(), 473);
    assert_eq!(style_map.get(0.49).unwrap().len(), 289);
    assert_eq!(style_map.get(1.64).unwrap().len(), 81);
    assert_eq!(style_map.get(0.27).unwrap().len(), 1);
    assert_eq!(style_map.get(0.57).unwrap().len(), 243);
    assert_eq!(style_map.get(0.31).unwrap().len(), 31);
    assert_eq!(style_map.get(2.22).unwrap().len(), 9);
    assert_eq!(style_map.get(0.66).unwrap().len(), 1161);
    assert_eq!(style_map.get(0.42).unwrap().len(), 27);
    assert_eq!(style_map.get(1.41).unwrap().len(), 211);
    assert_eq!(style_map.get(1.91).unwrap().len(), 27);
    assert_eq!(style_map.get(0.23).unwrap().len(), 1);
    assert_eq!(style_map.get(0.9).unwrap().len(), 577);
    assert_eq!(style_map.get(2.58).unwrap().len(), 3);
    assert_eq!(style_map.get(0.36).unwrap().len(), 29);
    assert_eq!(style_map.get(0.2).unwrap().len(), 1);
}

fn fern_test() {
    let mut svg = Svg::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    
//    svg.add_default_group(group);

    let svg_str = svg_out(svg);

//    let mut file = File::create("resursive.svg")?;

//    file.write(svg_str.as_bytes())?;

//    Ok(())

    let contents = include_str!("../showcase/example/fern.svg");

    assert_eq!(svg_str, contents);
}
