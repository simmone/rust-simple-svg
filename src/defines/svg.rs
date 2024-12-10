use crate::defines::shape::Shape;
use crate::defines::rect::Rect;
use std::collections::HashMap;

pub struct Svg<'a> {
    pub width: f64,
    pub height: f64,
    pub widget_id_count: u32,
    pub shape_define_map: HashMap<&'a String, &'static dyn Shape>,
}

impl<'a> Svg<'a> {
    pub fn new(width: f64, height: f64) -> Svg<'a> {
        Svg {
            width,
            height,
            widget_id_count: 0,
            shape_define_map: HashMap::new(),
        }
    }
    
    pub fn def_shape<T: Shape>(&mut self, shape: &T) -> String {
        self.widget_id_count += 1;
        let shape_id = format!("s{}", self.widget_id_count);
        self.shape_define_map.insert(&shape_id, shape);
        shape_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_svg() {
        let svg = SVG::new(640.0, 480.0);
        assert_eq!(svg.width, 640.0);
        assert_eq!(svg.height, 480.0);
        assert_eq!(svg.widget_id_count, 0);
        assert_eq!(svg.shape_define_map.len(), 0);
    }
    
    #[test]
    fn check_def_shape() {
        let svg = SVG::new(640.0, 480.0);

        let rect1 = Rect::new(30.0, 20.0);
        let rect_id = svg.def_shape(&rect1);
        assert_eq!(svg.widget_id_count, 1);
        assert_eq!(svg.shape_define_map.get(&"s1".to_string()), &rect1);

        let rect2 = Rect::new(30.0, 20.0);
        let rect_id = svg.def_shape(&rect2);
        assert_eq!(svg.widget_id_count, 1);
        assert_eq!(svg.shape_define_map.get(&"s2".to_string()), &rect2);
    }
}
