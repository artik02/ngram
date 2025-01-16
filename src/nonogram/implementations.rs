// MIT LICENSE
//
// Copyright 2024 artik02
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the “Software”), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is furnished to do
// so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/// Imports definitions for Nonogram puzzle components and background.
use super::definitions::{
    NonogramPalette, NonogramPuzzle, NonogramSegment, NonogramSolution, BACKGROUND,
};
/// Import macro to construct nonogram rules easily
use crate::nrule;

impl NonogramPuzzle {
    /// Creates a new `NonogramPuzzle` from a given `NonogramSolution`.
    ///
    /// # Arguments
    ///
    /// * `solution` - A reference to a `NonogramSolution` object containing the solution details.
    ///
    /// # Returns
    ///
    /// A new instance of `NonogramPuzzle` initialized with the provided solution's constraints.
    // TODO: Delete non-used colors.
    pub fn from_solution(solution: &NonogramSolution) -> Self {
        let rows = solution.rows();
        let cols = solution.cols();
        let row_constraints = solution.row_constraints();
        let col_constraints = solution.col_constraints();
        Self {
            rows,
            cols,
            row_constraints,
            col_constraints,
        }
    }
}

impl NonogramSolution {
    /// Returns the number of rows in the nonogram solution.
    pub fn rows(&self) -> usize {
        self.solution_grid.len()
    }

    /// Returns the number of columns in the nonogram solution.
    // TODO! Check if raw access "[0]" is more performant that ".get(0)"
    pub fn cols(&self) -> usize {
        self.solution_grid
            .get(0)
            .expect("The nonogram solution has zero rows")
            .len()
    }

    /// Computes the row constraints for the nonogram solution.
    ///
    /// This generates segments of consecutive colors in each row.
    pub fn row_constraints(&self) -> Vec<Vec<NonogramSegment>> {
        let mut row_constraints = Vec::with_capacity(self.rows());
        for row_color_data in self.solution_grid.iter() {
            let mut row_segments = Vec::new();
            let mut previous_segment_color = 0;
            let mut segment_length = 0;
            for &segment_color in row_color_data.iter() {
                if segment_color == previous_segment_color {
                    segment_length += 1;
                } else {
                    if segment_length != 0 && previous_segment_color != 0 {
                        row_segments.push(nrule!(previous_segment_color, segment_length));
                    }
                    previous_segment_color = segment_color;
                    segment_length = 1;
                }
            }
            if segment_length != 0 && previous_segment_color != 0 {
                row_segments.push(nrule!(previous_segment_color, segment_length));
            }
            row_constraints.push(row_segments);
        }
        row_constraints
    }

    /// Computes the column constraints for the nonogram solution.
    ///
    /// This generates segments of consecutive colors in each column.
    pub fn col_constraints(&self) -> Vec<Vec<NonogramSegment>> {
        let mut col_constraints = Vec::with_capacity(self.rows());
        for col_idx in 0..self.cols() {
            let mut col_segments = Vec::new();
            let mut previous_segment_color = 0;
            let mut segment_length = 0;
            for segment_color in self
                .solution_grid
                .iter()
                .map(|row_color_data| row_color_data[col_idx])
            {
                if segment_color == previous_segment_color {
                    segment_length += 1;
                } else {
                    if segment_length != 0 && previous_segment_color != 0 {
                        col_segments.push(nrule!(previous_segment_color, segment_length));
                    }
                    previous_segment_color = segment_color;
                    segment_length = 1;
                }
            }
            if segment_length != 0 && previous_segment_color != 0 {
                col_segments.push(nrule!(previous_segment_color, segment_length));
            }
            col_constraints.push(col_segments);
        }
        col_constraints
    }

