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

impl fmt::Display for LineCap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LineCap::Butt => write!(f, "butt"),
            LineCap::Round => write!(f, "round"),
            LineCap::Square => write!(f, "square"),
            LineCap::Inherit => write!(f, "inherit"),
        }
    }
}

pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

impl fmt::Display for LineJoin {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            LineJoin::Miter => write!(f, "miter"),
            LineJoin::Round => write!(f, "round"),
            LineJoin::Bevel => write!(f, "bevel"),
        }
    }
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
        
        if self.stroke_width.is_some() {
            format_str.push_str(&format!(" stroke-width=\"{}\"", self.stroke_width.as_ref().unwrap()));
        }

        if self.stroke.is_some() {
            format_str.push_str(&format!(" stroke=\"{}\"", self.stroke.as_ref().unwrap()));
        }

        if self.stroke_linejoin.is_some() {
            format_str.push_str(&format!(" stroke-linejoin=\"{}\"", self.stroke_linejoin.as_ref().unwrap()));
        }

        if self.stroke_linecap.is_some() {
            format_str.push_str(&format!(" stroke-linecap=\"{}\"", self.stroke_linecap.as_ref().unwrap()));
        }

        if self.stroke_miterlimit.is_some() {
            format_str.push_str(&format!(" stroke-miterlimit=\"{}\"", self.stroke_miterlimit.as_ref().unwrap()));
        }

        if self.stroke_dasharray.is_some() {
            format_str.push_str(&format!(" stroke-dasharray=\"{}\"", self.stroke_dasharray.as_ref().unwrap()));
        }

        if self.stroke_dashoffset.is_some() {
            format_str.push_str(&format!(" stroke-dashoffset=\"{}\"", self.stroke_dashoffset.as_ref().unwrap()));
        }
        
        if self.translate.is_some() || self.rotate.is_some() || self.scale.is_some() || self.skew_x.is_some() || self.skew_y.is_some() {
            format_str.push_str(" transform=\"{".to_string());
            
            if self.translate.is_some() {
                format_str.push_str(&format!(" translate({} {}", self.translate.as_ref().unwrap().0, self.translate.1));
            }
            
            if self.rotate.is_some() {
                format_str.push_str(&format!(" rotate({})", self.rotate.unwrap));
            }
                

        (printf " transform=\"~a\""
                (string-join
                 (filter
                  (lambda (a) (not (string=? a "")))
                  (list
                   (if (SSTYLE-translate _sstyle)
                       (format "translate(~a ~a)"
                               (~r (car (SSTYLE-translate _sstyle)))
                               (~r (cdr (SSTYLE-translate _sstyle))))
                       "")
                   (if (SSTYLE-rotate _sstyle)
                       (format "rotate(~a)" (~r (SSTYLE-rotate _sstyle)))
                       "")
                   (if (SSTYLE-scale _sstyle)
                       (if (pair? (SSTYLE-scale _sstyle))
                           (format "scale(~a ~a)"
                                   (~r (car (SSTYLE-scale _sstyle)))
                                   (~r (cdr (SSTYLE-scale _sstyle))))
                           (format "scale(~a)" (~r (SSTYLE-scale _sstyle))))
                       "")
                   (if (SSTYLE-skewX _sstyle)
                       (format "skewX(~a)" (~r (SSTYLE-skewX _sstyle)))
                       "")
                   (if (SSTYLE-skewY _sstyle)
                       (format "skewY(~a)" (~r (SSTYLE-skewY _sstyle)))
                       ""))))))

        
        format_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_format_fill1() {
        let mut sstyle = Sstyle::new();
        
        sstyle.fill_ruler = Some(FillRule::Nonzero);
        sstyle.fill_gradient = Some("s1".to_string());
        sstyle.fill_opacity = Some(30.0);
        
        assert_eq!(
            Sstyle::format(&sstyle),
            " fill=\"url(#s1)\" fill-rule=\"nonzero\" fill-opacity=\"30\"");
    }

    #[test]
    fn check_format_fill2() {
        let mut sstyle = Sstyle::new();
        
        sstyle.fill = Some("red".to_string());
        sstyle.fill_ruler = Some(FillRule::Nonzero);
        sstyle.fill_opacity = Some(30.0);
        
        assert_eq!(
            Sstyle::format(&sstyle),
            " fill=\"red\" fill-rule=\"nonzero\" fill-opacity=\"30\"");
    }

    #[test]
    fn check_format_stroke() {
        let mut sstyle = Sstyle::new();
        
        sstyle.stroke_width = Some(5.0);
        sstyle.stroke = Some("#ABABAB".to_string());
        sstyle.stroke_linejoin = Some(LineJoin::Round);
        sstyle.stroke_linecap = Some(LineCap::Square);
        sstyle.stroke_miterlimit = Some(2.0);
        sstyle.stroke_dasharray = Some("40,10".to_string());
        sstyle.stroke_dashoffset = Some(5.0);
        
        assert_eq!(
            Sstyle::format(&sstyle),
            " fill=\"none\" stroke-width=\"5\" stroke=\"#ABABAB\" stroke-linejoin=\"round\" stroke-linecap=\"square\" stroke-miterlimit=\"2\" stroke-dasharray=\"40,10\" stroke-dashoffset=\"5\""
        );
    }
}
