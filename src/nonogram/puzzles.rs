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

// Nonogram structures for file, palette, puzzle, and solution.
use super::definitions::{
    NonogramFile,     // Represents a file containing the solution and palette.
    NonogramPalette,  // Defines the set of colors used in a puzzle.
    NonogramPuzzle,   // Stores the constraints and dimensions of a puzzle.
    NonogramSolution, // Represents the solution grid of a puzzle.
};

// Default palette index for the background color.
use super::definitions::BACKGROUND;

/// A macro for defining Nonogram rules (constraints) concisely.
use crate::nrule;

/// Index of the leaves color in the palette.
pub const LEAVES: usize = 1;
/// Index of the wood color in the palette.
pub const WOOD: usize = 2;
/// Number of columns in the tree Nonogram puzzle.
const TREE_COLS: usize = 5;
/// Number of rows in the tree Nonogram puzzle.
const TREE_ROWS: usize = 5;

/// Generates a `NonogramFile` representing the tree Nonogram puzzle.
///
/// This includes the solution grid and the associated color palette.
///
/// # Returns
/// A `NonogramFile` containing the solution grid and palette for the tree puzzle.
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

/// Creates an empty solution grid for the tree Nonogram puzzle.
///
/// The empty grid is initialized with the background color index and matches the
/// dimensions of the tree puzzle.
///
/// # Returns
/// A `NonogramSolution` containing an empty solution grid.
pub fn tree_empty_nonogram_solution() -> NonogramSolution {
    NonogramSolution {
        solution_grid: vec![vec![BACKGROUND; TREE_COLS]; TREE_ROWS],
    }
}

/// Defines the constraints for the tree Nonogram puzzle.
///
/// The constraints specify the lengths and colors of contiguous segments for
/// each row and column in the puzzle.
///
/// # Returns
/// A `NonogramPuzzle` containing the constraints and dimensions for the tree puzzle.
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

/// Defines the color palette for the tree Nonogram puzzle.
///
/// The palette includes:
/// - Sky Blue (`#87ceeb`) for the background.
/// - Forest Green (`#228b22`) for the leaves.
/// - Saddle Brown (`#8b4513`) for the wood.
///
/// # Returns
/// A `NonogramPalette` containing the color definitions for the tree puzzle.
pub fn tree_nonogram_palette() -> NonogramPalette {
    NonogramPalette {
        color_palette: vec![
            String::from("#87ceeb"), // Sky Blue
            String::from("#228b22"), // Forest Green
            String::from("#8b4513"), // Saddle Brown
        ],
        brush: 0, // Default brush color (background)
    }
}
