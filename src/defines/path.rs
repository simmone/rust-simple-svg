#[derive(Debug, Clone)]
pub struct Path {
    pub defs: Vec<String>,
}

#[derive(Clone)]
ArcDirection {
    LeftBig,
    LeftSmall,
    RightBig,
    RightSmall,
}

impl fmt::Display for ArcDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FillRule::LeftBig => write!(f, "1, 0"),
            FillRule::LeftSmall => write!(f, "0, 0"),
            FillRule::RightBig => write!(f, "1, 1"),
            FillRule::RightSmall => write!(f, "0, 1"),
        }
    }
}

impl Path {
    pub fn new() -> Self {
        Path {
            defs: vec![],
        }
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
        self.defs.push(format!("A{},{} 0 {} {},{}", radius.0, radius.1, section, point.0, point.1));
    }

    pub fn arc_rel(&mut self, point: (f64, f64), radius: (f64, f64), section: ArcDirection) {
        self.defs.push(format!("a{},{} 0 {} {},{}", radius.0, radius.1, section, point.0, point.1));
    }

    pub fn format(&self, shape_id: String) -> String {
        let mut fmt_str = String::new();

        fmt_str.push_str(&format!("    <path id=\"{}\">\n", shape_id));

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
        
        path.defs.push("path strs".to_string());

        assert_eq!(Path::format(&path, "1".to_string()), {
            let mut c_str = String::new();

            c_str.push_str("    <path id=\"1\">\n");
            c_str.push_str("          d=\"\n");
            c_str.push_str("             path strs\n");
            c_str.push_str("            \"/>\n");
                   
            c_str
        });
    }
}
