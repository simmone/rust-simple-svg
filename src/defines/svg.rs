use crate::defines::group::Group;
use crate::defines::shape::Shape;
use std::collections::HashMap;

pub static DEFAULT_GROUP_ID: &str = "d0";
pub static BACKGROUND_GROUP_ID: &str = "b0";

pub struct Svg<'a> {
    pub width: f64,
    pub height: f64,
    pub background: Option<String>,
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
            background: None,
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
        self.add_name_group(group_id, group)
    }

    pub fn add_name_group(&mut self, group_id: String, group: Group<'a>) -> String {
        self.group_define_map.insert(group_id.clone(), group);
        group_id
    }

    pub fn add_default_group(&mut self, group: Group<'a>) -> String {
        self.add_name_group(DEFAULT_GROUP_ID.to_string(), group)
    }

    pub fn show_group_widgets(&self, group_id: String, prefix: String) -> String {
        let group = self.group_define_map.get(&group_id);

        let mut group_str = String::new();
        if group.is_some() {
            for widget in &group.unwrap().widget_list {
                group_str.push_str(&prefix);
                group_str.push_str(&widget.format());
                group_str.push_str("\n");
            }
        }

        group_str
    }

    pub fn flush_data(&self) -> String {
        let mut svg_str = String::new();

        if self.shape_define_map.len() > 0 {
            svg_str.push_str("  <defs>\n");

            let shape_define_map_c = self.shape_define_map.clone();
            let mut shape_ids: Vec<String> = shape_define_map_c.into_keys().collect();
            shape_ids.sort();

            for shape_id in shape_ids {
                let shape = self.shape_define_map.get(&shape_id).unwrap();

                svg_str.push_str(&format!("{}", shape.format(shape_id.to_string())));
            }

            svg_str.push_str("  </defs>\n\n");
        }

        let default_group = self.group_define_map.get(DEFAULT_GROUP_ID);
        if default_group.is_some() {
            if default_group.unwrap().widget_list.len() > 0 {
                svg_str.push_str(
                    &self.show_group_widgets(DEFAULT_GROUP_ID.to_string(), "  ".to_string()),
                );
            }
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
