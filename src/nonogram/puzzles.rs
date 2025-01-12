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

use crate::nrule;

use super::definitions::{
    NonogramFile, NonogramPalette, NonogramPuzzle, NonogramSolution, LEAVES, WOOD,
};

const TREE_COLS: usize = 5;
const TREE_ROWS: usize = 5;

pub fn tree_nonogram_file() -> NonogramFile {
    NonogramFile {
        solution: NonogramSolution {
            solution_grid: vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 2, 1, 1],
                vec![0, 0, 2, 0, 0],
                vec![0, 0, 2, 0, 0],
            ],
        },
        palette: tree_nonogram_palette(),
    }
}

pub fn tree_empty_nonogram_solution() -> NonogramSolution {
    NonogramSolution {
        solution_grid: vec![vec![0; TREE_COLS]; TREE_ROWS],
    }
}

pub fn tree_nonogram_puzzle() -> NonogramPuzzle {
    NonogramPuzzle {
        rows: TREE_ROWS,
        cols: TREE_COLS,
        row_constraints: vec![
            vec![nrule!(LEAVES, 3)],
            vec![nrule!(LEAVES, 5)],
            vec![nrule!(LEAVES, 2), nrule!(WOOD, 1), nrule!(LEAVES, 2)],
            vec![nrule!(WOOD, 1)],
            vec![nrule!(WOOD, 1)],
        ],
        col_constraints: vec![
            vec![nrule!(LEAVES, 2)],
            vec![nrule!(LEAVES, 3)],
            vec![nrule!(LEAVES, 2), nrule!(WOOD, 3)],
            vec![nrule!(LEAVES, 3)],
            vec![nrule!(LEAVES, 2)],
        ],
    }
}

pub fn tree_nonogram_palette() -> NonogramPalette {
    NonogramPalette {
        color_palette: vec![
            String::from("#87ceeb"),
            String::from("#228b22"),
            String::from("#8b4513"),
        ],
        brush: 0,
    }
}
