use std::fmt;

#[derive(Clone)]
pub enum TextKerning {
    Num(f64),
    Auto,
    Inherit,
}

impl fmt::Display for TextKerning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TextKerning::Num(n) => write!(f, "{}", n),
            TextKerning::Auto => write!(f, "auto"),
            TextKerning::Inherit => write!(f, "inherit"),
        }
    }
}

#[derive(Clone)]
pub enum TextSpace {
    Num(f64),
    Normal,
    Inherit,
}

impl fmt::Display for TextSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TextSpace::Num(n) => write!(f, "{}", n),
            TextSpace::Normal => write!(f, "normal"),
            TextSpace::Inherit => write!(f, "inherit"),
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
            path_start_offset: None,
        }
    }

    pub fn format(&self, shape_id: String) -> String {
        format!(
            "    <text id=\"{}\"{}>{}</text>\n",
            shape_id,
            {
                let mut options_str = String::new();

                if self.dx.is_some() {
                    options_str.push_str(&format!(" dx=\"{}\"", self.dx.as_ref().unwrap()));
                }

                if self.dy.is_some() {
                    options_str.push_str(&format!(" dy=\"{}\"", self.dy.as_ref().unwrap()));
                }

                if self.font_size.is_some() {
                    options_str.push_str(&format!(
                        " font-size=\"{}\"",
                        self.font_size.as_ref().unwrap()
                    ));
                }

                if self.font_family.is_some() {
                    options_str.push_str(&format!(
                        " font-family=\"{}\"",
                        self.font_family.as_ref().unwrap()
                    ));
                }

                if self.rotate.is_some() {
                    options_str.push_str(&format!(" rotate=\"{}\"", {
                        let mut rotate_items = vec![];

                        for degree in self.rotate.as_ref().unwrap() {
                            rotate_items.push(format!("{}", degree));
                        }

                        rotate_items.join(" ")
                    }));
                }

                if self.text_length.is_some() {
                    options_str.push_str(&format!(
                        " textLength=\"{}\"",
                        self.text_length.as_ref().unwrap()
                    ));
                }

                if self.kerning.is_some() {
                    options_str
                        .push_str(&format!(" kerning=\"{}\"", self.kerning.as_ref().unwrap()));
                }

                if self.letter_space.is_some() {
                    options_str.push_str(&format!(
                        " letter-space=\"{}\"",
                        self.letter_space.as_ref().unwrap()
                    ));
                }

                if self.word_space.is_some() {
                    options_str.push_str(&format!(
                        " word-space=\"{}\"",
                        self.word_space.as_ref().unwrap()
                    ));
                }

                if self.text_decoration.is_some() {
                    options_str.push_str(&format!(
                        " text-decoration=\"{}\"",
                        self.text_decoration.as_ref().unwrap()
                    ));
                }

                options_str
            },
            {
                if self.path.is_some() {
                    let mut text_str = String::new();

                    text_str.push_str(&format!(
                        "\n      <textPath xlink:href=\"#{}\" ",
                        self.path.as_ref().unwrap()
                    ));

                    if self.path_start_offset.is_some() {
                        text_str.push_str(&format!(
                            "startOffset=\"{}%\" ",
                            self.path_start_offset.as_ref().unwrap()
                        ));
                    }

                    text_str.push_str(&format!(">{}</textPath>\n    ", self.text));

                    text_str
                } else {
                    self.text.clone()
                }
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_format1() {
        let mut text = Text::new("hello world".to_string());
        text.font_size = Some(1.0);
        text.font_family = Some("Arial".to_string());
        text.dx = Some(2.0);
        text.dy = Some(3.0);
        text.rotate = Some(vec![4.0, 5.0, 6.0, 7.0]);
        text.text_length = Some(8.0);
        text.kerning = Some(TextKerning::Auto);
        text.letter_space = Some(TextSpace::Normal);
        text.word_space = Some(TextSpace::Inherit);
        text.text_decoration = Some(TextDecoration::UnderLine);

        assert_eq!(
            text.format("s1".to_string()),
            "    <text id=\"s1\" dx=\"2\" dy=\"3\" font-size=\"1\" font-family=\"Arial\" rotate=\"4 5 6 7\" textLength=\"8\" kerning=\"auto\" letter-space=\"normal\" word-space=\"inherit\" text-decoration=\"underline\">hello world</text>\n");

        text.path = Some("9.0, 10.0".to_string());
        text.path_start_offset = Some(11.0);
        assert_eq!(
            text.format("s1".to_string()),
            "    <text id=\"s1\" dx=\"2\" dy=\"3\" font-size=\"1\" font-family=\"Arial\" rotate=\"4 5 6 7\" textLength=\"8\" kerning=\"auto\" letter-space=\"normal\" word-space=\"inherit\" text-decoration=\"underline\">\n      <textPath xlink:href=\"#9.0, 10.0\" startOffset=\"11%\" >hello world</textPath>\n    </text>\n");
    }
}
