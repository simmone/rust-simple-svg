#![doc = include_str!("TABLE.md")]

use std::collections::HashMap;

use crate::define::group::Group;
use crate::define::shape::rect::Rect;
use crate::define::shape::text::Text;
use crate::define::shape::Shape;
use crate::define::sstyle::Sstyle;
use crate::define::svg::Svg;
use crate::define::widget::Widget;

#[derive(Debug, Clone, PartialEq, Default)]
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

#[derive(Default)]
pub struct Table {
    pub cells: Vec<Cell>,
    pub col_width: f64,
    pub row_height: f64,
    pub color: String,
    pub font_size: f64,
    pub font_color: String,
    pub cell_margin_top: f64,
    pub cell_margin_left: f64,
    row_height_map: HashMap<usize, f64>,
    col_width_map: HashMap<usize, f64>,
    col_margin_left_map: HashMap<usize, f64>,
    row_margin_top_map: HashMap<usize, f64>,
    cell_font_size_map: HashMap<(usize, usize), f64>,
    cell_font_color_map: HashMap<(usize, usize), String>,
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
            row_height_map: HashMap::new(),
            col_width_map: HashMap::new(),
            col_margin_left_map: HashMap::new(),
            row_margin_top_map: HashMap::new(),
            cell_font_size_map: HashMap::new(),
            cell_font_color_map: HashMap::new(),
        }
    }

    fn get_cells(matrix: &Vec<Vec<&str>>) -> Vec<(usize, usize, String)> {
        let mut axis_data_array = vec![];

        for (row_index, row) in matrix.iter().enumerate() {
            for (element_index, element) in row.iter().enumerate() {
                axis_data_array.push((row_index, element_index, element.to_string()));
            }
        }

        axis_data_array
    }

    fn matrix_to_cells(
        &self,
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

            let mut col_real_width = col_width;
            if self.col_width_map.contains_key(&col_index) {
                col_real_width = *self.col_width_map.get(&col_index).unwrap()
            }

            let mut row_real_height = row_height;
            if self.row_height_map.contains_key(&row_index) {
                row_real_height = *self.row_height_map.get(&row_index).unwrap();
            }

            let mut cell_real_margin_top = cell_margin_top;
            if self.row_margin_top_map.contains_key(&row_index) {
                cell_real_margin_top = *self.row_margin_top_map.get(&row_index).unwrap();
            }

            let mut cell_real_margin_left = cell_margin_left;
            if self.col_margin_left_map.contains_key(&col_index) {
                cell_real_margin_left = *self.col_margin_left_map.get(&col_index).unwrap();
            }

            let mut font_real_size = font_size;
            if self
                .cell_font_size_map
                .contains_key(&(axis_data.0, axis_data.1))
            {
                font_real_size = *self
                    .cell_font_size_map
                    .get(&(axis_data.0, axis_data.1))
                    .unwrap()
            }

            let mut font_real_color = font_color.to_string();
            if self
                .cell_font_color_map
                .contains_key(&(axis_data.0, axis_data.1))
            {
                font_real_color = self
                    .cell_font_color_map
                    .get(&(axis_data.0, axis_data.1))
                    .unwrap()
                    .clone();
            }

            cells.push(Cell {
                start_point: loop_point,
                width: col_real_width,
                height: row_real_height,
                color: color.to_string(),
                text,
                font_size: font_real_size,
                font_color: font_real_color,
                margin_top: cell_real_margin_top,
                margin_left: cell_real_margin_left,
            });

            if index < axis_data_array.len() - 1 {
                if row_index == axis_data_array[index + 1].0 {
                    loop_point = (loop_point.0 + col_real_width, loop_point.1);
                } else {
                    loop_point = (0.0, loop_point.1 + row_real_height);
                }
            }
        }

        cells
    }

    pub fn to_group(&self, svg: &mut Svg, matrix: &Vec<Vec<&str>>) -> Group {
        let cells = self.matrix_to_cells(
            matrix,
            self.col_width,
            self.row_height,
            &self.color,
            self.cell_margin_top,
            self.cell_margin_left,
            self.font_size,
            &self.font_color,
        );

        let mut group = Group::default();

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

    pub fn set_table_col_width(&mut self, cols: Vec<usize>, width: f64) {
        for col in cols {
            self.col_width_map.insert(col, width);
        }
    }

    pub fn set_table_row_height(&mut self, rows: Vec<usize>, height: f64) {
        for row in rows {
            self.row_height_map.insert(row, height);
        }
    }

    pub fn set_table_col_margin_left(&mut self, cols: Vec<usize>, margin: f64) {
        for col in cols {
            self.col_margin_left_map.insert(col, margin);
        }
    }

    pub fn set_table_row_margin_top(&mut self, rows: Vec<usize>, margin: f64) {
        for row in rows {
            self.row_margin_top_map.insert(row, margin);
        }
    }

    pub fn set_table_cell_font_size(&mut self, axises: Vec<(usize, usize)>, font_size: f64) {
        for axis in axises {
            self.cell_font_size_map.insert(axis, font_size);
        }
    }

    pub fn set_table_cell_font_color(&mut self, axises: Vec<(usize, usize)>, font_color: String) {
        for axis in axises {
            self.cell_font_color_map.insert(axis, font_color.clone());
        }
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
        let table = Table::new();

        let cells = Table::matrix_to_cells(
            &table,
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
