use crate::defines::group::Group;
use crate::defines::rect::Rect;
use crate::defines::shape::Shape;
use crate::defines::sstyle::Sstyle;
use crate::defines::viewbox::ViewBox;
use crate::defines::widget::Widget;
use std::collections::HashMap;

pub static DEFAULT_GROUP_ID: &str = "g0";
pub static BACKGROUND_GROUP_ID: &str = "g1";

pub struct Svg {
    pub width: f64,
    pub height: f64,
    pub background: Option<String>,
    pub view_box: Option<ViewBox>,
    pub shape_id_count: usize,
    pub group_id_count: usize,
    pub shape_define_map: HashMap<String, Shape>,
    pub group_define_map: HashMap<String, Group>,
    pub group_show_list: Vec<(String, (f64, f64))>,
}

impl Svg {
    pub fn new(width: f64, height: f64) -> Self {
        Svg {
            width,
            height,
            background: None,
            view_box: None,
            shape_id_count: 0,
            group_id_count: 1,
            shape_define_map: HashMap::new(),
            group_define_map: HashMap::new(),
            group_show_list: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, shape: Shape) -> String {
        self.shape_id_count += 1;
        let shape_id = format!("s{}", self.shape_id_count);
        self.shape_define_map.insert(shape_id.clone(), shape);
        shape_id
    }

    pub fn add_group(&mut self, group: Group) -> String {
        self.group_id_count += 1;
        let group_id = format!("g{}", self.group_id_count);
        self.add_name_group(group_id, group)
    }

    pub fn add_name_group(&mut self, group_id: String, group: Group) -> String {
        self.group_define_map.insert(group_id.clone(), group);
        group_id
    }

    pub fn add_default_group(&mut self, group: Group) -> String {
        self.add_name_group(DEFAULT_GROUP_ID.to_string(), group)
    }

    pub fn set_background(&mut self, background: String) {
        self.background = Some(background.clone());

        let rect_id = self.add_shape(Shape::Rect(Rect::new(self.width, self.height)));

        let mut background_sstyle = Sstyle::new();
        background_sstyle.fill = Some(background);

        let mut group = Group::new();
        group.place_widget(Widget {
            shape_id: rect_id,
            style: Some(background_sstyle),
            ..Default::default()
        });

        self.add_name_group(BACKGROUND_GROUP_ID.to_string(), group);
    }

    pub fn set_viewbox(&mut self, min_x: f64, min_y: f64, width: f64, height: f64) {
        self.view_box = Some(ViewBox::new(min_x, min_y, width, height));
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
    
    pub fn sort_id(ids: &mut Vec::<String>) {
        ids.sort_by(|a, b|
                    {
                        let va: Vec<&str> = a.split(char::is_alphabetic).collect();
                        let vb: Vec<&str> = b.split(char::is_alphabetic).collect();
                        let van = va[1].parse::<usize>().unwrap();
                        let vbn = vb[1].parse::<usize>().unwrap();
                        van.cmp(&vbn)
                    });
    }

    pub fn flush_data(&self) -> String {
        let mut svg_str = String::new();

        // defs
        if self.shape_define_map.len() > 0 {
            svg_str.push_str("  <defs>\n");

            let mut shape_ids: Vec<String> = self.shape_define_map.clone().into_keys().collect();
            Svg::sort_id(&mut shape_ids);
            for shape_id in shape_ids {
                let shape = self.shape_define_map.get(&shape_id).unwrap();

                svg_str.push_str(&format!("{}", shape.format(shape_id.to_string())));
            }

            svg_str.push_str("  </defs>\n");
        }

        // group defines
        let mut group_ids: Vec<String> = self
            .group_define_map
            .clone()
            .into_keys()
            .filter(|group_id| group_id != DEFAULT_GROUP_ID)
            .collect();

        Svg::sort_id(&mut group_ids);

        for group_id in group_ids {
            svg_str.push_str("\n");
            svg_str.push_str(&format!("  <symbol id=\"{}\">\n", group_id));
            svg_str.push_str(&self.show_group_widgets(group_id, "    ".to_string()));
            svg_str.push_str("  </symbol>\n");
        }

        // show group in order except default group
        let group_shows: Vec<(String, (f64, f64))> = self
            .group_show_list
            .clone()
            .into_iter()
            .filter(|group_show| group_show.0 != DEFAULT_GROUP_ID.to_string())
            .collect();

        if group_shows.len() > 0 {
            svg_str.push_str("\n");
        }

        for group_show in group_shows {
            let group_id = group_show.0;
            let group_pos = group_show.1;

            svg_str.push_str(&format!("  <use xlink:href=\"#{group_id}\" "));

            if group_pos != (0.0, 0.0) {
                svg_str.push_str(&format!("x=\"{}\" y=\"{}\" ", group_pos.0, group_pos.1));
            }

            svg_str.push_str("/>\n");
        }

        // show default group
        let default_group = self.group_define_map.get(DEFAULT_GROUP_ID);
        if default_group.is_some() {
            if default_group.unwrap().widget_list.len() > 0 {
                svg_str.push_str("\n");
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
        assert_eq!(svg.shape_id_count, 0);
        assert_eq!(svg.group_id_count, 1);
        assert_eq!(svg.shape_define_map.len(), 0);
    }
}
