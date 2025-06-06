use crate::define::sstyle::Sstyle;
use crate::tools::precision::svg_round;

#[derive(Default, Clone, Debug)]
pub struct Widget {
    pub shape_id: String,
    pub at: Option<(f64, f64)>,
    pub style: Option<Sstyle>,
    pub filter_id: Option<String>,
    pub marker_start_id: Option<String>,
    pub marker_mid_id: Option<String>,
    pub marker_end_id: Option<String>,
    pub precision: usize,
}

impl Widget {
    pub fn format(&self) -> String {
        let mut format_items = vec![];

        format_items.push(format!("<use xlink:href=\"#{}\"", self.shape_id));

        if self.at.is_some() && self.at.unwrap() != (0.0, 0.0) {
            format_items.push(format!(
                "x=\"{}\" y=\"{}\"",
                svg_round(self.at.unwrap().0, self.precision),
                svg_round(self.at.unwrap().1, self.precision)
            ));
        }

        if self.style.is_some() {
            format_items.push(self.style.as_ref().unwrap().format());
        }

        if self.filter_id.is_some() {
            format_items.push(format!(
                "filter=\"url(#{})\"",
                self.filter_id.clone().unwrap()
            ));
        }

        if self.marker_start_id.is_some() {
            format_items.push(format!(
                "marker-start=\"url(#{})\"",
                self.marker_start_id.clone().unwrap()
            ));
        }

        if self.marker_mid_id.is_some() {
            format_items.push(format!(
                "marker-mid=\"url(#{})\"",
                self.marker_mid_id.clone().unwrap()
            ));
        }

        if self.marker_end_id.is_some() {
            format_items.push(format!(
                "marker-end=\"url(#{})\"",
                self.marker_end_id.clone().unwrap()
            ));
        }

        format_items.push("/>".to_string());

        format_items.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_format_basic() {
        let widget = Widget {
            shape_id: "s1".to_string(),
            ..Default::default()
        };

        assert_eq!(widget.format(), "<use xlink:href=\"#s1\" />");
    }

    #[test]
    fn check_format_fill() {
        let mut rect_sstyle = Sstyle::new();
        rect_sstyle.fill = Some("#BBC42A".to_string());

        let widget = Widget {
            shape_id: "s1".to_string(),
            style: Some(rect_sstyle),
            ..Default::default()
        };

        assert_eq!(
            widget.format(),
            "<use xlink:href=\"#s1\" fill=\"#BBC42A\" />"
        );
    }

    #[test]
    fn check_format_at() {
        let widget1 = Widget {
            shape_id: "s1".to_string(),
            at: Some((0.0, 0.0)),
            ..Default::default()
        };
        assert_eq!(widget1.format(), "<use xlink:href=\"#s1\" />");

        let widget2 = Widget {
            shape_id: "s1".to_string(),
            at: Some((100.0, 50.0)),
            ..Default::default()
        };
        assert_eq!(
            widget2.format(),
            "<use xlink:href=\"#s1\" x=\"100\" y=\"50\" />"
        );

        let widget3 = Widget {
            shape_id: "s1".to_string(),
            at: Some((100.12345, 50.0)),
            ..Default::default()
        };
        assert_eq!(
            widget3.format(),
            "<use xlink:href=\"#s1\" x=\"100\" y=\"50\" />"
        );

        let widget4= Widget {
            shape_id: "s1".to_string(),
            at: Some((100.12345, 50.0)),
            precision: 3,
            ..Default::default()
        };
        assert_eq!(
            widget4.format(),
            "<use xlink:href=\"#s1\" x=\"100.123\" y=\"50\" />"
        );
    }

    #[test]
    fn check_format_else() {
        let widget = Widget {
            shape_id: "s1".to_string(),
            filter_id: Some("s2".to_string()),
            marker_start_id: Some("s3".to_string()),
            marker_mid_id: Some("s4".to_string()),
            marker_end_id: Some("s5".to_string()),
            ..Default::default()
        };
        assert_eq!(widget.format(), "<use xlink:href=\"#s1\" filter=\"url(#s2)\" marker-start=\"url(#s3)\" marker-mid=\"url(#s4)\" marker-end=\"url(#s5)\" />");
    }
}