    /// Draws a line on the nonogram solution grid from `start` to `end` using a specified `color`.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting coordinate `(row, column)` of the line.
    /// * `end` - The ending coordinate `(row, column)` of the line.
    /// * `color` - The color to be applied to the line.
    ///
    /// # Behavior
    ///
    /// - If the line is primarily horizontal or nearly so, it fills the appropriate columns with the specified color.
    /// - If the line is primarily vertical or nearly so, it fills the appropriate rows with the specified color.
    pub fn draw_line(&mut self, start: (usize, usize), end: (usize, usize), color: usize) {
        let dy = (start.0 as isize - end.0 as isize).abs();
        let dx = (start.1 as isize - end.1 as isize).abs();

        if dx >= dy {
            let x_start = start.1.min(end.1);
            let x_end = start.1.max(end.1);

            for x in x_start..=x_end {
                self.solution_grid[start.0][x] = color;
            }
        } else {
            let y_start = start.0.min(end.0);
            let y_end = start.0.max(end.0);

            for y in y_start..=y_end {
                self.solution_grid[y][start.1] = color;
            }
        }
    }

    /// Checks if a given coordinate `(coord)` is within the line segment defined by `start` and `end`.
    ///
    /// # Arguments
    ///
    /// * `start` - An optional starting coordinate `(row, column)` of the line.
    /// * `end` - An optional ending coordinate `(row, column)` of the line.
    /// * `coord` - The coordinate `(row, column)` to check.
    ///
    /// # Returns
    ///
    /// `true` if `coord` lies within the line segment defined by `start` and `end`, otherwise `false`.
    pub fn in_line(
        &self,
        start: Option<(usize, usize)>,
        end: Option<(usize, usize)>,
        coord: (usize, usize),
    ) -> bool {
        if start.is_none() || end.is_none() {
            return false;
        }
        let start = start.unwrap();
        let end = end.unwrap();

        let dy = (start.0 as isize - end.0 as isize).abs();
        let dx = (start.1 as isize - end.1 as isize).abs();

        if dx >= dy {
            let x_start = start.1.min(end.1);
            let x_end = start.1.max(end.1);

            coord.0 == start.0 && (x_start..=x_end).contains(&coord.1)
        } else {
            let y_start = start.0.min(end.0);
            let y_end = start.0.max(end.0);

            coord.1 == start.1 && (y_start..=y_end).contains(&coord.0)
        }
    }

    /// Sets the number of columns in the nonogram solution grid.
    ///
    /// This adjusts the number of columns to the specified `cols` value, ensuring the grid maintains its constraints.
    ///
    /// # Arguments
    ///
    /// * `cols` - The target number of columns.
    pub fn set_cols(&mut self, cols: usize) {
        let current_cols = self.cols();
        let target_cols = cols.max(2);

        if target_cols > current_cols {
            for row_data in self.solution_grid.iter_mut() {
                row_data.append(&mut vec![BACKGROUND; target_cols - current_cols]);
            }
        } else if target_cols < current_cols {
            for row_data in self.solution_grid.iter_mut() {
                row_data.truncate(target_cols);
            }
        }
    }

    /// Sets the number of rows in the nonogram solution grid.
    ///
    /// This adjusts the number of rows to the specified `rows` value, ensuring the grid maintains its constraints.
    ///
    /// # Arguments
    ///
    /// * `rows` - The target number of rows.
    pub fn set_rows(&mut self, rows: usize) {
        let current_rows = self.rows();
        let target_rows = rows.max(2);

        if target_rows > current_rows {
            self.solution_grid.append(&mut vec![
                vec![BACKGROUND; self.cols()];
                target_rows - current_rows
            ]);
        } else if target_rows < current_rows {
            self.solution_grid.truncate(target_rows);
        }
    }

    /// Clears the entire nonogram solution grid, setting all cells to the default background color.
    pub fn clear(&mut self) {
        for row_data in self.solution_grid.iter_mut() {
            row_data.fill(0);
        }
    }

    /// Slides the nonogram solution grid by a specified amount in the `dx` (horizontal) and `dy` (vertical) directions.
    ///
    /// This shifts the grid contents while preserving boundaries.
    ///
    /// # Arguments
    ///
    /// * `dx` - The amount to shift horizontally.
    /// * `dy` - The amount to shift vertically.
    pub fn slide(&mut self, dx: isize, dy: isize) {
        let rows = self.rows();
        let cols = self.cols();
        let mut new_grid = vec![vec![0; cols]; rows];
        for y in 0..rows {
            for x in 0..cols {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;

                if (0..cols as isize).contains(&new_x) && (0..rows as isize).contains(&new_y) {
                    new_grid[new_y as usize][new_x as usize] = self.solution_grid[y][x];
                }
            }
        }
        self.solution_grid = new_grid;
    }
}

