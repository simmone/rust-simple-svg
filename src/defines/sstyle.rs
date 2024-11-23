use std::fmt;

pub enum FillRule {
    Nonzero,
    Evenodd,
    Inerit,
}

impl fmt::Display for FillRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FillRule::Nonzero => write!(f, "nonzero"),
            FillRule::Evenodd => write!(f, "evenodd"),
            FillRule::Inerit => write!(f, "inerit"),
        }
    }
}

pub enum LineCap {
    Butt,
    Round,
    Square,
    Inherit,
}

pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

pub struct Sstyle {
    pub fill: Option<String>,
    pub fill_ruler: Option<FillRule>,
    pub fill_opacity: Option<f64>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f64>,
    pub stroke_linecap: Option<LineCap>,
    pub stroke_linejoin: Option<LineJoin>,
    pub stroke_miterlimit: Option<f64>,
    pub stroke_dasharray: Option<String>,
    pub stroke_dashoffset: Option<f64>,
    pub translate: Option<(f64, f64)>,
    pub rotate: Option<f64>,
    pub scale: Option<(f64, f64)>,
    pub skew_x: Option<f64>,
    pub skew_y: Option<f64>,
    pub fill_gradient: Option<String>,
}

impl Sstyle {
    pub fn new() -> Self {
        Sstyle {
            fill: None, fill_ruler: None, fill_opacity: None, stroke: None, stroke_width: None, stroke_linecap: None, stroke_linejoin: None, stroke_miterlimit: None, stroke_dasharray: None, stroke_dashoffset: None,
            translate: None, rotate: None, scale: None, skew_x: None, skew_y: None, fill_gradient: None
        }
    }
    
    pub fn format(&self) -> String {
        let mut format_str = String::new();
        
        if self.fill_gradient.is_some() {
            format_str.push_str(&format!(" fill=\"url(#{})\"", self.fill_gradient.as_ref().unwrap()));
        } else {
            if self.fill.is_some() {
                format_str.push_str(&format!(" fill=\"{}\"", self.fill.as_ref().unwrap()));
            } else {
                format_str.push_str(" fill=\"none\"");
            }
        }

        if self.fill_ruler.is_some() {
            format_str.push_str(&format!(" fill-rule=\"{}\"", self.fill_ruler.as_ref().unwrap()));
        }

        if self.fill_opacity.is_some() {
            format_str.push_str(&format!(" fill-opacity=\"{}\"", self.fill_opacity.as_ref().unwrap()));
        }
        
        format_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_format1() {
        let mut sstyle = Sstyle::new();
        
        sstyle.fill_ruler = Some(FillRule::Nonzero);
        sstyle.fill_gradient = Some("s1".to_string());
        sstyle.fill_opacity = Some(30.0);
        
        assert_eq!(
            Sstyle::format(&sstyle),
            " fill=\"url(#s1)\" fill-rule=\"nonzero\" fill-opacity=\"30\"");
    }

    #[test]
    fn check_format2() {
        let mut sstyle = Sstyle::new();
        
        sstyle.fill = Some("red".to_string());
        sstyle.fill_ruler = Some(FillRule::Nonzero);
        sstyle.fill_opacity = Some(30.0);
        
        assert_eq!(
            Sstyle::format(&sstyle),
            " fill=\"red\" fill-rule=\"nonzero\" fill-opacity=\"30\"");
    }

}
