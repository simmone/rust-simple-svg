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

fn get_cells<'a>(matrix: &Vec<[&'a str; 2]>) -> Vec<(usize, usize, &'a str)> {
    let row_count = matrix.len();
    let col_count = matrix[0].len();
    
    let mut axis_data_array = vec![];
    
    for row in 0..row_count {
        for col in 0..col_count {
            axis_data_array.push((row, col, matrix[row][col]));
        }
    }

    axis_data_array
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_get_cells() {
        let cells = get_cells(&vec![["1", "2"], ["3", "4"]]);
        
        assert_eq!(cells.len(), 4);
        
        assert_eq!(cells, vec![(0, 0, "1"), (0, 1, "2"), (1, 0, "3"), (1, 1, "4")]);
    }
}
