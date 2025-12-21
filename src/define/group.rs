#![doc = include_str!("GROUP.md")]

use crate::define::widget::Widget;

/// Group contains a widget list.
#[derive(Clone, Default)]
pub struct Group {
    pub widget_list: Vec<Widget>,
}

impl Group {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn place_widget(&mut self, widget: Widget) {
        self.widget_list.push(widget);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::define::shape::rect::Rect;
    use crate::define::shape::Shape;
    use crate::define::sstyle::Sstyle;
    use crate::define::svg::Svg;

    #[test]
    fn check_new() {
        let group = Group::new();

        assert_eq!(group.widget_list.len(), 0);
    }

    #[test]
    fn check_place_widget() {
        let mut svg = Svg::new(640.0, 480.0);
        let shape_id = svg.add_shape(Shape::Rect(Rect::new(30.0, 20.0)));

        let mut rect_sstyle = Sstyle::new();
        rect_sstyle.fill = Some("#BBC42A".to_string());

        let mut group = Group::new();
        group.place_widget(Widget {
            shape_id: shape_id,
            style: Some(rect_sstyle),
            ..Default::default()
        });

        assert_eq!(group.widget_list.len(), 1);
        assert_eq!(group.widget_list[0].shape_id, "s1".to_string());
    }
}
