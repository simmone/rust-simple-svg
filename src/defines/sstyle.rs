pub enum FillRule {
    nonzero,
    evenodd,
    inerit,
}

pub enum LineCap {
    butt,
    round,
    square,
    inherit,
}

pub enum LineJoin {
    miter,
    round,
    bevel,
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

impl Sstye {
    pub new() -> Self {
        Sstyle {
            None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None
        }
    }
    
    pub format() -> String {
        let mut format_str = String::new();
        
        if self.fill_gradient.is_some() {
            format_str.push_str(&format!(" fill=\"url(#{})\"" self.fill_gradient));
            
            if self.fill.is_some() {
                format_str.push_str(&format!(" fill=\"{}\"" self.fill));
            } else {
                format_str.push_str(" fill=\"none\"");
            }
        }
    }
}
