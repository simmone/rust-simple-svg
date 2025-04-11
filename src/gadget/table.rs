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
    (vec![(0, 0)], vec!["1".to_string()])
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_get_cells() {
        let cells = get_cells(vec![["1", "2"], ["3"]]);
        
        assert_eq!(cells.0.length, 4);
        
        assert_eq!(cells.1.length, 4);
    }
}
