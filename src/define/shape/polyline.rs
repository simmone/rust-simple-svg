#![doc = include_str!("POLYLINE.md")]

use crate::tools::precision::svg_round;

#[derive(Debug, Clone)]
pub struct Polyline {
    pub points: Vec<(f64, f64)>,
    pub precision: usize,
}

impl Polyline {
    pub fn new(points: Vec<(f64, f64)>) -> Self {
        Polyline { points, precision: 0 }
    }

    pub fn format(&self, shape_id: String) -> String {
        format!("    <polyline id=\"{}\" points=\"{}\" />\n", shape_id, {
            let mut point_pairs = vec![];

            for point in &self.points {
                point_pairs.push(format!("{},{}", svg_round(point.0, self.precision), svg_round(point.1, self.precision)));
            }

            point_pairs.join(" ")
        })
    }

    pub fn unique(&self) -> String {
        format!("Polyline/points/{:?}", self.points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_polyline() {
        let polyline = Polyline::new(vec![(1.0, 2.0), (3.0, 4.0)]);

        assert_eq!(polyline.points, vec![(1.0, 2.0), (3.0, 4.0)]);
    }

    #[test]
    fn check_format() {
        let polyline = Polyline::new(vec![(1.0, 2.0), (3.0, 4.0)]);

        assert_eq!(
            Polyline::format(&polyline, "1".to_string()),
            "    <polyline id=\"1\" points=\"1,2 3,4\" />\n"
        );
    }
}
