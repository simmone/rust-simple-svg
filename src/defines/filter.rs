#[derive(Debug, Clone)]
pub struct Filter {
    pub blur: Option<f64>,
    pub dropdown_offset: Option<f64>,
    pub dropdown_color: Option<f64>,
}

impl Filter {
    pub fn new() -> Self {
        Filter {
            blur: 2.0,
            dropdown_offset: 3.0,
            dropdown_color: "black",
        }
    }

    pub fn format(&self, shape_id: String) -> String {
        let mut fmt_str = String::new();

        fmt_str.push_str(&format!("    <filter id=\"{}\">\n", shape_id));
        fmt_str.push_str(&format!("      <feGaussianBlur in=\"SourceAlpha\" stdDeviation=\"{}\"></feGaussianBlur>\n", self.blur));
        fmt_str.push_str(&format!("      <feOffset dx=\"{}\" dy=\"{}\" result=\"offsetblur\"></feOffset>\n", self.dropdown_offset, self.dropdown_offset));
        fmt_str.push_str(&format!("      <feFlood flood-color=\"{}\"></feFlood>\n", self.dropdown_color));
        fmt_str.push_str(&format!("      <feComposite in2=\"offsetblur\" operator=\"in\"></feComposite>\n"));
        fmt_str.push_str(&format!("      <feMerge>\n"));
        fmt_str.push_str(&format!("        <feMergeNode></feMergeNode>\n"));
        fmt_str.push_str(&format!("        <feMergeNode in=\"SourceGraphic\"></feMergeNode>\n"));
        fmt_str.push_str(&format!("      </feMerge>\n"));
        fmt_str.push_str(&format!("    </filter>\n"));
        
        fmt_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_filter() {
        let filter = Filter::new();

        assert_eq!(filter.blur, 2.0);
        assert_eq!(filter.dropdown_offset, 3.0);
        assert_eq!(filter.dropdown_color, "black");
    }

    #[test]
    fn check_format() {
        let filter = Filter::new();

        assert_eq!(
            Filter::format(&filter, "1".to_string()),
            "    <filter id=\"1\" r=\"30\" />\n"
    <filter id="1">
      <feGaussianBlur in="SourceAlpha" stdDeviation="2"></feGaussianBlur>
      <feOffset dx="3" dy="3" result="offsetblur"></feOffset>
      <feFlood flood-color="black"></feFlood>
      <feComposite in2="offsetblur" operator="in"></feComposite>
      <feMerge>
        <feMergeNode></feMergeNode>
        <feMergeNode in="SourceGraphic"></feMergeNode>
      </feMerge>
    </filter>

        );
    }
}
