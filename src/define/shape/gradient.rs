#![doc = include_str!("GRADIENT.md")]

use crate::tools::precision::svg_round;
use std::fmt;

#[derive(Clone, Debug)]
pub enum GradientUnits {
    UserSpaceOnUse,
    ObjectBoundingBox,
}

impl fmt::Display for GradientUnits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GradientUnits::UserSpaceOnUse => write!(f, "userSpaceOnUse"),
            GradientUnits::ObjectBoundingBox => write!(f, "objectBoundingBox"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SpreadMethod {
    Repeat,
    Reflect,
}

impl fmt::Display for SpreadMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpreadMethod::Repeat => write!(f, "repeat"),
            SpreadMethod::Reflect => write!(f, "reflect"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinearGradient {
    pub stops: Vec<(f64, String, f64)>,
    pub x1: Option<f64>,
    pub y1: Option<f64>,
    pub x2: Option<f64>,
    pub y2: Option<f64>,
    pub gradient_units: Option<GradientUnits>,
    pub spread_method: Option<SpreadMethod>,
    pub precision: usize,
}

impl LinearGradient {
    pub fn new(stops: Vec<(f64, String, f64)>) -> Self {
        LinearGradient {
            stops,
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            gradient_units: None,
            spread_method: None,
            precision: 0,
        }
    }

    pub fn format(&self, shape_id: String) -> String {
        let mut fmt_str = String::new();

        fmt_str.push_str(&format!("    <linearGradient id=\"{}\" {}>\n", shape_id, {
            let mut option_items = vec![];

            if self.x1.is_some() {
                option_items.push(format!("x1=\"{}\"", svg_round(*self.x1.as_ref().unwrap(), self.precision)));
            }

            if self.y1.is_some() {
                option_items.push(format!("y1=\"{}\"", svg_round(*self.y1.as_ref().unwrap(), self.precision)));
            }

            if self.x2.is_some() {
                option_items.push(format!("x2=\"{}\"", svg_round(*self.x2.as_ref().unwrap(), self.precision)));
            }

            if self.y2.is_some() {
                option_items.push(format!("y2=\"{}\"", svg_round(*self.y2.as_ref().unwrap(), self.precision)));
            }

            if self.gradient_units.is_some() {
                option_items.push(format!(
                    "gradientUnits=\"{}\"",
                    self.gradient_units.as_ref().unwrap()
                ));
            }

            if self.spread_method.is_some() {
                option_items.push(format!(
                    "spreadMethod=\"{}\"",
                    self.spread_method.as_ref().unwrap()
                ));
            }

            option_items.join(" ")
        }));

        for stop in self.stops.clone() {
            fmt_str.push_str(&format!(
                "      <stop offset=\"{}%\" stop-color=\"{}\" ",
                svg_round(stop.0, self.precision), stop.1
            ));

            if svg_round(stop.2, self.precision) != "1" {
                fmt_str.push_str(&format!("stop-opacity=\"{}\" ", svg_round(stop.2, self.precision)));
            }

            fmt_str.push_str("/>\n");
        }

        fmt_str.push_str(&format!("    </linearGradient>\n"));

        fmt_str
    }

    pub fn unique(&self) -> String {
        format!(
            "LinearGradient/stops/{:?}/x1/{:?}/y1/{:?}/x2/{:?}/y2/{:?}/gradient_units/{:?}/spread_method/{:?}",
            self.stops,
            self.x1,
            self.y1,
            self.x2,
            self.y2,
            self.gradient_units,
            self.spread_method)
    }
}

#[derive(Debug, Clone)]
pub struct RadialGradient {
    pub stops: Vec<(f64, String, f64)>,
    pub cx: Option<f64>,
    pub cy: Option<f64>,
    pub fx: Option<f64>,
    pub fy: Option<f64>,
    pub r: Option<f64>,
    pub gradient_units: Option<GradientUnits>,
    pub spread_method: Option<SpreadMethod>,
    pub precision: usize,
}

impl RadialGradient {
    pub fn new(stops: Vec<(f64, String, f64)>) -> Self {
        RadialGradient {
            stops,
            cx: None,
            cy: None,
            fx: None,
            fy: None,
            r: None,
            gradient_units: None,
            spread_method: None,
            precision: 0,
        }
    }

    pub fn format(&self, shape_id: String) -> String {
        let mut fmt_str = String::new();

        fmt_str.push_str(&format!("    <radialGradient id=\"{}\" {}>\n", shape_id, {
            let mut option_items = vec![];

            if self.cx.is_some() {
                option_items.push(format!("cx=\"{}\"", svg_round(*self.cx.as_ref().unwrap(), self.precision)));
            }

            if self.cy.is_some() {
                option_items.push(format!("cy=\"{}\"", svg_round(*self.cy.as_ref().unwrap(), self.precision)));
            }

            if self.fx.is_some() {
                option_items.push(format!("fx=\"{}\"", svg_round(*self.fx.as_ref().unwrap(), self.precision)));
            }

            if self.fy.is_some() {
                option_items.push(format!("fy=\"{}\"", svg_round(*self.fy.as_ref().unwrap(), self.precision)));
            }

            if self.r.is_some() {
                option_items.push(format!("r=\"{}\"", svg_round(*self.r.as_ref().unwrap(), self.precision)));
            }

            if self.gradient_units.is_some() {
                option_items.push(format!(
                    "gradientUnits=\"{}\"",
                    self.gradient_units.as_ref().unwrap()
                ));
            }

            if self.spread_method.is_some() {
                option_items.push(format!(
                    "spreadMethod=\"{}\"",
                    self.spread_method.as_ref().unwrap()
                ));
            }

            option_items.join(" ")
        }));

        for stop in self.stops.clone() {
            fmt_str.push_str(&format!(
                "      <stop offset=\"{}%\" stop-color=\"{}\" ",
                svg_round(stop.0, self.precision), stop.1
            ));

            if svg_round(stop.2, self.precision) != "1" {
                fmt_str.push_str(&format!("stop-opacity=\"{}\" ", svg_round(stop.2, self.precision)));
            }

            fmt_str.push_str("/>\n");
        }

        fmt_str.push_str(&format!("    </radialGradient>\n"));

        fmt_str
    }

    pub fn unique(&self) -> String {
        format!(
            "RadialGradient/stops/{:?}/cx/{:?}/cy/{:?}/fx/{:?}/fy/{:?}/r/{:?}/gradient_units/{:?}/spread_method/{:?}",
            self.stops,
            self.cx,
            self.cy,
            self.fx,
            self.fy,
            self.r,
            self.gradient_units,
            self.spread_method)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_linear_gradient() {
        let gradient = LinearGradient::new(vec![
            (0.0, "#BBC42A".to_string(), 1.0),
            (100.0, "#ED6E46".to_string(), 1.0),
        ]);

        assert_eq!(gradient.stops.len(), 2);
    }

    #[test]
    fn check_linear_format() {
        let mut gradient = LinearGradient::new(vec![
            (0.00001, "#BBC42A".to_string(), 1.00001),
            (100.00001, "#ED6E46".to_string(), 1.00001),
        ]);

        gradient.x1 = Some(0.00001);
        gradient.y1 = Some(1.00001);
        gradient.x2 = Some(2.00001);
        gradient.y2 = Some(3.00001);
        gradient.gradient_units = Some(GradientUnits::UserSpaceOnUse);
        gradient.spread_method = Some(SpreadMethod::Repeat);

        let contents = include_str!("../../../showcase/gradient/linear_gradient_define.svg");

        assert_eq!(
            LinearGradient::format(&gradient, "s1".to_string()),
            contents
        );
    }

    #[test]
    fn check_new_radial_gradient() {
        let gradient = RadialGradient::new(vec![
            (0.0, "#BBC42A".to_string(), 1.0),
            (100.0, "#ED6E46".to_string(), 1.0),
        ]);

        assert_eq!(gradient.stops.len(), 2);
    }

    #[test]
    fn check_radial_format() {
        let mut gradient = RadialGradient::new(vec![
            (0.00001, "#BBC42A".to_string(), 1.00001),
            (100.00001, "#ED6E46".to_string(), 1.00001),
        ]);

        gradient.cx = Some(0.00001);
        gradient.cy = Some(1.00001);
        gradient.fx = Some(2.00001);
        gradient.fy = Some(3.00001);
        gradient.r = Some(4.0);
        gradient.gradient_units = Some(GradientUnits::UserSpaceOnUse);
        gradient.spread_method = Some(SpreadMethod::Repeat);

        let contents = include_str!("../../../showcase/gradient/radial_gradient_define.svg");

        assert_eq!(
            RadialGradient::format(&gradient, "s1".to_string()),
            contents
        );
    }
}
