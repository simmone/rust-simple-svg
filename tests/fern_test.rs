use pretty_assertions::assert_eq;
//use std::fs::File;
//use std::io::prelude::*;
use num::complex::Complex;
use std::f64::consts::PI;
use simple_svg::*;

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
