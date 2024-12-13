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

#[cfg(test)]
mod tests {
    
}
