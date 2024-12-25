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

use super::definitions::{NonogramPuzzle, NonogramSegment, NonogramSolution};
use crate::nrule;

impl NonogramPuzzle {
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
    pub fn rows(&self) -> usize {
        self.solution_grid.len()
    }

    // TODO! Check if raw access "[0]" is more performant that ".get(0)"
    pub fn cols(&self) -> usize {
        self.solution_grid
            .get(0)
            .expect("La solución del nonograma tiene cero filas")
            .len()
    }

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

    pub fn col_constraints(&self) -> Vec<Vec<NonogramSegment>> {
        let mut col_constraints = Vec::with_capacity(self.cols());
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
}
