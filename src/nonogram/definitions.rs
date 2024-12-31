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

use serde::{Deserialize, Serialize};

use crate::define_palette;

// TODO! Remove brush color, since global context can be accessed by type
#[derive(Clone, Deserialize, Serialize)]
pub struct NonogramPalette {
    pub color_palette: Vec<String>,
    #[serde(skip_serializing, default)]
    pub brush: usize,
}

pub const BACKGROUND: usize = 0;
pub const LEAVES: usize = 1;
pub const WOOD: usize = 2;
define_palette!(
    DEFAULT_PALETTE,
    "#b7e1f9", // Sky Blue
    "#2b711f", // Green (Somewhat Dark, like foliage)
    "#8b4513", // Brown (Somewhat Dark, like a tree)
    "#000000", // Black
    "#ffffff", // White
    "#e65724", // Orange (Somewhat Bright)
    "#ae7e40", // Light Brown (Soft, like beige)
    "#879f31"  // Light Green (Like Grass)
);

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct NonogramSegment {
    pub segment_color: usize,
    pub segment_length: usize,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct NonogramPuzzle {
    pub rows: usize,
    pub cols: usize,
    pub row_constraints: Vec<Vec<NonogramSegment>>,
    pub col_constraints: Vec<Vec<NonogramSegment>>,
}

#[derive(Clone)]
pub struct NonogramSolution {
    pub solution_grid: Vec<Vec<usize>>,
}

#[derive(Deserialize, Serialize)]
pub struct NonogramFile {
    pub puzzle: NonogramPuzzle,
    pub palette: NonogramPalette,
}

#[derive(Clone)]
pub struct NonogramData {
    pub filename: String,
    pub block_size: usize,
    pub start: Option<(usize, usize)>,
    pub end: Option<(usize, usize)>,
}
