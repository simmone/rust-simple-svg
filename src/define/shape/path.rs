#![doc = include_str!("PATH.md")]

use crate::tools::precision::svg_round;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Path {
    pub defs: Vec<String>,
    pub precision: usize,
}

#[derive(Clone)]
pub enum ArcDirection {
    LeftBig,
    LeftSmall,
    RightBig,
    RightSmall,
}

impl fmt::Display for ArcDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ArcDirection::LeftBig => write!(f, "1,0"),
            ArcDirection::LeftSmall => write!(f, "0,0"),
            ArcDirection::RightBig => write!(f, "1,1"),
            ArcDirection::RightSmall => write!(f, "0,1"),
        }
    }
}

impl Path {
    pub fn new() -> Self {
        Path { defs: vec![], precision: 0 }
    }

    pub fn lineto_abs(&mut self, point: (f64, f64)) {
        self.defs.push(format!("L{},{}", svg_round(point.0, self.precision), svg_round(point.1, self.precision)));
    }

    pub fn lineto_rel(&mut self, point: (f64, f64)) {
        self.defs.push(format!("l{},{}", svg_round(point.0, self.precision), svg_round(point.1, self.precision)));
    }

    pub fn lineto_hor(&mut self, length: f64) {
        self.defs.push(format!("h{}", svg_round(length, self.precision)));
    }

    pub fn lineto_ver(&mut self, length: f64) {
        self.defs.push(format!("v{}", svg_round(length, self.precision)));
    }

    pub fn arc_abs(&mut self, point: (f64, f64), radius: (f64, f64), section: ArcDirection) {
        self.defs.push(format!(
            "A{},{} 0 {} {},{}",
            svg_round(radius.0, self.precision),
            svg_round(radius.1, self.precision),
            section,
            svg_round(point.0, self.precision),
            svg_round(point.1, self.precision)
        ));
    }

    pub fn arc_rel(&mut self, point: (f64, f64), radius: (f64, f64), section: ArcDirection) {
        self.defs.push(format!(
            "a{},{} 0 {} {},{}",
            svg_round(radius.0, self.precision),
            svg_round(radius.1, self.precision),
            section,
            svg_round(point.0, self.precision),
            svg_round(point.1, self.precision)
        ));
    }

    pub fn ccurve_abs(&mut self, point1: (f64, f64), point2: (f64, f64), point3: (f64, f64)) {
        self.defs.push(format!(
            "C{},{} {},{} {},{}",
            svg_round(point1.0, self.precision),
            svg_round(point1.1, self.precision),
            svg_round(point2.0, self.precision),
            svg_round(point2.1, self.precision),
            svg_round(point3.0, self.precision),
            svg_round(point3.1, self.precision)
        ));
    }

    pub fn ccurve_rel(&mut self, point1: (f64, f64), point2: (f64, f64), point3: (f64, f64)) {
        self.defs.push(format!(
            "c{},{} {},{} {},{}",
            svg_round(point1.0, self.precision),
            svg_round(point1.1, self.precision),
            svg_round(point2.0, self.precision),
            svg_round(point2.1, self.precision),
            svg_round(point3.0, self.precision),
            svg_round(point3.1, self.precision)
        ));
    }

    pub fn qcurve_abs(&mut self, point1: (f64, f64), point2: (f64, f64)) {
        self.defs.push(format!(
            "Q{},{} {},{}",
            svg_round(point1.0, self.precision),
            svg_round(point1.1, self.precision),
            svg_round(point2.0, self.precision),
            svg_round(point2.1, self.precision)
        ));
    }

    pub fn qcurve_rel(&mut self, point1: (f64, f64), point2: (f64, f64)) {
        self.defs.push(format!(
            "q{},{} {},{}",
            svg_round(point1.0, self.precision),
            svg_round(point1.1, self.precision),
            svg_round(point2.0, self.precision),
            svg_round(point2.1, self.precision)
        ));
    }

    pub fn moveto_abs(&mut self, point: (f64, f64)) {
        self.defs.push(format!("M{},{}",svg_round(point.0, self.precision), svg_round(point.1, self.precision)));
    }

    pub fn moveto_rel(&mut self, point: (f64, f64)) {
        self.defs.push(format!("m{},{}", svg_round(point.0, self.precision), svg_round(point.1, self.precision)));
    }

    pub fn raw(&mut self, raw_data: String) {
        self.defs.push(raw_data);
    }

    pub fn close(&mut self) {
        self.defs.push("z".to_string());
    }

    pub fn format(&self, shape_id: String) -> String {
        let mut fmt_str = String::new();

        fmt_str.push_str(&format!("    <path id=\"{}\"\n", shape_id));

        fmt_str.push_str(&format!("          d=\"\n"));

        for def in self.defs.clone() {
            fmt_str.push_str(&format!("             {}\n", def));
        }

        fmt_str.push_str(&format!("            \"/>\n"));

        fmt_str
    }

    pub fn unique(&self) -> String {
        format!("Path/defs/{:?}", self.defs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_path() {
        let path = Path::new();

        assert_eq!(path.defs.len(), 0);
    }

    #[test]
    fn check_format() {
        let mut path = Path::new();

        path.defs.push("def1".to_string());
        path.defs.push("def2".to_string());

        assert_eq!(Path::format(&path, "1".to_string()), {
            let mut c_str = String::new();

            c_str.push_str("    <path id=\"1\"\n");
            c_str.push_str("          d=\"\n");
            c_str.push_str("             def1\n");
            c_str.push_str("             def2\n");
            c_str.push_str("            \"/>\n");

            c_str
        });
    }
}
