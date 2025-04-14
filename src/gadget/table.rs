#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub start_point: (f64, f64),
    pub width: f64,
    pub height: f64,
    pub color: String,
    pub text: String,
    pub font_size: f64,
    pub font_color: String,
    pub margin_top: f64,
    pub margin_left: f64,
}

fn get_cells(matrix: &Vec<[&str; 2]>) -> Vec<(usize, usize, String)> {
    let row_count = matrix.len();
    let col_count = matrix[0].len();

    let mut axis_data_array = vec![];

    for row in 0..row_count {
        for col in 0..col_count {
            axis_data_array.push((row, col, matrix[row][col].to_string()));
        }
    }

    axis_data_array
}

fn matrix_to_cells(
    matrix: &Vec<[&str; 2]>,
    col_width: f64,
    row_height: f64,
    color: &str,
    cell_margin_top: f64,
    cell_margin_left: f64,
    start_point: (f64, f64),
    font_size: f64,
    font_color: &str,
) -> Vec<Cell> {
    let axis_data_array = get_cells(matrix);

    let mut cells = vec![];

    let mut loop_point = start_point;

    for (index, axis_data) in axis_data_array.iter().enumerate() {
        let row_index = axis_data.0;
        let col_index = axis_data.1;
        let text = axis_data.2.clone();

        cells.push(Cell {
            start_point: loop_point,
            width: col_width,
            height: row_height,
            color: color.to_string(),
            text,
            font_size,
            font_color: font_color.to_string(),
            margin_top: cell_margin_top,
            margin_left: cell_margin_left,
        });

        if index < axis_data_array.len() - 1 {
            if row_index == axis_data_array[index + 1].0 {
                loop_point = (loop_point.0 + col_width, loop_point.1);
            } else {
                loop_point = (start_point.0, loop_point.1 + row_height);
            }
        }
    }

    cells
}

//(define (matrix-to-cells matrix col_width row_height color cell_margin_top cell_margin_left start_point font_size font_color)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_get_cells() {
        let cells = get_cells(&vec![["1", "2"], ["3", "4"]]);

        assert_eq!(cells.len(), 4);

        assert_eq!(
            cells,
            vec![
                (0, 0, "1".to_string()),
                (0, 1, "2".to_string()),
                (1, 0, "3".to_string()),
                (1, 1, "4".to_string())
            ]
        );
    }

    #[test]
    fn check_matrix_to_cells() {
        let cells = matrix_to_cells(
            &vec![["1", "2"], ["3", "4"]],
            5.0,
            5.0,
            "black",
            1.0,
            1.0,
            (0.0, 0.0),
            10.0,
            "red",
        );

        assert_eq!(cells.len(), 4);

        assert_eq!(
            cells,
            vec![
                Cell {
                    start_point: (0.0, 0.0),
                    width: 5.0,
                    height: 5.0,
                    color: "black".to_string(),
                    text: "1".to_string(),
                    font_size: 10.0,
                    font_color: "red".to_string(),
                    margin_top: 1.0,
                    margin_left: 1.0
                },
                Cell {
                    start_point: (5.0, 0.0),
                    width: 5.0,
                    height: 5.0,
                    color: "black".to_string(),
                    text: "2".to_string(),
                    font_size: 10.0,
                    font_color: "red".to_string(),
                    margin_top: 1.0,
                    margin_left: 1.0
                },
                Cell {
                    start_point: (0.0, 5.0),
                    width: 5.0,
                    height: 5.0,
                    color: "black".to_string(),
                    text: "3".to_string(),
                    font_size: 10.0,
                    font_color: "red".to_string(),
                    margin_top: 1.0,
                    margin_left: 1.0
                },
                Cell {
                    start_point: (5.0, 5.0),
                    width: 5.0,
                    height: 5.0,
                    color: "black".to_string(),
                    text: "4".to_string(),
                    font_size: 10.0,
                    font_color: "red".to_string(),
                    margin_top: 1.0,
                    margin_left: 1.0
                }
            ]
        );
    }
}
