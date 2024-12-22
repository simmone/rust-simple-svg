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
        let mut format_items = vec![];

        format_items.push(format!("<use xlink:href=\"#{}\"", self.shape_id));
        
        if self.style.is_some() {
            format_items.push(self.style.unwrap().format());
        }
        
        format_items.push("/>".to_string());
        
        format_items.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_format1() {
        let widget = Widget { shape_id: "s1".to_string(), ..Default::default() };
        
        assert_eq!(widget.format(), "<use xlink:href=\"#s1\" />");
    }

    #[test]
    fn check_format2() {
        let mut rect_sstyle = Sstyle::new();
        rect_sstyle.fill = Some("#BBC42A".to_string());

        let widget = Widget { shape_id: "s1".to_string(), style: Some(&rect_sstyle), ..Default::default() };
        
        assert_eq!(widget.format(), "<use xlink:href=\"#s1\" fill=\"#BBC42A\" />");
    }

}
