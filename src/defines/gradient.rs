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
pub struct LinearGradient {
    pub stops: Vec<(f64, String, f64)>,
    pub x1: Option<f64>,
    pub y1: Option<f64>,
    pub x2: Option<f64>,
    pub y2: Option<f64>,
    pub gradientUnits: Option<GradientUnits>,
    pub spreadMethod: Option<SpreadMethod>,
}

impl LinearGradient {
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
                                      
                                      if self.x1.is_some() {
                                          optionItems.push(format!("x1={}", self.x1.as_ref().unwrap()));
                                      }

                                      if self.y1.is_some() {
                                          optionItems.push(format!("y1={}", self.y1.as_ref().unwrap()));
                                      }

                                      if self.x2.is_some() {
                                          optionItems.push(format!("x2={}", self.x2.as_ref().unwrap()));
                                      }

                                      if self.y2.is_some() {
                                          optionItems.push(format!("y2={}", self.y2.as_ref().unwrap()));
                                      }

                                      if self.gradientUnits.is_some() {
                                          optionItems.push(format!("gradientUnits={}", self.gradientUnits.as_ref().unwrap()));
                                      }

                                      if self.spreadMethod.is_some() {
                                          optionItems.push(format!("spreadMethod={}", self.spreadMethod.as_ref().unwrap()));
                                      }
                                      
                                      optionItems.join(" ")
                                  }));
        
        if self.stops.is_some() {
            for stop in self.stops.as_ref().unwrap() {
                fmt_str.push_str(&format!("      <stop offset=\"{}%\" stop-color=\"{}\" ", stop.0, stop.1));

          (when (not (= (list-ref (car stops) 2) 1))
            (printf "stop-opacity=\"~a\" " (~r (list-ref (car stops) 2))))
          (printf "/>\n")

            }
        }

        fmt_str.push_str(&format!("    </linearGradient>\n",

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
