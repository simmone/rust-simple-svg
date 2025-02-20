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

    pub fn format(&self, shape_id: String) -> String {
        format!(
            "    <text id=\"{}\"{}>{}</text>\n",
            shape_id,
            {
                let mut options = vec![];

                if self.dx.is_some() {
                    options.push(format!(" dx=\"{}\"", self.dx.as_ref().unwrap()));
                }

                if self.dy.is_some() {
                    options.push(format!(" dy=\"{}\"", self.dy.as_ref().unwrap()));
                }

                if self.font_size.is_some() {
                    options.push(format!(" font-size=\"{}\"", self.font_size.as_ref().unwrap()));
                }

                if self.font_family.is_some() {
                    options.push(format!(" font-family=\"{}\"", self.font_family.as_ref().unwrap()));
                }
                
                if self.rotate.is_some() {
                    let mut rotate_items = vec![];
                    
                    for degree in self.rotate.as_ref().unwrap() {
                        rotate_items.push(format!("{}", degree));
                    }
                    
                    options.push(rotate_items.join(" "));
                }

                if self.text_length.is_some() {
                    options.push(format!(" textLength=\"{}\"", self.text_length.as_ref().unwrap()));
                }

                if self.kerning.is_some() {
                    options.push(format!(" kerning=\"{}\"", self.kerning.as_ref().unwrap()));
                }

                if self.letter_space.is_some() {
                    options.push(format!(" letter-space=\"{}\"", self.letter_space.as_ref().unwrap()));
                }

                if self.word_space.is_some() {
                    options.push(format!(" word-space=\"{}\"", self.word_space.as_ref().unwrap()));
                }

                if self.text_decoration.is_some() {
                    options.push(format!(" text-decoration=\"{}\"", self.text_decoration.as_ref().unwrap()));
                }

                options.join(" ")
            },
            self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_format() {
        let text = Text::new("1".to_string());

        assert_eq!(
            text.format("s1".to_string()),
            "fill=\"none\" transform=\"translate(0.1 0.2) rotate(30) scale(1) skewX(2) skewY(3)\""
        );
    }
}
