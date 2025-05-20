#![doc = include_str!("POLYGON.md")]

use crate::tools::precision::svg_round;

#[derive(Debug, Clone)]
pub struct Polygon {
    pub points: Vec<(f64, f64)>,
}

impl Polygon {
    pub fn new(points: Vec<(f64, f64)>) -> Self {
        Polygon { points }
    }

    pub fn format(&self, shape_id: String) -> String {
        format!("    <polygon id=\"{}\" points=\"{}\" />\n", shape_id, {
            let mut point_pairs = vec![];

            for point in &self.points {
                point_pairs.push(format!("{},{}", svg_round(point.0), svg_round(point.1)));
            }

            point_pairs.join(" ")
        })
    }

    pub fn unique(&self) -> String {
        format!("Polygon/points/{:?}", self.points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_polygon() {
        let polygon = Polygon::new(vec![(1.0, 2.0), (3.0, 4.0)]);

        assert_eq!(polygon.points, vec![(1.0, 2.0), (3.0, 4.0)]);
    }

    #[test]
    fn check_format() {
        let polygon = Polygon::new(vec![(1.0, 2.0), (3.0, 4.0)]);

        assert_eq!(
            Polygon::format(&polygon, "1".to_string()),
            "    <polygon id=\"1\" points=\"1,2 3,4\" />\n"
        );
    }
}
