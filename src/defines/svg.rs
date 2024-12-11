use crate::defines::rect::build_rect;
use crate::defines::rect::Rect;
use crate::defines::shape::Shape;
use std::collections::HashMap;

pub struct Svg<'a, T> {
    pub width: f64,
    pub height: f64,
    pub widget_id_count: u32,
    pub shape_define_map: HashMap<String, &'a T>,
}

pub fn build_svg<'a, T>(width: f64, height: f64) -> Svg<'a, T> {
    Svg {
        width,
        height,
        widget_id_count: 0,
        shape_define_map: HashMap::new(),
    }
}

impl<'a, T> Svg<'a, T> {
    pub fn def_shape(&mut self, shape: &'a T) -> String {
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
    fn check_new_svg<T>() {
        let svg:Svg<T> = build_svg::<T>(640.0, 480.0);
        assert_eq!(svg.width, 640.0);
        assert_eq!(svg.height, 480.0);
        assert_eq!(svg.widget_id_count, 0);
        assert_eq!(svg.shape_define_map.len(), 0);
    }

    #[test]
    fn check_def_shape() {
        let mut svg:Svg<_,_> = build_svg(640.0, 480.0);

        let rect1 = build_rect(30.0, 20.0);
        let _rect_id = svg.def_shape(&rect1);
        assert_eq!(svg.widget_id_count, 1);
        assert_eq!(svg.shape_define_map.get("s1").unwrap().width, 30.0);

        let rect2 = build_rect(10.0, 5.0);
        let _rect_id = svg.def_shape(&rect2);
        assert_eq!(svg.widget_id_count, 2);
        assert_eq!(svg.shape_define_map.get("s2").unwrap().width, 10.0);
    }
}
