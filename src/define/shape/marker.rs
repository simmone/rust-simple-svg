#![doc = include_str!("MARKER.md")]

use crate::tools::precision::svg_round;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Marker {
    pub shape: MarkerType,
    pub size: f64,
    pub x: f64,
    pub path: String,
    pub precision: usize,
}

#[derive(Debug, Clone)]
pub enum MarkerType {
    Triangle1,
    Triangle2,
    Circle,
    Indent1,
    Indent2,
    Diamond1,
    Diamond2,
    Curve1,
    Curve2,
}

impl fmt::Display for MarkerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MarkerType::Triangle1 => write!(f, "triangle1"),
            MarkerType::Triangle2 => write!(f, "triangle2"),
            MarkerType::Circle => write!(f, "circle"),
            MarkerType::Indent1 => write!(f, "indent1"),
            MarkerType::Indent2 => write!(f, "indent2"),
            MarkerType::Diamond1 => write!(f, "diamond1"),
            MarkerType::Diamond2 => write!(f, "diamond2"),
            MarkerType::Curve1 => write!(f, "curve1"),
            MarkerType::Curve2 => write!(f, "curve2"),
        }
    }
}

impl Marker {
    pub fn new(shape: MarkerType) -> Self {
        match shape {
            MarkerType::Triangle1 => Marker {
                shape,
                size: 6.0,
                x: 1.0,
                path: "<path d=\"M0,0 L10,5 L0,10 z\"".to_string(),
                precision: 0,
            },
            MarkerType::Triangle2 => Marker {
                shape,
                size: 6.0,
                x: 1.0,
                path: "<path d=\"M0,0 L15,5 L0,10 z\"".to_string(),
                precision: 0,
            },
            MarkerType::Circle => Marker {
                shape,
                size: 6.0,
                x: 5.0,
                path: "<circle r=\"5\" cx=\"5\" cy=\"5\"".to_string(),
                precision: 0,
            },
            MarkerType::Indent1 => Marker {
                shape,
                size: 6.0,
                x: 4.0,
                path: "<path d=\"M0,0 L10,5 L0,10 L5,5 z\"".to_string(),
                precision: 0,
            },
            MarkerType::Indent2 => Marker {
                shape,
                size: 6.0,
                x: 4.0,
                path: "<path d=\"M0,0 L15,5 L0,10 L5,5 z\"".to_string(),
                precision: 0,
            },
            MarkerType::Diamond1 => Marker {
                shape,
                size: 6.0,
                x: 1.0,
                path: "<path d=\"M3,0 L10,5 L3,10 L0,5 z\"".to_string(),
                precision: 0,
            },
            MarkerType::Diamond2 => Marker {
                shape,
                size: 6.0,
                x: 1.0,
                path: "<path d=\"M3,0 L15,5 L3,10 L0,5 z\"".to_string(),
                precision: 0,
            },
            MarkerType::Curve1 => Marker {
                shape,
                size: 6.0,
                x: 2.0,
                path: "<path d=\"M0,0 L10,5 L0,10 C0,10 5,5 0,0 z\"".to_string(),
                precision: 0,
            },
            MarkerType::Curve2 => Marker {
                shape,
                size: 6.0,
                x: 2.0,
                path: "<path d=\"M0,0 L15,5 L0,10 C0,10 5,5 0,0 z\"".to_string(),
                precision: 0,
            },
        }
    }

    pub fn format(&self, shape_id: String) -> String {
        let mut fmt_str = String::new();

        fmt_str.push_str(&format!("    <marker id=\"{}\" markerWidth=\"{}\" markerHeight=\"{}\" orient=\"auto-start-reverse\" viewBox=\"0 0 15 15\" refX=\"{}\" refY=\"5\" markerUnits=\"strokeWidth\">\n",
                                  shape_id,
                                  svg_round(self.size, self.precision),
                                  svg_round(self.size, self.precision),
                                  svg_round(self.x, self.precision)));
        fmt_str.push_str(&format!("      {} fill=\"context-stroke\" />\n", self.path));
        fmt_str.push_str(&format!("    </marker>\n"));

        fmt_str
    }

    pub fn unique(&self) -> String {
        format!(
            "Marker/shape/{}/size/{}/x/{}/path/{}",
            self.shape, self.size, self.x, self.path
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_marker() {
        let mut marker = Marker::new(MarkerType::Curve1);

        marker.size = 10.0;

        assert_eq!(marker.size, 10.0);
    }
    
    #[test]
    fn marker_format() {
        let marker = Marker {
                shape: MarkerType::Triangle1,
                size: 6.00001,
                x: 1.00001,
                path: "path".to_string(),
                precision: 0,
            };
        
        assert_eq!(
            marker.format("s1".to_string()),
            {
                let mut fmt_str = String::new();
                fmt_str.push_str(&format!("    <marker id=\"s1\" markerWidth=\"6\" markerHeight=\"6\" orient=\"auto-start-reverse\" viewBox=\"0 0 15 15\" refX=\"1\" refY=\"5\" markerUnits=\"strokeWidth\">\n"));

                fmt_str.push_str(&format!("      path fill=\"context-stroke\" />\n"));
                fmt_str.push_str(&format!("    </marker>\n"));
                fmt_str
            });
    }
}
