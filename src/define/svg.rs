#![doc = include_str!("SVG.md")]

use crate::define::group::Group;
use crate::define::shape::rect::Rect;
use crate::define::shape::Shape;
use crate::define::sstyle::Sstyle;
use crate::define::viewbox::ViewBox;
use crate::define::widget::Widget;
use crate::tools::precision::svg_round;
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
    pub unique_shape_map: HashMap<String, String>,
    pub shape_define_map: HashMap<String, Shape>,
    pub group_define_map: HashMap<String, Group>,
    pub group_show_list: Vec<(String, (f64, f64))>,
    pub precision: usize,
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
            unique_shape_map: HashMap::new(),
            shape_define_map: HashMap::new(),
            group_define_map: HashMap::new(),
            group_show_list: Vec::new(),
            precision: 4,
        }
    }

    pub fn add_shape(&mut self, mut shape: Shape) -> String {
        shape.set_precision(self.precision);

        if self.unique_shape_map.contains_key(&shape.unique()) {
            self.unique_shape_map
                .get(&shape.unique())
                .unwrap()
                .to_string()
        } else {
            self.shape_id_count += 1;
            let shape_id = format!("s{}", self.shape_id_count);

            self.unique_shape_map
                .insert(shape.unique(), shape_id.clone());
            self.shape_define_map.insert(shape_id.clone(), shape);

            shape_id
        }
    }

    pub fn add_group(&mut self, group: Group) -> String {
        self.group_id_count += 1;
        let group_id = format!("g{}", self.group_id_count);
        self.add_name_group(group_id, group)
    }

    pub fn add_name_group(&mut self, group_id: String, mut group: Group) -> String {
        for widget in &mut group.widget_list {
            widget.precision = self.precision;

            if widget.style.is_some() {
                widget.style.as_mut().unwrap().precision = self.precision;
            }
        }

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

        let mut group = Group::default();
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
        if let Some(group) = group {
            for widget in group.widget_list.clone() {
                group_str.push_str(&prefix);
                group_str.push_str(&widget.format());
                group_str.push('\n');
            }
        }

        group_str
    }

    pub fn sort_id(ids: &mut [String]) {
        ids.sort_by(|a, b| {
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
                svg_str.push_str(&format!(
                    "x=\"{}\" y=\"{}\" ",
                    svg_round(group_pos.0, self.precision),
                    svg_round(group_pos.1, self.precision)
                ));
            }

            svg_str.push_str("/>\n");
        }

        // show default group
        let default_group = self.group_define_map.get(DEFAULT_GROUP_ID);
        if let Some(default_group) = default_group {
            if default_group.widget_list.len() > 0 {
                svg_str.push('\n');
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

#[test]
fn check_add_shape() {
    let mut svg: Svg = Svg::new(640.0, 480.0);

    let rect1 = Rect::new(30.0, 20.0);
    let shape1 = Shape::Rect(rect1);
    let _rect_id = svg.add_shape(shape1);
    assert_eq!(svg.shape_id_count, 1);
    match svg.shape_define_map.get("s1").unwrap() {
        Shape::Rect(s1) => {
            assert_eq!(s1.width, 30.0);
        }
        Shape::Circle(_) => {}
        Shape::Ellipse(_) => {}
        Shape::Line(_) => {}
        Shape::Polygon(_) => {}
        Shape::Polyline(_) => {}
        Shape::Filter(_) => {}
        Shape::LinearGradient(_) => {}
        Shape::RadialGradient(_) => {}
        Shape::Path(_) => {}
        Shape::Text(_) => {}
        Shape::Marker(_) => {}
        Shape::Arrow(_) => {}
    }

    let rect2 = Rect::new(10.0, 5.0);
    let shape2 = Shape::Rect(rect2);
    let _rect_id = svg.add_shape(shape2);
    assert_eq!(svg.shape_id_count, 2);
    match svg.shape_define_map.get("s2").unwrap() {
        Shape::Rect(s2) => {
            assert_eq!(s2.width, 10.0);
        }
        Shape::Circle(_) => {}
        Shape::Ellipse(_) => {}
        Shape::Line(_) => {}
        Shape::Polygon(_) => {}
        Shape::Polyline(_) => {}
        Shape::Filter(_) => {}
        Shape::LinearGradient(_) => {}
        Shape::RadialGradient(_) => {}
        Shape::Path(_) => {}
        Shape::Text(_) => {}
        Shape::Marker(_) => {}
        Shape::Arrow(_) => {}
    }
}

#[test]
fn check_shape_sort() {
    let mut ids: Vec<String> = vec!["s10", "s1", "s2", "g3", "g0"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    Svg::sort_id(&mut ids);

    let expected_strings: Vec<String> = vec!["g0", "s1", "s2", "g3", "s10"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    assert_eq!(ids, expected_strings);
}

#[test]
fn check_add_group() {
    let mut svg: Svg = Svg::new(640.0, 480.0);
    let shape_id = svg.add_shape(Shape::Rect(Rect::new(30.0, 20.0)));

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: shape_id,
        ..Default::default()
    });

    svg.add_group(group);

    assert_eq!(svg.group_define_map.len(), 1);
    assert_eq!(
        svg.group_define_map.get("g2").unwrap().widget_list[0].shape_id,
        "s1".to_string()
    );
}

#[test]
fn check_flush_data() {
    let mut svg: Svg = Svg::new(100.0, 100.0);
    let shape1_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));

    let mut rect_sstyle = Sstyle::new();
    rect_sstyle.fill = Some("#BBC42A".to_string());

    let mut group = Group::new();
    group.place_widget(Widget {
        shape_id: shape1_id,
        style: Some(rect_sstyle.clone()),
        ..Default::default()
    });

    let shape2_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));
    group.place_widget(Widget {
        shape_id: shape2_id,
        style: Some(rect_sstyle.clone()),
        ..Default::default()
    });

    let shape3_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));
    group.place_widget(Widget {
        shape_id: shape3_id,
        style: Some(rect_sstyle.clone()),
        ..Default::default()
    });

    let shape4_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));
    group.place_widget(Widget {
        shape_id: shape4_id,
        style: Some(rect_sstyle.clone()),
        ..Default::default()
    });

    let shape5_id = svg.add_shape(Shape::Rect(Rect::new(100.0, 100.0)));
    group.place_widget(Widget {
        shape_id: shape5_id,
        style: Some(rect_sstyle.clone()),
        ..Default::default()
    });

    svg.add_default_group(group);

    let mut expected_str = String::new();
    expected_str.push_str("  <defs>\n");
    expected_str.push_str("    <rect id=\"s1\" width=\"100\" height=\"100\" />\n");
    expected_str.push_str("  </defs>\n\n");
    expected_str.push_str("  <use xlink:href=\"#s1\" fill=\"#BBC42A\" />\n");
    expected_str.push_str("  <use xlink:href=\"#s1\" fill=\"#BBC42A\" />\n");
    expected_str.push_str("  <use xlink:href=\"#s1\" fill=\"#BBC42A\" />\n");
    expected_str.push_str("  <use xlink:href=\"#s1\" fill=\"#BBC42A\" />\n");
    expected_str.push_str("  <use xlink:href=\"#s1\" fill=\"#BBC42A\" />\n");

    assert_eq!(svg.flush_data(), expected_str);
}
