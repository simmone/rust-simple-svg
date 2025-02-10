#[derive(Clone)]
pub enum GradientUnits {
    UserSpaceOnUse,
    ObjectBoundingBox,
}

impl fmt::Display for GradientUnits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GradientUnits::UserSpaceOnUse => write!(f, "userSpaceOnUse"),
            GradientUnits::ObjectBoundingBox => write!(f, "objectBoundingBox"),
        }
    }
}

#[derive(Clone)]
pub enum SpreadMethod {
    Repeat,
    Reflect,
}

impl fmt::Display for SpreadMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpreadMethod::Repeat => write!(f, "repeat"),
            SpreadMethod::Reflect => write!(f, "reflect"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gradient {
    pub stops: Vec<(f64, String, f64)>,
    pub x1: Option<f64>,
    pub y1: Option<f64>,
    pub x2: Option<f64>,
    pub y2: Option<f64>,
    pub gradientUnits: Option<GradientUnits>,
    pub spreadMethod: Option<SpreadMethod>,
}

impl Gradient {
    pub fn new(Vec<(f64, STring, f64): stops) -> Self {
        Gradient {
            stops,
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            gradientUnits: None,
            spreadMethod: None,
        }
    }

    pub fn format(&self, shape_id: String) -> String {
        let mut fmt_str = String::new();

        fmt_str.push_str(&format!("    <linearGradient id=\"{}\" {}>\n",
                                  shape_id,
                                  {
                                      let mut optionItems = vec![];
                                      
                                      

              (string-join
               (filter
                (lambda (a) (not (string=? a "")))
                (list
                 (if (LINEAR-GRADIENT-x1 gradient) (format "x1=\"~a\"" (~r (LINEAR-GRADIENT-x1 gradient))) "")
                 (if (LINEAR-GRADIENT-y1 gradient) (format "y1=\"~a\"" (~r (LINEAR-GRADIENT-y1 gradient))) "")
                 (if (LINEAR-GRADIENT-x2 gradient) (format "x2=\"~a\"" (~r (LINEAR-GRADIENT-x2 gradient))) "")
                 (if (LINEAR-GRADIENT-y2 gradient) (format "y2=\"~a\"" (~r (LINEAR-GRADIENT-y2 gradient))) "")
                 (if (LINEAR-GRADIENT-gradientUnits gradient) (format "gradientUnits=\"~a\"" (LINEAR-GRADIENT-gradientUnits gradient)) "")
                 (if (LINEAR-GRADIENT-spreadMethod gradient) (format "spreadMethod=\"~a\"" (LINEAR-GRADIENT-spreadMethod gradient)) "")))))


        fmt_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_gradient() {
        let gradient = Gradient::new();

        assert_eq!(gradient.blur, Some(2.0));
        assert_eq!(gradient.dropdown_offset, Some(3.0));
        assert_eq!(gradient.dropdown_color, Some("black".to_string()));
    }

    #[test]
    fn check_format() {
        let gradient = Gradient::new();

        assert_eq!(Gradient::format(&gradient, "1".to_string()), {
            let mut c_str = String::new();

            c_str.push_str("    <gradient id=\"1\">\n");
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
            c_str.push_str("    </gradient>\n");

            c_str
        });
    }
}
