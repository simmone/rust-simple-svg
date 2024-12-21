use crate::defines::sstyle::Sstyle;

#[derive(Default)]
pub struct Widget<'a> {
    pub shape_id: String,
    pub at: Option<(f64, f64)>,
    pub style: Option<&'a Sstyle>,
    pub filter_id: Option<String>,
    pub marker_start_id: Option<String>,
    pub marker_mid_id: Option<String>,
    pub marker_end_id: Option<String>,
}

impl<'a> Widget<'a> {
    pub fn format(&self) -> String {
        let mut fmt_str = String::new();
        
        fmt_str.push_str(&format!("<use xlink:href=\"#{}\" ", self.shape_id));
        
        fmt_str.push_str("/>");
        
        fmt_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_format() {
        let widget1 = Widget { shape_id: "s1".to_string(), ..Default::default() };
        
        assert_eq!(widget1.format(), "<use xlink:href=\"#s1\" />");
    }
}
