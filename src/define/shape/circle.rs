#![doc = include_str!("CIRCLE.md")]

use crate::tools::precision::svg_round;

#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f64,
    pub precision: usize,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle {
            radius,
            precision: 0,
        }
    }

    pub fn format(&self, shape_id: String) -> String {
        format!(
            "    <circle id=\"{}\" r=\"{}\" />\n",
            shape_id,
            svg_round(self.radius, self.precision)
        )
    }

    pub fn unique(&self) -> String {
        format!("Circle/radius/{}", self.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_circle() {
        let circle = Circle::new(30.0);

        assert_eq!(circle.radius, 30.0);
    }

    #[test]
    fn check_format() {
        let circle = Circle::new(30.00001);

        assert_eq!(
            Circle::format(&circle, "1".to_string()),
            "    <circle id=\"1\" r=\"30\" />\n"
        );
    }
}
