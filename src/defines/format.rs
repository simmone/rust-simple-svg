pub trait Format {
    fn format(&self, shape_id: String) -> String;
}
