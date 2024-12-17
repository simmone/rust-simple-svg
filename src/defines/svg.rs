use crate::defines::shape::Shape;
use crate::defines::group::Group;
use std::collections::HashMap;

pub struct Svg {
    pub width: f64,
    pub height: f64,
    pub widget_id_count: usize,
    pub shape_define_map: HashMap<String, Shape>,
    pub group_define_map: HashMap<String, Group>,
    pub group_show_list: Vec<Group>,
}

impl Svg {
    pub fn new(width: f64, height: f64) -> Self {
        Svg {
            width,
            height,
            widget_id_count: 0,
            shape_define_map: HashMap::new(),
            group_define_map: HashMap::new(),
            group_show_list: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, shape: Shape) -> String {
        self.widget_id_count += 1;
        let shape_id = format!("s{}", self.widget_id_count);
        self.shape_define_map.insert(shape_id.clone(), shape);
        shape_id
    }
    
    pub fn add_group(&mut self, group: Group) -> String {
        self.widget_id_count += 1;
        let group_id = format!("g{}", self.widget_id_count);
        self.group_define_map.insert(group_id.clone(), group);
        group_id
    }
    
    pub fn flush_data(&self) -> String {
        let mut svg_str = String::new();
        
        if self.shape_define_map.len() > 0 {
            svg_str.push_str("  <defs>\n");
            
            for (shape_id, shape) in &self.shape_define_map {
                svg_str.push_str(&format!("{}", shape.format(shape_id.to_string())));
            }
        }
        
        svg_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::defines::rect::Rect;
    use crate::defines::widget::Widget;
    use crate::defines::sstyle::Sstyle;

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
    fn check_add_group() {
        let mut svg: Svg = Svg::new(640.0, 480.0);
        let shape_id = svg.add_shape(Shape::Rect(Rect::new(30.0, 20.0)));

        let mut group = Group::new();
        group.place_widget(Widget{shape_id: shape_id, ..Default::default()});
        
        let group_id = svg.add_group(group);

        assert_eq!(svg.group_define_map.len(), 1);
        assert_eq!(svg.group_define_map.get("g2").unwrap().widget_list[0].shape_id, "s1".to_string());
    }
    
    #[test]
    fn check_flush_data() {
        let mut svg: Svg = Svg::new(100.0, 100.0);
        let shape_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));

        let mut rect_sstyle = Sstyle::new();
        rect_sstyle.fill = Some("#BBC42A".to_string());

        let mut group = Group::new();
        group.place_widget(Widget{shape_id: shape_id, style: Some(rect_sstyle), ..Default::default()});
        
        let group_id = svg.add_group(group);
        
        let mut expected_str = String::new();
        expected_str.push_str("  <defs>\n");
        expected_str.push_str("    <rect id=\"s1\" width=\"100.0\" height=\"100.00\" />\n");
        expected_str.push_str("  </defs>\n\n");
        expected_str.push_str("<use xlink:href=\"#s1\" fill=\"#BBC42A\" />\n");
        
        assert_eq!(svg.flush_data(), expected_str);
    }
}
