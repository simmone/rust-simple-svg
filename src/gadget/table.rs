pub use crate::define::group::Group;
pub use crate::define::shape::rect::Rect;
pub use crate::define::shape::text::Text;
pub use crate::define::shape::Shape;
pub use crate::define::sstyle::Sstyle;
pub use crate::define::svg::Svg;
pub use crate::define::widget::Widget;

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

pub struct Table {
    pub cells: Vec<Cell>,
    pub col_width: f64,
    pub row_height: f64,
    pub color: String,
    pub font_size: f64,
    pub font_color: String,
    pub cell_margin_top: f64,
    pub cell_margin_left: f64,
}

impl Table {
    pub fn new() -> Self {
        Table {
            cells: vec![],
            col_width: 50.0,
            row_height: 30.0,
            color: "black".to_string(),
            cell_margin_top: 22.0,
            cell_margin_left: 20.0,
            font_size: 20.0,
            font_color: "black".to_string(),
        }
    }

    fn get_cells(matrix: &Vec<Vec<&str>>) -> Vec<(usize, usize, String)> {
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
        matrix: &Vec<Vec<&str>>,
        col_width: f64,
        row_height: f64,
        color: &str,
        cell_margin_top: f64,
        cell_margin_left: f64,
        font_size: f64,
        font_color: &str,
    ) -> Vec<Cell> {
        let axis_data_array = Table::get_cells(matrix);

        let mut cells = vec![];

        let mut loop_point = (0.0, 0.0);

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
                    loop_point = (0.0, loop_point.1 + row_height);
                }
            }
        }

        cells
    }

    pub fn to_group(&self, svg: &mut Svg, matrix: &Vec<Vec<&str>>) -> Group {
        let cells = Self::matrix_to_cells(
            matrix,
            self.col_width,
            self.row_height,
            &self.color,
            self.cell_margin_top,
            self.cell_margin_left,
            self.font_size,
            &self.font_color,
        );

        let mut group = Group::new();

        for cell in cells {
            let rect_id = svg.add_shape(Shape::Rect(Rect::new(cell.width, cell.height)));

            let mut rect_sstyle = Sstyle::new();
            rect_sstyle.stroke = Some(cell.color);

            group.place_widget(Widget {
                shape_id: rect_id,
                style: Some(rect_sstyle),
                at: Some(cell.start_point),
                ..Default::default()
            });

            let mut text = Text::new(cell.text);
            text.font_size = Some(cell.font_size);
            let text_id = svg.add_shape(Shape::Text(text));

            let mut text_sstyle = Sstyle::new();
            text_sstyle.fill = Some(cell.font_color.clone());
            text_sstyle.stroke = Some(cell.font_color.clone());

            group.place_widget(Widget {
                shape_id: text_id,
                style: Some(text_sstyle),
                at: Some((
                    cell.start_point.0 + cell.margin_left,
                    cell.start_point.1 + cell.margin_top,
                )),
                ..Default::default()
            });
        }

        group
    }
    
    pub fn set_table_col_width(&self, cols: &Vec<usize>, width: f64) {
    }
    
    pub fn set_table_row_height(&self, rows: &Vec<usize>, height: f64) {
    }
    
    pub fn set_table_col_margin_left(&self, cols: &Vec<usize>, margin: f64) {
    }
    
    pub fn set_table_row_margin_top(&self, rows: &Vec<usize>, margin: f64) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_get_cells() {
        let cells = Table::get_cells(&vec![vec!["1", "2"], vec!["3", "4"]]);

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
        let cells = Table::matrix_to_cells(
            &vec![vec!["1", "2"], vec!["3", "4"]],
            5.0,
            5.0,
            "black",
            1.0,
            1.0,
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
