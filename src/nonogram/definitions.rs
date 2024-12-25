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

#[derive(Clone)]
pub struct NonogramPalette {
    pub color_palette: Vec<String>,
    pub brush_color: usize,
}

pub const BACKGROUND: usize = 0;
pub const PRIMARY: usize = 1;
pub const SECUNDARY: usize = 2;
pub const TERCIARY: usize = 3;
pub const QUATERNARY: usize = 4;
pub const QUINTINARY: usize = 5;

pub struct NonogramSegment {
    pub segment_color: usize,
    pub segment_length: usize,
}

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
