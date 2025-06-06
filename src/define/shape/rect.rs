#![doc = include_str!("RECT.md")]

use crate::tools::precision::svg_round;

#[derive(Debug, Clone)]
pub struct Rect {
    pub width: f64,
    pub height: f64,
    pub radius_x: Option<f64>,
    pub radius_y: Option<f64>,
    pub precision: usize,
}

impl Rect {
    pub fn new(width: f64, height: f64) -> Self {
        Rect {
            width,
            height,
            radius_x: None,
            radius_y: None,
            precision: 0,
        }
    }

    pub fn format(&self, shape_id: String) -> String {
        format!("    <rect id=\"{}\" {} />\n", shape_id, {
            let mut shape_str = format!("width=\"{}\" height=\"{}\"", svg_round(self.width, self.precision), svg_round(self.height, self.precision));

            if self.radius_x.is_some() && self.radius_y.is_some() {
                shape_str.push_str(&format!(
                    " rx=\"{}\" ry=\"{}\"",
                    svg_round(self.radius_x.unwrap(), self.precision),
                    svg_round(self.radius_y.unwrap(), self.precision),
                ));
            }

            shape_str
        })
    }

    pub fn unique(&self) -> String {
        format!(
            "Rect/width/{}/height/{}/radius_x/{:?}/radius_y/{:?}",
            self.width, self.height, self.radius_x, self.radius_y
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_rect() {
        let rect = Rect {
            width: 30.0,
            height: 20f64,
            radius_x: Some(10.0),
            radius_y: Some(5f64),
            precision: 4,
        };

        assert_eq!(rect.width, 30.0);
        assert_eq!(rect.height, 20.0);
        assert_eq!(rect.radius_x.unwrap(), 10.0);
        assert_eq!(rect.radius_y.unwrap(), 5.0);
        assert_eq!(rect.precision, 4);
    }

    #[test]
    fn check_new() {
        let rect = Rect::new(30f64, 20f64);
        assert_eq!(rect.width, 30.0);
        assert_eq!(rect.height, 20.0);
        assert!(rect.radius_x.is_none());
        assert!(rect.radius_y.is_none());
        assert_eq!(rect.precision, 0);
    }

    #[test]
    fn check_format1() {
        let rect = Rect::new(30.0, 20.0);

        assert_eq!(
            Rect::format(&rect, "1".to_string()),
            "    <rect id=\"1\" width=\"30\" height=\"20\" />\n"
        );
    }

    #[test]
    fn check_format2() {
        let rect = Rect {
            width: 30.00001,
            height: 20.00001,
            radius_x: Some(10.00001),
            radius_y: Some(5.00001),
            precision: 4,
        };

        assert_eq!(
            Rect::format(&rect, "1".to_string()),
            "    <rect id=\"1\" width=\"30\" height=\"20\" rx=\"10\" ry=\"5\" />\n"
        );
    }
}
