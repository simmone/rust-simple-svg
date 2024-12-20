use crate::defines::shape::Shape;
use crate::defines::group::Group;
use std::collections::HashMap;

pub struct Svg<'a> {
    pub width: f64,
    pub height: f64,
    pub widget_id_count: usize,
    pub shape_define_map: HashMap<String, Shape>,
    pub group_define_map: HashMap<String, Group<'a>>,
    pub group_show_list: Vec<Group<'a>>,
}

impl<'a> Svg<'a> {
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
    
    pub fn add_group(&mut self, group: Group<'a>) -> String {
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

            svg_str.push_str("  </defs>\n\n");
        }
        
        svg_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_svg() {
        let svg: Svg = Svg::new(640.0, 480.0);
        assert_eq!(svg.width, 640.0);
        assert_eq!(svg.height, 480.0);
        assert_eq!(svg.widget_id_count, 0);
        assert_eq!(svg.shape_define_map.len(), 0);
    }
}
