#![doc = include_str!("ELLIPSE.md")]

use crate::tools::precision::svg_round;

#[derive(Debug, Clone)]
pub struct Ellipse {
    pub radius_x: f64,
    pub radius_y: f64,
    pub precision: usize,
}

impl Ellipse {
    pub fn new(radius_x: f64, radius_y: f64) -> Self {
        Ellipse { radius_x, radius_y, precision: 0 }
    }

    pub fn format(&self, shape_id: String) -> String {
        format!(
            "    <ellipse id=\"{}\" rx=\"{}\" ry=\"{}\" />\n",
            shape_id, svg_round(self.radius_x, self.precision), svg_round(self.radius_y, self.precision)
        )
    }

    pub fn unique(&self) -> String {
        format!(
            "Ellipse/radius_x/{}/radius_y/{}",
            self.radius_x, self.radius_y
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_ellipse() {
        let ellipse = Ellipse::new(30.0, 20.0);

        assert_eq!(ellipse.radius_x, 30.0);
        assert_eq!(ellipse.radius_y, 20.0);
    }

    #[test]
    fn check_format() {
        let ellipse = Ellipse::new(30.00001, 20.00001);

        assert_eq!(
            Ellipse::format(&ellipse, "1".to_string()),
            "    <ellipse id=\"1\" rx=\"30\" ry=\"20\" />\n"
        );
    }
}
