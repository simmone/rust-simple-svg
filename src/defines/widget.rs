use crate::defines::sstyle::Sstyle;

pub struct Widget {
    pub id: String,
    pub at: Option<(f64, f64)>,
    pub style: Option<Sstyle>,
    pub filter_id: Option<String>,
    pub marker_start_id: Option<String>,
    pub marker_mid_id: Option<String>,
    pub marker_end_id: Option<String>,
}

impl Widget {
    pub fn new(id: String) -> Self {
        Widget {
            id,
            at: None,
            style: None,
            filter_id: None,
            marker_start_id: None,
            marker_mid_id: None,
            marker_end_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_new() {
        let widget = Widget::new("s1".to_string());
        
        assert_eq!(widget.id, "s1");
    }
}
