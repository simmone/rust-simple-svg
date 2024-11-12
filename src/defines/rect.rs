use crate::defines::shape::Shape;

pub struct Rect {
    pub width: f64,
    pub height: f64,
    pub radius_x: u32,
    pub radius_y: u32,
}

impl Shape for Rect {
    fn format(&self, shape_id: String) -> String {
        format!("    <rect id=\"{}\" {} />\n",
        shape_id,
        || {
            let mut shape_str = format!("width=\"{}\" height=\"{}\"",
                                        self.width, self.height);

            when (self.radius_x >= 0) && (self.radius_y >= 0) {
                shape_str.push_str(&format!(" rx=\"{}\" ry=\"{}\"",
                                            self.radius_x, self.radius_y));
            }

            shape_str
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_rect() {
        let rect = Rect {
            width: 30.0,
            height: 20.0,
            radius_x: 10,
            radius_y: 5,
        };
        assert_eq!(rect.width, 30.0);
        assert_eq!(rect.height, 20.0);
        assert_eq!(rect.radius_x, 10);
        assert_eq!(rect.radius_y, 6);
    }
}
