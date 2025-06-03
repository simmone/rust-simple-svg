#![doc = include_str!("FILTER.md")]

use crate::tools::precision::svg_round;

#[derive(Debug, Clone)]
pub struct Filter {
    pub blur: Option<f64>,
    pub dropdown_offset: Option<f64>,
    pub dropdown_color: Option<String>,
    pub precision: usize,
}

impl Filter {
    pub fn new() -> Self {
        Filter {
            blur: Some(2.0),
            dropdown_offset: Some(3.0),
            dropdown_color: Some("black".to_string()),
            precision: 0,
        }
    }

    pub fn format(&self, shape_id: String) -> String {
        let mut fmt_str = String::new();

        fmt_str.push_str(&format!("    <filter id=\"{}\">\n", shape_id));
        fmt_str.push_str(&format!(
            "      <feGaussianBlur in=\"SourceAlpha\" stdDeviation=\"{}\"></feGaussianBlur>\n",
            svg_round(*self.blur.as_ref().unwrap(), self.precision)
        ));
        fmt_str.push_str(&format!(
            "      <feOffset dx=\"{}\" dy=\"{}\" result=\"offsetblur\"></feOffset>\n",
            svg_round(*self.dropdown_offset.as_ref().unwrap(), self.precision),
            svg_round(*self.dropdown_offset.as_ref().unwrap(), self.precision)
        ));
        fmt_str.push_str(&format!(
            "      <feFlood flood-color=\"{}\"></feFlood>\n",
            self.dropdown_color.as_ref().unwrap()
        ));
        fmt_str.push_str(&format!(
            "      <feComposite in2=\"offsetblur\" operator=\"in\"></feComposite>\n"
        ));
        fmt_str.push_str(&format!("      <feMerge>\n"));
        fmt_str.push_str(&format!("        <feMergeNode></feMergeNode>\n"));
        fmt_str.push_str(&format!(
            "        <feMergeNode in=\"SourceGraphic\"></feMergeNode>\n"
        ));
        fmt_str.push_str(&format!("      </feMerge>\n"));
        fmt_str.push_str(&format!("    </filter>\n"));

        fmt_str
    }

    pub fn unique(&self) -> String {
        format!(
            "Filter/blur/{:?}/dropdown_offset/{:?}/dropdown_color/{:?}",
            self.blur, self.dropdown_offset, self.dropdown_color
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_filter() {
        let filter = Filter::new();

        assert_eq!(filter.blur, Some(2.0));
        assert_eq!(filter.dropdown_offset, Some(3.0));
        assert_eq!(filter.dropdown_color, Some("black".to_string()));
    }

    #[test]
    fn check_format() {
        let filter = Filter::new();

        assert_eq!(Filter::format(&filter, "1".to_string()), {
            let mut c_str = String::new();

            c_str.push_str("    <filter id=\"1\">\n");
            c_str.push_str(
                "      <feGaussianBlur in=\"SourceAlpha\" stdDeviation=\"2\"></feGaussianBlur>\n",
            );
            c_str.push_str("      <feOffset dx=\"3\" dy=\"3\" result=\"offsetblur\"></feOffset>\n");
            c_str.push_str("      <feFlood flood-color=\"black\"></feFlood>\n");
            c_str
                .push_str("      <feComposite in2=\"offsetblur\" operator=\"in\"></feComposite>\n");
            c_str.push_str("      <feMerge>\n");
            c_str.push_str("        <feMergeNode></feMergeNode>\n");
            c_str.push_str("        <feMergeNode in=\"SourceGraphic\"></feMergeNode>\n");
            c_str.push_str("      </feMerge>\n");
            c_str.push_str("    </filter>\n");

            c_str
        });
    }
}
