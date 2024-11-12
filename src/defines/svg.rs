use std::collections::HashMap;
use crate::defines::shape::Shape;

pub struct Svg {
    pub width: f64,
    pub height: f64,
    pub widget_id_count: u32,
    pub shape_define_map: HashMap<String, &'static dyn Shape>,
}

pub fn new_svg(width: f64, height: f64) -> Svg {
    Svg {
        width,
        height,
        widget_id_count: 0,
        shape_define_map: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_svg() {
        let svg = new_svg(640.0, 480.0);
        assert_eq!(svg.width, 640.0);
        assert_eq!(svg.height, 480.0);
        assert_eq!(svg.widget_id_count, 0);
        assert_eq!(svg.shape_define_map.len(), 0);
    }
}
