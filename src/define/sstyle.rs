//! Sstyle used to set shape/group style, it includes multiple effect.
//! Please check Joni's tutorial: [SVG Pocket Guide](http://svgpocketguide.com/)

use std::fmt;

use crate::tools::precision::svg_round;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug, Default)]
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
    pub precision: usize,
}

impl Sstyle {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn format(&self) -> String {
        let mut transforms = vec![];

        if let Some(fill_gradient) = &self.fill_gradient {
            transforms.push(format!(
                "fill=\"url(#{})\"",
                fill_gradient
            ));
        } else if let Some(fill) = &self.fill {
            transforms.push(format!("fill=\"{}\"", fill));
        } else {
            transforms.push("fill=\"none\"".to_string());
        }

        if let Some(fill_rule) = &self.fill_rule {
            transforms.push(format!(
                "fill-rule=\"{}\"",
                fill_rule
            ));
        }

        if let Some(fill_opacity) = self.fill_opacity {
            transforms.push(format!(
                "fill-opacity=\"{}\"",
                svg_round(fill_opacity, self.precision)
            ));
        }

        if let Some(stroke_width) = self.stroke_width {
            transforms.push(format!(
                "stroke-width=\"{}\"",
                svg_round(stroke_width, self.precision)
            ));
        }

        if let Some(stroke) = &self.stroke {
            transforms.push(format!("stroke=\"{}\"", stroke));
        }

        if let Some(stroke_linejoin) = &self.stroke_linejoin {
            transforms.push(format!(
                "stroke-linejoin=\"{}\"",
                stroke_linejoin
            ));
        }

        if let Some(stroke_linecap)= &self.stroke_linecap {
            transforms.push(format!(
                "stroke-linecap=\"{}\"",
                stroke_linecap
            ));
        }

        if let Some(stroke_miterlimit) = self.stroke_miterlimit {
            transforms.push(format!(
                "stroke-miterlimit=\"{}\"",
                svg_round(stroke_miterlimit, self.precision)
            ));
        }

        if let Some(stroke_dasharray) = &self.stroke_dasharray {
            transforms.push(format!("stroke-dasharray=\"{}\"", stroke_dasharray));
        }

        if let Some(stroke_dashoffset) = self.stroke_dashoffset {
            transforms.push(format!(
                "stroke-dashoffset=\"{}\"",
                svg_round(stroke_dashoffset, self.precision)
            ));
        }

        let mut translates = vec![];

        if let Some(translate) = self.translate {
            translates.push(format!(
                "translate({} {})",
                svg_round(translate.0, self.precision),
                svg_round(translate.1, self.precision)
            ));
        }

        if let Some(rotate) = self.rotate {
            translates.push(format!("rotate({})", svg_round(rotate, self.precision)));
        }

        if let Some(scale_all) = self.scale_all {
            translates.push(format!("scale({})", svg_round(scale_all, self.precision)));
        }

        if let Some(scale_xy) = self.scale_xy {
            translates.push(format!(
                "scale({} {})",
                svg_round(scale_xy.0, self.precision),
                svg_round(scale_xy.1, self.precision)
            ));
        }

        if let Some(skew_x) = self.skew_x {
            translates.push(format!("skewX({})", svg_round(skew_x, self.precision)));
        }

        if let Some(skew_y) = self.skew_y {
            translates.push(format!("skewY({})", svg_round(skew_y, self.precision)));
        }

        if !translates.is_empty() {
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
        sstyle.fill_opacity = Some(0.51);

        sstyle.precision = 1;
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
        sstyle.fill_opacity = Some(30.01);

        sstyle.precision = 1;
        assert_eq!(
            sstyle.format(),
            "fill=\"red\" fill-rule=\"nonzero\" fill-opacity=\"30\""
        );
    }

    #[test]
    fn check_format_stroke() {
        let mut sstyle = Sstyle::new();

        sstyle.stroke_width = Some(5.01);
        sstyle.stroke = Some("#ABABAB".to_string());
        sstyle.stroke_linejoin = Some(LineJoin::Round);
        sstyle.stroke_linecap = Some(LineCap::Square);
        sstyle.stroke_miterlimit = Some(2.01);
        sstyle.stroke_dasharray = Some("40,10".to_string());
        sstyle.stroke_dashoffset = Some(5.01);

        sstyle.precision = 1;
        assert_eq!(
            Sstyle::format(&sstyle),
            "fill=\"none\" stroke-width=\"5\" stroke=\"#ABABAB\" stroke-linejoin=\"round\" stroke-linecap=\"square\" stroke-miterlimit=\"2\" stroke-dasharray=\"40,10\" stroke-dashoffset=\"5\""
        );
    }

    #[test]
    fn check_format_transform1() {
        let mut sstyle = Sstyle::new();

        sstyle.translate = Some((0.11, 0.21));
        sstyle.rotate = Some(30.01);
        sstyle.scale_all = Some(1.01);
        sstyle.skew_x = Some(2.01);
        sstyle.skew_y = Some(3.01);

        sstyle.precision = 1;
        assert_eq!(
            sstyle.format(),
            "fill=\"none\" transform=\"translate(0.1 0.2) rotate(30) scale(1) skewX(2) skewY(3)\""
        );

        sstyle.scale_all = None;
        sstyle.scale_xy = Some((2.01, 3.01));

        assert_eq!(
            sstyle.format(),
            "fill=\"none\" transform=\"translate(0.1 0.2) rotate(30) scale(2 3) skewX(2) skewY(3)\""
        );
    }
}