impl NonogramPalette {
    /// Returns the number of colors in the palette.
    pub fn len(&self) -> usize {
        self.color_palette.len()
    }

    /// Retrieves a color from the palette by its index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the color to retrieve.
    ///
    /// # Returns
    ///
    /// A reference to the color at the specified index.
    pub fn get(&self, index: usize) -> &str {
        &self.color_palette[index]
    }

    /// Sets the current brush color to the specified color.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to set as the current brush color.
    pub fn set_current(&mut self, color: String) {
        self.color_palette[self.brush] = color;
    }

    /// Retrieves the current brush color.
    ///
    /// # Returns
    ///
    /// A reference to the current brush color.
    pub fn get_current(&self) -> &str {
        &self.color_palette[self.brush]
    }

    /// Adds a new color to the palette.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to be added to the palette.
    pub fn add_color(&mut self, color: String) {
        self.color_palette.push(color);
    }

    /// Removes a color from the palette by its index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the color to remove.
    pub fn remove_color(&mut self, index: usize) {
        self.color_palette.remove(index);
        if self.brush > 0 {
            self.brush -= 1;
        }
    }

    /// Sets the brush to a specific color index in the palette.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the color to set as the current brush.
    pub fn set_brush(&mut self, index: usize) {
        if let Some(_) = self.color_palette.get(index) {
            self.brush = index;
        }
    }

    /// Displays the current brush information in the format: `index -> color`.
    ///
    /// # Returns
    ///
    /// A string representing the current brush index and color.
    pub fn show_brush(&self) -> String {
        format!("{} -> {}", self.brush, self.get_current())
    }

    /// Determines the appropriate text color (either black or white) based on the background color's luminance.
    ///
    /// # Arguments
    ///
    /// * `background` - The index of the background color.
    ///
    /// # Returns
    ///
    /// A string representing either `#000000` (black) or `#ffffff` (white) based on the luminance of the background.
    pub fn text_color(&self, background: usize) -> String {
        let background = self.get(background);
        if let Some((r, g, b)) = Self::parse_color(background) {
            if Self::is_darker(r, g, b) {
                "#ffffff".to_string()
            } else {
                "#000000".to_string()
            }
        } else {
            String::new()
        }
    }

    /// Determines the appropriate border color based on the background color's luminance.
    ///
    /// # Arguments
    ///
    /// * `background` - The index of the background color.
    ///
    /// # Returns
    ///
    /// A string representing either `#9ca3af` (default) or `#ffffff` depending on the background's luminance.
    pub fn border_color(&self, background: usize) -> String {
        let background = self.get(background);
        if let Some((r, g, b)) = Self::parse_color(background) {
            if Self::is_darker(r, g, b) {
                "#ffffff".to_string()
            } else {
                "#9ca3af".to_string()
            }
        } else {
            "#9ca3af".to_string()
        }
    }

    /// Checks if a given color is darker based on its RGB values.
    ///
    /// # Arguments
    ///
    /// * `r` - The red component of the color.
    /// * `g` - The green component of the color.
    /// * `b` - The blue component of the color.
    ///
    /// # Returns
    ///
    /// `true` if the color is darker, `false` otherwise.
    fn is_darker(r: u8, g: u8, b: u8) -> bool {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        luminance <= 0.5
    }

    /// Parses a hexadecimal color string into its RGB components.
    ///
    /// # Arguments
    ///
    /// * `color` - A string representing a hexadecimal color (e.g., `#RRGGBB`).
    ///
    /// # Returns
    ///
    /// An `Option<(u8, u8, u8)>` containing the red, green, and blue components if the parsing is successful, otherwise `None`.
    fn parse_color(color: &str) -> Option<(u8, u8, u8)> {
        if color.starts_with('#') && color.len() == 7 {
            let r = u8::from_str_radix(&color[1..3], 16).ok()?;
            let g = u8::from_str_radix(&color[3..5], 16).ok()?;
            let b = u8::from_str_radix(&color[5..7], 16).ok()?;
            Some((r, g, b))
        } else {
            None
        }
    }
}
