use std::fmt;

#[derive(Clone)]
pub enum TextKerning {
    Num(f64),
    Auto,
    Inerit,
}

impl fmt::Display for TextKerning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TextKerning::Num(n) => write!(f, "{}", n),
            TextKerning::Auto => write!(f, "auto"),
            TextKerning::Inerit => write!(f, "inerit"),
        }
    }
}

#[derive(Clone)]
pub enum TextSpace {
    Num(f64),
    Normal,
    Inerit,
}

impl fmt::Display for TextSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TextSpace::Num(n) => write!(f, "{}", n),
            TextSpace::Normal => write!(f, "normal"),
            TextSpace::Inerit => write!(f, "inerit"),
        }
    }
}

#[derive(Clone)]
pub enum TextDecoration {
    OverLine,
    UnderLine,
    LineThrough,
}

impl fmt::Display for TextDecoration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TextDecoration::OverLine => write!(f, "overline"),
            TextDecoration::UnderLine => write!(f, "underline"),
            TextDecoration::LineThrough => write!(f, "line-through"),
        }
    }
}

#[derive(Clone)]
pub struct Text {
    pub text: String,
    pub font_size: Option<f64>,
    pub font_family: Option<String>,
    pub dx: Option<f64>,
    pub dy: Option<f64>,
    pub rotate: Option<Vec<f64>>,
    pub text_length: Option<f64>,
    pub kerning: Option<TextKerning>,
    pub letter_space: Option<TextSpace>,
    pub word_space: Option<TextSpace>,
    pub text_decoration: Option<TextDecoration>,
    pub path: Option<String>,
    pub path_start_offset: Option<f64>,
}

impl Text {
    pub fn new(text: String) -> Self {
        Text {
            text,
            font_size: None,
            font_family: None,
            dx: None,
            dy: None,
            rotate: None,
            text_length: None,
            kerning: None,
            letter_space: None,
            word_space: None,
            text_decoration: None,
            path: None,
            path_start_offset: None
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
        let mut text = Text::new();

        text.fill_rule = Some(FillRule::Nonzero);
        text.fill_gradient = Some("s1".to_string());
        text.fill_opacity = Some(0.5);

        assert_eq!(
            text.format(),
            "fill=\"url(#s1)\" fill-rule=\"nonzero\" fill-opacity=\"0.5\""
        );
    }

    #[test]
    fn check_format_fill2() {
        let mut text = Text::new();

        text.fill = Some("red".to_string());
        text.fill_rule = Some(FillRule::Nonzero);
        text.fill_opacity = Some(30.0);

        assert_eq!(
            text.format(),
            "fill=\"red\" fill-rule=\"nonzero\" fill-opacity=\"30\""
        );
    }

    #[test]
    fn check_format_stroke() {
        let mut text = Text::new();

        text.stroke_width = Some(5.0);
        text.stroke = Some("#ABABAB".to_string());
        text.stroke_linejoin = Some(LineJoin::Round);
        text.stroke_linecap = Some(LineCap::Square);
        text.stroke_miterlimit = Some(2.0);
        text.stroke_dasharray = Some("40,10".to_string());
        text.stroke_dashoffset = Some(5.0);

        assert_eq!(
            Text::format(&text),
            "fill=\"none\" stroke-width=\"5\" stroke=\"#ABABAB\" stroke-linejoin=\"round\" stroke-linecap=\"square\" stroke-miterlimit=\"2\" stroke-dasharray=\"40,10\" stroke-dashoffset=\"5\""
        );
    }

    #[test]
    fn check_format_transform1() {
        let mut text = Text::new();

        text.translate = Some((0.1, 0.2));
        text.rotate = Some(30.0);
        text.scale_all = Some(1.0);
        text.skew_x = Some(2.0);
        text.skew_y = Some(3.0);

        assert_eq!(
            text.format(),
            "fill=\"none\" transform=\"translate(0.1 0.2) rotate(30) scale(1) skewX(2) skewY(3)\""
        );

        text.scale_all = None;
        text.scale_xy = Some((2.0, 3.0));

        assert_eq!(
            text.format(),
            "fill=\"none\" transform=\"translate(0.1 0.2) rotate(30) scale(2 3) skewX(2) skewY(3)\""
        );
    }
}
