use pretty_assertions::assert_eq;
//use std::fs::File;
//use std::io::prelude::*;
use num::complex::Complex;
use std::f64::consts::PI;
use simple_svg::*;

#[test]
fn polar_test() {
    let complex_number = Complex::new(0.0, 1.0);
    
    let magnitude = 10.0;
    
    let angle = PI * 0.5;
    
    let polar = (angle.cos() * magnitude) + (angle.sin() * magnitude * complex_number);
    
    assert_eq!(polar, Complex::new(6.123233995736766e-16, 10.0));
}
