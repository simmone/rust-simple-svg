use std::fmt;

#[derive(Debug, Clone)]
pub struct Path {
    pub defs: Vec<String>,
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
        Path { defs: vec![] }
    }

    pub fn lineto_abs(&mut self, point: (f64, f64)) {
        self.defs.push(format!("L{},{}", point.0, point.1));
    }

    pub fn lineto_rel(&mut self, point: (f64, f64)) {
        self.defs.push(format!("l{},{}", point.0, point.1));
    }

    pub fn lineto_hor(&mut self, length: f64) {
        self.defs.push(format!("h{}", length));
    }

    pub fn lineto_ver(&mut self, length: f64) {
        self.defs.push(format!("v{}", length));
    }

    pub fn arc_abs(&mut self, point: (f64, f64), radius: (f64, f64), section: ArcDirection) {
        self.defs.push(format!(
            "A{},{} 0 {} {},{}",
            radius.0, radius.1, section, point.0, point.1
        ));
    }

    pub fn arc_rel(&mut self, point: (f64, f64), radius: (f64, f64), section: ArcDirection) {
        self.defs.push(format!(
            "a{},{} 0 {} {},{}",
            radius.0, radius.1, section, point.0, point.1
        ));
    }

    pub fn ccurve_abs(&mut self, point1: (f64, f64), point2: (f64, f64), point3: (f64, f64)) {
        self.defs.push(format!(
            "C{},{} {},{} {},{}",
            point1.0, point1.1, point2.0, point2.1, point3.0, point3.1
        ));
    }

    pub fn ccurve_rel(&mut self, point1: (f64, f64), point2: (f64, f64), point3: (f64, f64)) {
        self.defs.push(format!(
            "c{},{} {},{} {},{}",
            point1.0, point1.1, point2.0, point2.1, point3.0, point3.1
        ));
    }

    pub fn qcurve_abs(&mut self, point1: (f64, f64), point2: (f64, f64)) {
        self.defs.push(format!(
            "Q{},{} {},{}",
            point1.0, point1.1, point2.0, point2.1
        ));
    }

    pub fn qcurve_rel(&mut self, point1: (f64, f64), point2: (f64, f64)) {
        self.defs.push(format!(
            "q{},{} {},{}",
            point1.0, point1.1, point2.0, point2.1
        ));
    }

    pub fn moveto_abs(&mut self, point: (f64, f64)) {
        self.defs.push(format!("M{},{}", point.0, point.1));
    }

    pub fn moveto_rel(&mut self, point: (f64, f64)) {
        self.defs.push(format!("m{},{}", point.0, point.1));
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
