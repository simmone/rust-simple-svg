use crate::defines::shape::Shape;
use std::collections::HashMap;
use crate::defines::sstyle::Sstyle;

pub struct Svg {
    pub width: f64,
    pub height: f64,
    pub widget_id_count: u32,
    pub shape_define_map: HashMap<String, Shape>,
}

pub struct PlaceWidget {
    pub style Option<Sstyle>,
    pub at Option<(f64, f64)>,
    pub filter_id Option<String>,
    pub marker_start_id Option<String>,
    pub marker_mid_id Option<String>,
    pub marker_end_id Option<String>,
}

impl Svg {
    pub fn new(width: f64, height: f64) -> Self {
        Svg {
            width,
            height,
            widget_id_count: 0,
            shape_define_map: HashMap::new(),
        }
    }

    pub fn add_shape(&mut self, shape: Shape) -> String {
        self.widget_id_count += 1;
        let shape_id = format!("s{}", self.widget_id_count);
        self.shape_define_map.insert(shape_id.clone(), shape);
        shape_id
    }
    
    pub fn place_widget(&mut self, shape_id: String, arguments: PlaceWidget) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::defines::rect::Rect;

    #[test]
    fn check_new_svg() {
        let svg: Svg = Svg::new(640.0, 480.0);
        assert_eq!(svg.width, 640.0);
        assert_eq!(svg.height, 480.0);
        assert_eq!(svg.widget_id_count, 0);
        assert_eq!(svg.shape_define_map.len(), 0);
    }

    #[test]
    fn check_add_shape() {
        let mut svg: Svg = Svg::new(640.0, 480.0);

        let rect1 = Rect::new(30.0, 20.0);
        let shape1 = Shape::Rect(rect1);
        let _rect_id = svg.add_shape(shape1);
        assert_eq!(svg.widget_id_count, 1);
        match svg.shape_define_map.get("s1").unwrap() {
            Shape::Rect(s1) => {
                assert_eq!(s1.width, 30.0);
            }
        }

        let rect2 = Rect::new(10.0, 5.0);
        let shape2 = Shape::Rect(rect2);
        let _rect_id = svg.add_shape(shape2);
        assert_eq!(svg.widget_id_count, 2);
        match svg.shape_define_map.get("s2").unwrap() {
            Shape::Rect(s2) => {
                assert_eq!(s2.width, 10.0);
            }
        }
    }
    
    #[test]
    fn check_place_widget() {
        let mut svg: Svg = Svg::new(640.0, 480.0);

        let shape_id = svg.add_shape(Shape::Rect(Rect::new(30.0, 20.0)));

        let rect_sstyle = Sstyle::new();
        rect_sstyle.fill = Some("#BBC42A".to_string());
        
        svg.place_widget(shape_id, PlaceWidget{sstyle: rect_sstyle, ..});

        assert_eq!(svg.widget_id_count, 1);
        match svg.shape_define_map.get("s1").unwrap() {
            Shape::Rect(s1) => {
                assert_eq!(s1.width, 30.0);
            }
        }
    }
}
