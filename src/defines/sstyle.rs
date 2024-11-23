pub enum FillRule {
    Nonzero,
    Evenodd,
    Inerit,
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
    pub skewX: Option<f64>,
    pub skewY: Option<f64>,
    pub fill_gradient: Option<String>,
}

impl Sstyle {
    pub fn new() -> Self {
        Sstyle {
            fill: None, fill_ruler: None, fill_opacity: None, stroke: None, stroke_width: None, stroke_linecap: None, stroke_linejoin: None, stroke_miterlimit: None, stroke_dasharray: None, stroke_dashoffset: None,
            translate: None, rotate: None, scale: None, skewX: None, skewY: None, fill_gradient: None
        }
    }
    
    pub fn format(&self) -> String {
        let mut format_str = String::new();
        
        if self.fill_gradient.is_some() {
            format_str.push_str(&format!(" fill=\"url(#{})\"", self.fill_gradient.unwrap()));
        } else {
            if self.fill.is_some() {
                format_str.push_str(&format!(" fill=\"{}\"", self.fill.unwrap()));
            } else {
                format_str.push_str(" fill=\"none\"");
            }
        }

        if self.fill_ruler.is_some() {
            format_str.push_str(&format!(" fill-rule=\"{}\"", self.fill_ruler.unwrap()));
        }

        if self.fill_opacity.is_some() {
            format_str.push_str(&format!(" fill-opacity=\"{}\"", self.fill_opacity.unwrap()));
        }
        
        format_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_format() {
        let mut sstyle = Sstyle::new();
        
        sstyle.fill = "red";
        sstyle.fill_ruler = FillRule.nonzero;
        sstyle.gradient = "s1";
        sstyle.opacity = 30.0;
        
        assert_eq!(
            Sstyle::format(&sstyle),
            "");
    }
}
