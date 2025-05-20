#![doc = include_str!("LINE.md")]

use crate::tools::precision::svg_round;

#[derive(Debug, Clone)]
pub struct Line {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
}

impl Line {
    pub fn new(start: (f64, f64), end: (f64, f64)) -> Self {
        Line {
            start_x: start.0,
            start_y: start.1,
            end_x: end.0,
            end_y: end.1,
        }
    }

    pub fn format(&self, shape_id: String) -> String {
        format!(
            "    <line id=\"{}\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" />\n",
            shape_id, svg_round(self.start_x), svg_round(self.start_y), svg_round(self.end_x), svg_round(self.end_y)
        )
    }

    pub fn unique(&self) -> String {
        format!(
            "Line/start_x/{}/start_y/{}/end_x/{}/end_y/{}",
            self.start_x, self.start_y, self.end_x, self.end_y
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_line() {
        let line = Line::new((10.0, 20.0), (30.0, 40.0));

        assert_eq!(line.start_x, 10.0);
        assert_eq!(line.start_y, 20.0);
        assert_eq!(line.end_x, 30.0);
        assert_eq!(line.end_y, 40.0);
    }

    #[test]
    fn check_format() {
        let line = Line::new((10.00001, 20.00001), (30.00001, 40.00001));

        assert_eq!(
            Line::format(&line, "1".to_string()),
            "    <line id=\"1\" x1=\"10\" y1=\"20\" x2=\"30\" y2=\"40\" />\n"
        );
    }
}
