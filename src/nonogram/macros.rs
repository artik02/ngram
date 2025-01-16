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

/// Defines a macro for creating a lazily initialized Nonogram palette.
///
/// This macro generates a `LazyLock` instance containing a `NonogramPalette` with the specified colors.
///
/// # Arguments
/// - `$color:expr`: A series of string literals representing hexadecimal color codes.
///
/// # Example
/// ```rust
/// let palette = define_palette!("#FFFFFF", "#000000", "#FF0000");
/// ```
#[macro_export]
macro_rules! define_palette {
    ($($color:expr),+) => {
        std::sync::LazyLock::new(|| {
            crate::nonogram::definitions::NonogramPalette {
                color_palette: vec![$(String::from($color)),+],
                brush: 0,
            }
        })
    };
}

/// Defines a macro for creating a Nonogram segment.
///
/// This macro simplifies the creation of a `NonogramSegment` by specifying its color and length.
/// Makes the code more readable when used with constants.
///
/// # Arguments
/// - `$color:expr`: The index of the color in the palette.
/// - `$length:expr`: The length of the segment.
///
/// # Example
/// ```rust
/// let segment = nrule!(1, 5); // Creates a segment with color index 1 and length 5.
/// ```
#[macro_export]
macro_rules! nrule {
    ($color:expr, $length:expr) => {
        crate::nonogram::definitions::NonogramSegment {
            color: $color,
            length: $length,
        }
    };
}

/// Defines a macro for creating a Nonogram solution.
///
/// This macro simplifies the creation of a `NonogramSolution` by directly providing the grid.
///
/// # Arguments
/// - `$grid:expr`: A 2D vector representing the solution grid of the Nonogram.
///
/// # Example
/// ```rust
/// let solution = nsol!(vec![
///     vec![0, 1, 1],
///     vec![1, 0, 1],
///     vec![0, 0, 1],
/// ]);
/// ```
#[macro_export]
macro_rules! nsol {
    ($grid:expr) => {
        crate::nonogram::definitions::NonogramSolution {
            solution_grid: $grid,
        }
    };
}
