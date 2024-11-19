use crate::defines::shape::Shape;
use std::fmt::Display;

pub struct Rect<T, U, W, Y> {
    pub width: T,
    pub height: U,
    pub radius_x: Option<W>,
    pub radius_y: Option<Y>,
}

impl<T, U, W, Y> Rect<T, U, W, Y> {
    pub fn new(width: T, height: U) -> Self {
        Rect {
            width,
            height,
            radius_x: None,
            radius_y: None,
        }
    }
}

impl<T, U, W, Y> Shape for Rect<T, U, W, Y> where
    T: Display,
    U: Display,
    W: Display,
    Y: Display,
{
    fn format(&self, shape_id: String) -> String {
        format!("    <rect id=\"{}\" {} />\n", shape_id, {
            let mut shape_str = format!("width=\"{}\" height=\"{}\"", self.width, self.height);

            if self.radius_x.is_some() && self.radius_y.is_some() {
                shape_str.push_str(&format!(
                    " rx=\"{}\" ry=\"{}\"",
                    self.radius_x.as_ref().unwrap(),
                    self.radius_y.as_ref().unwrap(),
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
    fn check_new_rect1() {
        let rect = Rect {
            width: 30.0,
            height: 20.0,
            radius_x: Some(10),
            radius_y: Some(5),
        };

        assert_eq!(rect.width, 30.0);
        assert_eq!(rect.height, 20.0);
        assert_eq!(rect.radius_x.unwrap() as f64, 10.0);
        assert_eq!(rect.radius_y.unwrap() as f64, 5.0);
    }

    #[test]
    fn check_new_rect2() {
        let rect = Rect {
            width: 30.0,
            height: 20.0,
            radius_x: Some(10),
            radius_y: Some(5.0),
        };

        assert_eq!(rect.radius_x.unwrap() as f64, 10.0);
        assert_eq!(rect.radius_y.unwrap() as f64, 5.0);
    }

    #[test]
    fn check_new_rect3() {
        let rect = Rect {
            width: 30.0,
            height: 20.0,
            radius_x: Some(10.0),
            radius_y: Some(5),
        };

        assert_eq!(rect.radius_x.unwrap() as f64, 10.0);
        assert_eq!(rect.radius_y.unwrap() as f64, 5.0);
    }

    #[test]
    fn check_new_rect4() {
        let rect = Rect {
            width: 30.0,
            height: 20.0,
            radius_x: Some(10.0),
            radius_y: Some(5.0),
        };

        assert_eq!(rect.radius_x.unwrap() as f64, 10.0);
        assert_eq!(rect.radius_y.unwrap() as f64, 5.0);
    }
    
    #[test]
    fn check_new() {
        let rect = Rect::new(30, 20);
        assert_eq!(rect.width as f64, 30.0);
        assert_eq!(rect.height as f64, 20.0);
        assert!(rect.radius_x.is_none());
        assert!(rect.radius_y.is_none());

        let rect = Rect::new(30.0, 20);
        assert_eq!(rect.width as f64, 30.0);
        assert_eq!(rect.height as f64, 20.0);

        let rect = Rect::new(30, 20.0);
        assert_eq!(rect.width as f64, 30.0);
        assert_eq!(rect.height as f64, 20.0);

        let rect = Rect::new(30.0, 20.0);
        assert_eq!(rect.width as f64, 30.0);
        assert_eq!(rect.height as f64, 20.0);
    }
}
