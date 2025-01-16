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

/// Macro for defining palettes used in Nonogram puzzles.
use crate::define_palette;

/// Serialization and deserialization support for Nonogram data structures.
use serde::{Deserialize, Serialize};

/// Utilities for formatting and displaying Nonogram-related types.
use std::fmt;

/// Lazy initialization for static or constant data, used for Nonogram palettes.
use std::sync::LazyLock;

/// A palette used for Nonogram puzzles that stores a collection of colors and the currently selected brush color
#[derive(Clone, Deserialize, Serialize)]
pub struct NonogramPalette {
    /// The collection of colors in the palette, represented as hexadecimal strings.
    pub color_palette: Vec<String>,
    /// The index of the currently selected brush color.
    /// This field is not serialized.
    #[serde(skip_serializing, default)]
    pub brush: usize,
}

/// Index of the background color in the palette.
pub const BACKGROUND: usize = 0;

/// Default palette definition for Nonogram puzzles.
///
/// Colors include:
/// - Sky Blue (`#b7e1f9`)
/// - Green (`#2b711f`)
/// - Brown (`#8b4513`)
/// - Black (`#000000`)
/// - White (`#ffffff`)
/// - Orange (`#e65724`)
/// - Light Brown (`#ae7e40`)
/// - Light Green (`#879f31`)
pub const DEFAULT_PALETTE: LazyLock<NonogramPalette> = define_palette!(
    "#b7e1f9", "#2b711f", "#8b4513", "#000000", "#ffffff", "#e65724", "#ae7e40", "#879f31"
);

/// Represents a segment of a Nonogram puzzle.
///
/// Each segment has a color and a length, which define a sequence of
/// contiguous cells in the Nonogram grid.
#[derive(Clone, PartialEq, Debug)]
pub struct NonogramSegment {
    /// The color index of the segment, corresponding to a palette entry.
    pub color: usize,
    /// The length of the segment in cells.
    pub length: usize,
}

/// Represents the complete structure of a Nonogram puzzle.
///
/// This includes the number of rows and columns, as well as the constraints
/// for both rows and columns.
#[derive(Clone, PartialEq, Debug)]
pub struct NonogramPuzzle {
    /// The number of rows in the Nonogram grid.
    pub rows: usize,
    /// The number of columns in the Nonogram grid.
    pub cols: usize,
    /// Constraints for each row, specifying the segments in that row.
    pub row_constraints: Vec<Vec<NonogramSegment>>,
    /// Constraints for each column, specifying the segments in that column.
    pub col_constraints: Vec<Vec<NonogramSegment>>,
}

/// Represents the solution to a Nonogram puzzle.
///
/// The solution is stored as a grid of color indices, where each index corresponds
/// to an entry in the palette.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NonogramSolution {
    /// The solution grid, where each cell contains a color index.
    pub solution_grid: Vec<Vec<usize>>,
}
impl fmt::Display for NonogramSolution {
    /// Formats the solution as a grid of space-separated numbers for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.solution_grid {
            let row_str = row
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(f, "{}", row_str)?;
        }
        Ok(())
    }
}

/// Represents the contents of a Nonogram puzzle file.
///
/// This struct stores the solution grid and the associated color palette.
/// Initially, the rules (constraints) of the puzzle were considered for storage, but it was later determined
/// that storing the solution ensures reproducibility and simplifies editing.
/// While rules can be derived from the solution, the reverse is not true.
#[derive(Deserialize, Serialize, Clone)]
pub struct NonogramFile {
    /// The complete solution grid for the Nonogram puzzle.
    pub solution: NonogramSolution,
    /// The color palette associated with the puzzle, defining the colors used in the solution.
    pub palette: NonogramPalette,
}

/// Metadata and state for a Nonogram puzzle.
///
/// Includes the file name, display block size, and whether the puzzle is completed.
#[derive(Clone)]
pub struct NonogramData {
    /// The name of the file containing the puzzle.
    pub filename: String,
    /// The size of each block in pixels, for display purposes.
    pub block_size: usize,
    /// Whether the puzzle has been completed.
    pub completed: bool,
}
