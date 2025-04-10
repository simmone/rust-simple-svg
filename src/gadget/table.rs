#[derive(Debug, Clone)]
pub struct Cell {
    pub start_point: (f64, f64),
    pub width: f64,
    pub height: f64,
    pub color: String,
    pub text: String,
    pub font_size: f64,
    pub font_color: String,
    pub margin_top: f64,
    pub margin_left: f64
}

fn get_cells(matrix: &Vec<Vec<String>>) -> (Vec<(usize, usize)>, Vec<String>) {
}
