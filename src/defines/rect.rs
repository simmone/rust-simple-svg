use crate::defines::shape::Shape;

pub struct Rect<T, U> {
    pub width: T,
    pub height: U,
    pub radius_x: Option<i32>,
    pub radius_y: Option<i32>,
}

pub fn build_rect<T, U>(width: T, height: U) -> Rect<T, U> {
    Rect {
        width,
        height,
        radius_x: None,
        radius_y: None,
    }
}

impl<T: std::fmt::Display, U: std::fmt::Display> Shape for Rect<T, U> {
    fn format(&self, shape_id: String) -> String {
        format!("    <rect id=\"{}\" {} />\n", shape_id, {
            let mut shape_str = format!("width=\"{}\" height=\"{}\"", self.width, self.height);

            if self.radius_x.is_some() && self.radius_y.is_some() {
                shape_str.push_str(&format!(
                    " rx=\"{}\" ry=\"{}\"",
                    self.radius_x.unwrap(),
                    self.radius_y.unwrap()
                ));
            }

            shape_str
        })
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
            radius_x: Some(10),
            radius_y: Some(5),
        };

        assert_eq!(rect.width, 30.0);
        assert_eq!(rect.height, 20.0);
        assert_eq!(rect.radius_x.unwrap(), 10);
        assert_eq!(rect.radius_y.unwrap(), 5);
    }
    
    #[test]
    fn check_build_rect() {
        let rect = build_rect(30, 20);

        assert_eq!(rect.width as f64, 30.0);
        assert_eq!(rect.height as f64, 20.0);
        assert!(rect.radius_x.is_none());
        assert!(rect.radius_y.is_none());

        let rect = build_rect(30.0, 20);

        assert_eq!(rect.width as f64, 30.0);
        assert_eq!(rect.height as f64, 20.0);
        assert!(rect.radius_x.is_none());
        assert!(rect.radius_y.is_none());
    }
}
