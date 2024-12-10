use crate::defines::shape::Shape;
use crate::defines::rect::Rect;
use std::collections::HashMap;

pub struct Svg<'a> {
    pub width: f64,
    pub height: f64,
    pub widget_id_count: u32,
    pub shape_define_map: HashMap<String, &'a Rect>,
}

pub fn build_svg(width: f64, height: f64) -> Svg<'static> {
    Svg {
        width,
        height,
        widget_id_count: 0,
        shape_define_map: HashMap::new(),
    }
}

impl<'a> Svg<'a> {
    pub fn def_shape(&mut self, shape: &'a Rect) -> String {
        self.widget_id_count += 1;
        let shape_id = format!("s{}", self.widget_id_count);
        self.shape_define_map.insert(shape_id.clone(), shape);
        shape_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_svg() {
        let svg = build_svg(640.0, 480.0);
        assert_eq!(svg.width, 640.0);
        assert_eq!(svg.height, 480.0);
        assert_eq!(svg.widget_id_count, 0);
        assert_eq!(svg.shape_define_map.len(), 0);
    }
    
    #[test]
    fn check_def_shape() {
        let mut svg = build_svg(640.0, 480.0);

        let rect1 = Rect::new(30.0, 20.0);
        let _rect_id = svg.def_shape(&rect1);
        assert_eq!(svg.widget_id_count, 1);
        assert_eq!(svg.shape_define_map.get("s1").unwrap().width, 30.0);

        let rect2 = Rect::new(10.0, 5.0);
        let _rect_id = svg.def_shape(&rect2);
        assert_eq!(svg.widget_id_count, 2);
        assert_eq!(svg.shape_define_map.get("s2").unwrap().width, 10.0);
    }
}
