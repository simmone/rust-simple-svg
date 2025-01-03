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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LineJoin::Miter => write!(f, "miter"),
            LineJoin::Round => write!(f, "round"),
            LineJoin::Bevel => write!(f, "bevel"),
        }
    }
}

pub struct Sstyle {
    pub fill: Option<String>,
    pub fill_rule: Option<FillRule>,
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
    pub scale_all: Option<f64>,
    pub scale_xy: Option<(f64, f64)>,
    pub skew_x: Option<f64>,
    pub skew_y: Option<f64>,
    pub fill_gradient: Option<String>,
}

impl Sstyle {
    pub fn new() -> Self {
        Sstyle {
            fill: None,
            fill_rule: None,
            fill_opacity: None,
            stroke: None,
            stroke_width: None,
            stroke_linecap: None,
            stroke_linejoin: None,
            stroke_miterlimit: None,
            stroke_dasharray: None,
            stroke_dashoffset: None,
            translate: None,
            rotate: None,
            scale_all: None,
            scale_xy: None,
            skew_x: None,
            skew_y: None,
            fill_gradient: None,
        }
    }

    pub fn format(&self) -> String {
        let mut transforms = vec![];

        if self.fill_gradient.is_some() {
            transforms.push(format!(
                "fill=\"url(#{})\"",
                self.fill_gradient.as_ref().unwrap()
            ));
        } else {
            if self.fill.is_some() {
                transforms.push(format!("fill=\"{}\"", self.fill.as_ref().unwrap()));
            } else {
                transforms.push("fill=\"none\"".to_string());
            }
        }

        if self.fill_rule.is_some() {
            transforms.push(format!(
                "fill-rule=\"{}\"",
                self.fill_rule.as_ref().unwrap()
            ));
        }

        if self.fill_opacity.is_some() {
            transforms.push(format!(
                "fill-opacity=\"{}\"",
                self.fill_opacity.as_ref().unwrap()
            ));
        }

        if self.stroke_width.is_some() {
            transforms.push(format!(
                "stroke-width=\"{}\"",
                self.stroke_width.as_ref().unwrap()
            ));
        }

        if self.stroke.is_some() {
            transforms.push(format!("stroke=\"{}\"", self.stroke.as_ref().unwrap()));
        }

        if self.stroke_linejoin.is_some() {
            transforms.push(format!(
                "stroke-linejoin=\"{}\"",
                self.stroke_linejoin.as_ref().unwrap()
            ));
        }

        if self.stroke_linecap.is_some() {
            transforms.push(format!(
                "stroke-linecap=\"{}\"",
                self.stroke_linecap.as_ref().unwrap()
            ));
        }

        if self.stroke_miterlimit.is_some() {
            transforms.push(format!(
                "stroke-miterlimit=\"{}\"",
                self.stroke_miterlimit.as_ref().unwrap()
            ));
        }

        if self.stroke_dasharray.is_some() {
            transforms.push(format!(
                "stroke-dasharray=\"{}\"",
                self.stroke_dasharray.as_ref().unwrap()
            ));
        }

        if self.stroke_dashoffset.is_some() {
            transforms.push(format!(
                "stroke-dashoffset=\"{}\"",
                self.stroke_dashoffset.as_ref().unwrap()
            ));
        }

        if self.translate.is_some()
            || self.rotate.is_some()
            || self.scale_all.is_some()
            || self.scale_xy.is_some()
            || self.skew_x.is_some()
            || self.skew_y.is_some()
        {
            let mut translates = vec![];

            if self.translate.is_some() {
                translates.push(format!(
                    "translate({} {})",
                    self.translate.as_ref().unwrap().0,
                    self.translate.as_ref().unwrap().1
                ));
            }

            if self.rotate.is_some() {
                translates.push(format!("rotate({})", self.rotate.as_ref().unwrap()));
            }

            if self.scale_all.is_some() || self.scale_xy.is_some() {
                if self.scale_all.is_some() {
                    translates.push(format!("scale({})", self.scale_all.as_ref().unwrap()));
                } else {
                    translates.push(format!(
                        "scale({} {})",
                        self.scale_xy.as_ref().unwrap().0,
                        self.scale_xy.as_ref().unwrap().1
                    ));
                }
            }

            if self.skew_x.is_some() {
                translates.push(format!("skewX({})", self.skew_x.as_ref().unwrap()));
            }

            if self.skew_y.is_some() {
                translates.push(format!("skewY({})", self.skew_y.as_ref().unwrap()));
            }

            transforms.push(format!("transform=\"{}\"", translates.join(" ")));
        }

        transforms.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_format_fill1() {
        let mut sstyle = Sstyle::new();

        sstyle.fill_rule = Some(FillRule::Nonzero);
        sstyle.fill_gradient = Some("s1".to_string());
        sstyle.fill_opacity = Some(0.5);

        assert_eq!(
            sstyle.format(),
            "fill=\"url(#s1)\" fill-rule=\"nonzero\" fill-opacity=\"0.5\""
        );
    }

    #[test]
    fn check_format_fill2() {
        let mut sstyle = Sstyle::new();

        sstyle.fill = Some("red".to_string());
        sstyle.fill_rule = Some(FillRule::Nonzero);
        sstyle.fill_opacity = Some(30.0);

        assert_eq!(
            sstyle.format(),
            "fill=\"red\" fill-rule=\"nonzero\" fill-opacity=\"30\""
        );
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
            "fill=\"none\" stroke-width=\"5\" stroke=\"#ABABAB\" stroke-linejoin=\"round\" stroke-linecap=\"square\" stroke-miterlimit=\"2\" stroke-dasharray=\"40,10\" stroke-dashoffset=\"5\""
        );
    }

    #[test]
    fn check_format_transform1() {
        let mut sstyle = Sstyle::new();

        sstyle.translate = Some((0.1, 0.2));
        sstyle.rotate = Some(30.0);
        sstyle.scale_all = Some(1.0);
        sstyle.skew_x = Some(2.0);
        sstyle.skew_y = Some(3.0);

        assert_eq!(
            sstyle.format(),
            "fill=\"none\" transform=\"translate(0.1 0.2) rotate(30) scale(1) skewX(2) skewY(3)\""
        );
    }

    #[test]
    fn check_format_transform2() {
        let mut sstyle = Sstyle::new();

        sstyle.scale_xy = Some((2.0, 3.0));

        assert_eq!(sstyle.format(), "fill=\"none\" transform=\"scale(2 3)\"");
    }
}
