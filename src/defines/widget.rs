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
