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

use crate::nsol;

use super::definitions::{NonogramPuzzle, NonogramSegment, NonogramSolution, BACKGROUND};
use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use std::mem;

impl NonogramPuzzle {
    /// Generates a new random solution using a chromosome-based approach for the Nonogram puzzle.
    ///
    /// This method creates a `NonogramSolution` that satisfies the row constraints of the puzzle.
    /// The generated solution is constructed by randomly distributing gaps (background cells)
    /// around the segments in each row while ensuring that the total segment lengths and gaps
    /// match the specified number of columns in the puzzle.
    ///
    /// # Arguments
    ///
    /// - `rng`: A mutable reference to a random number generator implementing `StdRng`.
    ///   This ensures that the solution generation is reproducible when using the same seed.
    ///
    /// # Returns
    ///
    /// - A `NonogramSolution` containing a valid solution grid where all rows satisfy the
    ///   given row constraints. The solution grid is represented as a 2D vector of `usize` values,
    ///   where each cell indicates either a segment color or the background.
    ///
    /// # Advanced Explanation
    ///
    /// This method follows these steps for each row:
    ///
    /// 1. **Calculate Space Requirements**:
    ///    Determine the total length of all segments in the row, then calculate the remaining spaces (gaps)
    ///    by subtracting this total from the number of columns (`self.cols`).
    ///
    /// 2. **Random Gap Distribution**:
    ///    For each segment in the row:
    ///    - Generate a random gap size between 0 and the remaining spaces (inclusive).
    ///    - Append the gap to the chromosome.
    ///    - Deduct the gap size from the remaining spaces.
    ///    - Append the segment to the chromosome.
    ///
    /// 3. **Fill Remaining Gaps**:
    ///    After all segments have been added, if there are any remaining spaces, append them as a final gap.
    ///
    /// The random placement of gaps ensures that the solution is non-deterministic while always conforming
    /// to the row constraints. This makes it a useful starting point for optimization algorithms like
    /// genetic algorithms or simulated annealing, where random initial solutions are required.
    pub fn new_chromosome_solution(&self, rng: &mut StdRng) -> NonogramSolution {
        let solution_grid = self
            .row_constraints
            .iter()
            .map(|row_segments| {
                let row_segments_length = row_segments
                    .iter()
                    .map(|segment| segment.segment_length)
                    .sum::<usize>();
                let required_spaces = row_segments
                    .windows(2)
                    .filter(|segments| segments[0].segment_color == segments[1].segment_color)
                    .count();
                let chromosome_length = self.cols;
                let mut remaining_spaces =
                    chromosome_length - row_segments_length - required_spaces;
                let mut row_chromosome = Vec::with_capacity(chromosome_length);
                for (i, segment) in row_segments.iter().enumerate() {
                    if rng.gen_bool(0.5) {
                        let gap_size = rng.gen_range(0..=remaining_spaces);
                        remaining_spaces -= gap_size;
                        if gap_size != 0 {
                            let mut gap_segment = vec![BACKGROUND; gap_size];
                            row_chromosome.append(&mut gap_segment);
                        }
                    }
                    let color = segment.segment_color;
                    let mut segment = vec![segment.segment_color; segment.segment_length];
                    row_chromosome.append(&mut segment);
                    if let Some(next_segment) = row_segments.get(i + 1) {
                        if next_segment.segment_color == color {
                            row_chromosome.push(BACKGROUND);
                        }
                    }
                }
                if remaining_spaces != 0 {
                    let mut gap_segment = vec![BACKGROUND; remaining_spaces];
                    row_chromosome.append(&mut gap_segment);
                }
                row_chromosome
            })
            .collect();
        NonogramSolution { solution_grid }
    }

    /// Calculates the "badness" score of a given candidate solution in relation to the expected column constraints.
    ///
    /// This method evaluates how closely a candidate solution matches the expected column constraints by assigning
    /// a "badness" score, which quantifies the deviation from the expected solution. A lower score indicates a closer
    /// match.
    ///
    /// # Arguments
    ///
    /// - `candidate`: A reference to a `NonogramSolution`, which represents a proposed solution to the puzzle.
    ///   The `NonogramSolution` consists of a `solution_grid`, a 2D grid where each cell contains the color (as a `usize`)
    ///   of that cell.
    ///
    /// # Returns
    ///
    /// - A `usize` representing the total "badness" score of the candidate solution.
    ///
    /// # Advanced Explanation
    ///
    /// This method compares the candidate's column constraints against the expected column constraints by:
    ///
    /// 1. Summing the lengths of segments in both the candidate and expected columns.
    /// 2. Calculating the absolute difference between these sums.
    /// 3. Aggregating these differences across all columns to compute the total "badness" score.
    ///
    /// This scoring mechanism assumes that row constraints are already satisfied, as the `NonogramSolution`
    /// generation methods are designed to produce candidates fulfilling the row constraints.
    ///
    /// Nonograms are popular for their blend of logical deduction and artistic creation, as solving them
    /// often reveals a hidden image encoded in the grid. This scoring function serves as a useful heuristic
    /// for guiding algorithms in generating or improving candidate solutions.
    pub fn score(&self, candidate: &NonogramSolution) -> usize {
        candidate
            .col_constraints()
            .iter()
            .zip(self.col_constraints.iter())
            .map(|(current_segments, expected_segments)| {
                let max_len = current_segments.len().max(expected_segments.len());
                let current = Self::normalize_vec(current_segments, max_len);
                let expected = Self::normalize_vec(expected_segments, max_len);
                current
                    .iter()
                    .zip(expected.iter())
                    .map(|(cur, exp)| {
                        if cur.segment_color == exp.segment_color {
                            (cur.segment_length as isize - exp.segment_length as isize).abs()
                                as usize
                        } else {
                            cur.segment_length * 2 + exp.segment_length
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    pub fn _score(&self, candidate: &NonogramSolution) -> usize {
        candidate
            .col_constraints()
            .iter()
            .zip(self.col_constraints.iter())
            .map(|(current_segments, expected_segments)| {
                let c: usize = current_segments
                    .iter()
                    .map(|segment| segment.segment_length)
                    .sum();
                let e: usize = expected_segments
                    .iter()
                    .map(|segment| segment.segment_length)
                    .sum();
                (c as isize - e as isize).abs() as usize
            })
            .sum::<usize>()
    }

    pub fn normalize_vec(vec: &Vec<NonogramSegment>, len: usize) -> Vec<NonogramSegment> {
        let padding = len.saturating_sub(vec.len());
        let mut normalized_vec = Vec::with_capacity(len);
        normalized_vec.extend(vec![
            NonogramSegment {
                segment_color: 0,
                segment_length: 0
            };
            padding
        ]);
        normalized_vec.extend(vec.iter().cloned());
        normalized_vec
    }

    /// Creates two new solutions using a genetic approach, combining rows (chromosomes) from two ancestor solutions.
    ///
    /// This method generates two `NonogramSolution` instances by performing a uniform crossover operation
    /// between two parent solutions. Each row (chromosome) in the descendants is randomly selected from one
    /// of the two ancestors, ensuring diversity while preserving the structural integrity of the solutions.
    ///
    /// # Arguments
    ///
    /// - `ancestor_1`: A reference to the first parent solution (`NonogramSolution`).
    /// - `ancestor_2`: A reference to the second parent solution (`NonogramSolution`).
    /// - `cross_probability`: The probability of a crossover between chromosomes happening.
    /// - `rng`: A mutable reference to a random number generator implementing `StdRng`.
    ///   This ensures reproducibility of the crossover operation when using the same seed.
    ///
    /// # Returns
    ///
    /// - A tuple containing two new solutions (`descendant_1`, `descendant_2`), which are combinations of the rows
    ///   (chromosomes) from the two ancestor solutions.
    ///
    /// # Method Details
    ///
    /// - For each row in the grid:
    ///   - A random decision is made (with `cross_probability`) to take the row from either `ancestor_1` or `ancestor_2`.
    ///   - The row from the chosen ancestor is added to the corresponding descendant.
    ///   - This ensures that each descendant is a unique combination of rows from the two ancestors.
    ///
    /// - The method uses `.get(i)` to safely access the rows in the ancestor solutions. If a row is missing in either
    ///   ancestor, the method panics with an error message indicating the missing row's index.
    ///
    /// This genetic approach mimics biological inheritance, where traits (rows in this case) are passed
    /// from both parents to offspring. The uniform crossover is useful in optimization algorithms like
    /// genetic algorithms, where diverse offspring are crucial for exploring the solution space.
    // TODO! Check if raw access "[i]" is more performant that ".get(i)"
    pub fn uniform_cross(
        &self,
        ancestor_1: &NonogramSolution,
        ancestor_2: &NonogramSolution,
        cross_probability: f64,
        rng: &mut StdRng,
    ) -> (NonogramSolution, NonogramSolution) {
        let mut descendant_1 = Vec::with_capacity(self.rows);
        let mut descendant_2 = Vec::with_capacity(self.rows);

        for i in 0..self.rows {
            if rng.gen_bool(cross_probability) {
                descendant_1.push(
                    ancestor_1
                        .solution_grid
                        .get(i)
                        .expect(&format!("El primer ancestro no tiene la fila {}", i + 1))
                        .clone(),
                );
                descendant_2.push(
                    ancestor_2
                        .solution_grid
                        .get(i)
                        .expect(&format!("El segundo ancestro no tiene la fila {}", i + 1))
                        .clone(),
                );
            } else {
                descendant_2.push(
                    ancestor_1
                        .solution_grid
                        .get(i)
                        .expect(&format!("El primer ancestro no tiene la fila {}", i + 1))
                        .clone(),
                );
                descendant_1.push(
                    ancestor_2
                        .solution_grid
                        .get(i)
                        .expect(&format!("El segundo ancestro no tiene la fila {}", i + 1))
                        .clone(),
                );
            }
        }

        (nsol!(descendant_1), nsol!(descendant_2))
    }

    /// Creates two new solutions using a genetic two-point crossover approach, combining rows (chromosomes) from two ancestor solutions.
    ///
    /// This method generates two `NonogramSolution` instances by selecting two random crossover points
    /// and swapping the rows (chromosomes) between two ancestor solutions within the range defined by these points.
    /// Rows outside the crossover range are directly inherited from their respective ancestors.
    ///
    /// # Arguments
    ///
    /// - `ancestor_1`: A reference to the first parent solution (`NonogramSolution`).
    /// - `ancestor_2`: A reference to the second parent solution (`NonogramSolution`).
    /// - `cross_probability`: The probability of a crossover happening at all.
    /// - `rng`: A mutable reference to a random number generator implementing `StdRng`.
    ///   This ensures reproducibility of the crossover operation when using the same seed.
    ///
    /// # Returns
    ///
    /// - A tuple containing two new solutions (`descendant_1`, `descendant_2`), which are combinations of rows
    ///   from the two ancestor solutions, with rows swapped between the two crossover points.
    ///
    /// # Method Details
    ///
    /// 1. **Determine Crossover Will Happen**:
    ///    - A random decision is made (with `cross_probability`) to apply the crossover or simply return clones.
    ///
    /// 2. **Determine Crossover Points**:
    ///    - Two random crossover points, `point_1` and `point_2`, are selected within the range `[1, self.cols - 1)`.
    ///    - If `point_1` is greater than `point_2`, the values are swapped to ensure `point_1 < point_2`.
    ///
    /// 3. **Row Inheritance**:
    ///    - For rows outside the range `[point_1, point_2]`:
    ///      - Rows are inherited directly from `ancestor_1` into `descendant_1` and from `ancestor_2` into `descendant_2`.
    ///    - For rows within the range `[point_1, point_2]`:
    ///      - Rows are swapped, with `ancestor_1` contributing to `descendant_2` and `ancestor_2` contributing to `descendant_1`.
    ///
    /// 4. **Indexing Safety**:
    ///    - Rows are accessed using `.get(i)` to ensure safe access to the ancestor grids. If a row is missing in either
    ///      ancestor, the method panics with an error message indicating the missing row's index.
    ///
    /// This two-point crossover method introduces more structured variability compared to uniform crossover,
    /// preserving longer contiguous sections of chromosomes from each ancestor. It is a valuable technique in
    /// genetic algorithms for maintaining solution diversity while combining desirable traits from both parents.
    // TODO! Check if raw access "[i]" is more performant that ".get(i)"
    pub fn two_point_cross(
        &self,
        ancestor_1: &NonogramSolution,
        ancestor_2: &NonogramSolution,
        cross_probability: f64,
        rng: &mut StdRng,
    ) -> (NonogramSolution, NonogramSolution) {
        if !rng.gen_bool(cross_probability) {
            return (ancestor_1.clone(), ancestor_2.clone());
        }

        let mut descendant_1 = Vec::with_capacity(self.rows);
        let mut descendant_2 = Vec::with_capacity(self.rows);

        let mut point_1 = rng.gen_range(1..(self.cols - 1));
        let mut point_2 = rng.gen_range(1..(self.cols - 1));

        if point_1 > point_2 {
            mem::swap(&mut point_1, &mut point_2);
        }

        for i in 0..self.rows {
            if i < point_1 || i > point_2 {
                descendant_1.push(
                    ancestor_1
                        .solution_grid
                        .get(i)
                        .expect(&format!("El primer ancestro no tiene la fila {}", i + 1))
                        .clone(),
                );
                descendant_2.push(
                    ancestor_2
                        .solution_grid
                        .get(i)
                        .expect(&format!("El segundo ancestro no tiene la fila {}", i + 1))
                        .clone(),
                );
            } else {
                descendant_2.push(
                    ancestor_1
                        .solution_grid
                        .get(i)
                        .expect(&format!("El primer ancestro no tiene la fila {}", i + 1))
                        .clone(),
                );
                descendant_1.push(
                    ancestor_2
                        .solution_grid
                        .get(i)
                        .expect(&format!("El segundo ancestro no tiene la fila {}", i + 1))
                        .clone(),
                );
            }
        }

        (nsol!(descendant_1), nsol!(descendant_2))
    }

    /// Applies mutation to a candidate solution by randomly sliding a single segment within rows (chromosomes) based on a given mutation probability.
    ///
    /// This method introduces variability into a `NonogramSolution` by occasionally swapping an end of a segment with an adjacent background cell,
    /// effectively "sliding" the segment within a row. The probability of mutation is controlled by the `mutation_probability` parameter,
    /// and the mutation affects only rows where the random condition is met.
    ///
    /// # Arguments
    ///
    /// - `candidate`: A mutable reference to a `NonogramSolution`. This is the solution that will be mutated.
    /// - `mutation_probability`: A `f64` value representing the probability of mutating each row.
    ///   Values should typically range between `0.0` and `1.0`, where `0.0` means no mutation and `1.0` means every row mutates.
    /// - `rng`: A mutable reference to a random number generator implementing `StdRng`.
    ///   This ensures reproducibility of mutations when using the same seed.
    ///
    /// # Method Details
    ///
    /// 1. **Row Mutation Check**:
    ///    - For each row in the candidate solution's grid, a random boolean is generated using `rng.gen_bool(mutation_probability)`.
    ///    - If the result is true, the row is selected for mutation.
    ///
    /// 2. **Identifying Slidable Segments**:
    ///    - The method `Self::get_slidables` is used to identify segments that are "slidable." This refers to segments where
    ///      either end of the segment can be swapped with adjacent background cells while maintaining the row's integrity.
    ///
    /// 3. **Sliding (Swapping) Segments**:
    ///    - A random pair of slidable indices is selected using `choose(rng)`.
    ///    - The segments at these indices are swapped. Essentially, the end of the segment is slid into the adjacent empty space (background),
    ///      and the background is moved to the position previously occupied by the segment’s end.
    ///
    /// This mutation process mimics the genetic mutation operation, introducing random changes that allow exploration of the solution space.
    /// It helps avoid premature convergence by creating diversity in the population of solutions. The mutation probability controls
    /// the degree of randomness, with higher probabilities generating more diverse solutions and lower probabilities preserving the
    /// stability of promising solutions.
    ///
    /// # Notes
    ///
    /// - Consider using the [Bernoulli] distribution if mutation probabilities remain constant over many iterations,
    ///   as it may improve performance in these scenarios.
    /// - The mutation process respects the integrity of the row, ensuring that only valid swaps occur based on the
    ///   constraints of the puzzle.
    pub fn chromosome_mutation(
        &self,
        candidate: &mut NonogramSolution,
        mutation_probability: f64,
        slide_tries: usize,
        rng: &mut StdRng,
    ) {
        for row_segment_colors in candidate.solution_grid.iter_mut() {
            (0..slide_tries).for_each(|_| {
                if rng.gen_bool(mutation_probability) {
                    let slidable_segments = Self::get_slidables(row_segment_colors);
                    if let Some(&(a, b)) = slidable_segments.choose(rng) {
                        row_segment_colors.swap(a, b);
                    }
                }
            });
        }
    }

    /// Identifies all valid segment slides within a row based on its segment colors.
    ///
    /// This method analyzes a row of segment colors and determines all the possible "slidable" segment positions.
    /// A "slidable" segment is one where an end of a segment can be swapped with an adjacent background space,
    /// effectively sliding the segment within the row. This is important for mutation operations, where segments
    /// are randomly slid to introduce variability in genetic algorithms.
    ///
    /// # Arguments
    ///
    /// - `row_segment_colors`: A reference to a `Vec<usize>`, representing a row in a `NonogramSolution`.
    ///   The row consists of segment colors (non-zero values) and background (zero values). A segment is a contiguous
    ///   group of non-zero values representing a color, and the background is denoted by zero.
    ///
    /// # Returns
    ///
    /// - A `Vec<(usize, usize)>` containing pairs of indices where segments can be slid. Each pair represents
    ///   a valid "slidable" position, where the first index is the end of a segment (the "sliding" point),
    ///   and the second index is the start of the adjacent background space that can replace the segment's end.
    ///
    /// # Method Details
    ///
    /// 1. **Identify Segments and Background**:
    ///    - The method iterates over the row to detect segments (non-zero values) and background cells (zero values).
    ///    - A segment is defined as a contiguous block of non-zero values. Background cells are areas between or around segments.
    ///
    /// 2. **Detect Possible Slidable Positions**:
    ///    - When a background cell (`BACKGROUND`) is adjacent to a segment, the method identifies potential sliding points.
    ///    - The end of the segment and the adjacent background are marked as a valid sliding pair.
    ///
    /// 3. **Slide Segments**:
    ///    - If a segment is detected, and there is an adjacent background, the method identifies two types of slides:
    ///      - **Left Slide**: The end of a segment can be swapped with the adjacent background to the left.
    ///      - **Right Slide**: The start of the segment can be swapped with the adjacent background to the right.
    ///    - The method ensures that sliding only happens between adjacent background and segment, maintaining the row's integrity.
    ///
    /// 4. **Return Valid Slide Indices**:
    ///    - The method returns a vector of tuples representing the valid sliding indices within the row.
    ///
    /// This method is particularly useful in mutation operations for genetic algorithms, where sliding segments
    /// within rows helps introduce diversity and avoid premature convergence in solution search spaces.
    ///
    /// # Example:
    /// Given a row `vec![0, 1, 1, 0, 2, 2, 0]`, the method would return pairs of indices that represent the
    /// valid places where segments `1` and `2` can slide within the background: vec![(0, 2), (1, 3), (3, 5), (4, 6)].
    pub fn get_slidables(row_segment_colors: &Vec<usize>) -> Vec<(usize, usize)> {
        let mut slidable_segments = Vec::new();

        let mut segment_colors_iter = row_segment_colors.iter().enumerate();

        // We check atleast one element exist
        if let Some((_, previous_block_color)) = segment_colors_iter.next() {
            let mut previous_block_color = *previous_block_color;
            let mut previous_segment_color = None;
            let mut background_end = None;

            // We set a marker in the segment start if needed
            let mut segment_start = if previous_block_color != 0 {
                Some(0)
            } else {
                None
            };
            for (i, &current_block_color) in segment_colors_iter {
                match (previous_block_color, current_block_color) {
                    // Set the background end and segment start
                    (BACKGROUND, b) if b != BACKGROUND => {
                        background_end = Some(i - 1);
                        segment_start = Some(i);
                        if let Some(previous_color) = previous_segment_color {
                            if previous_color == b {
                                background_end = None;
                            }
                        }
                    }
                    // Push valid slides of the segment
                    (a, BACKGROUND) if a != BACKGROUND => {
                        previous_segment_color = Some(a);
                        if let Some(end) = background_end {
                            slidable_segments.push((end, i - 1 /* segment_a_end */));
                            background_end = None;
                        }

                        if row_segment_colors.get(i + 1).is_none()
                            || previous_segment_color.is_none()
                            || previous_segment_color.unwrap()
                                != *row_segment_colors.get(i + 1).unwrap()
                        {
                            slidable_segments.push((
                                segment_start.expect("Couldn't find the segment start, look into setting the segment start and update of it to find the error."),
                                i /* background_start */
                                )
                            );
                        }
                        segment_start = None;
                    }
                    // Check the left slide and update the segment start
                    (a, b) if a != b => {
                        if let Some(end) = background_end {
                            slidable_segments.push((end, i - 1 /* segment_a_end */));
                            background_end = None;
                        }

                        segment_start = Some(i /* segment_b_start */);
                    }
                    // Same segment, do nothing
                    (_, _) => {}
                }
                // Update the previous segment
                previous_block_color = current_block_color;
            }
            // If the background marker it's set, the last segment can slide
            if let Some(end) = background_end {
                slidable_segments.push((end, row_segment_colors.len() - 1 /* segment_end */));
            }
        }
        slidable_segments
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use crate::nonogram::puzzles::tree_nonogram_puzzle;

    use super::*;

    #[test]
    fn assert_row_constraints_mantain() {
        let puzzle = tree_nonogram_puzzle();
        let mut rng = StdRng::seed_from_u64(0);
        let solution = puzzle.new_chromosome_solution(&mut rng);
        assert_eq!(solution.row_constraints(), puzzle.row_constraints)
    }

    // Helper function to compare slidables
    // This function compares the actual and expected slidable positions and checks that they are identical.
    // It ensures that the number of slidable positions is the same and that the corresponding positions
    // match in value and order.
    fn assert_slidable_positions_equal(actual: Vec<(usize, usize)>, expected: Vec<(usize, usize)>) {
        println!("Actual: {:?}", actual);
        println!("Expected: {:?}", expected);

        // Assert the lengths are equal first to avoid unnecessary iteration
        assert_eq!(
            actual.len(),
            expected.len(),
            "The number of slidable positions does not match."
        );

        // Iterate over both vectors and assert each corresponding pair of slidable positions are the same
        for (a, b) in actual.iter().zip(expected.iter()) {
            assert_eq!(
                a, b,
                "The slidable positions do not match: expected {:?}, but got {:?}",
                a, b
            );
        }
    }

    // Test cases for the `get_slidables` method

    // Test with an empty row, where there are no slidable segments.
    // Expected result: No slidable positions, so an empty Vec should be returned.
    #[test]
    fn get_slidables_from_empty_row() {
        let row_segment_colors = vec![];
        let result = NonogramPuzzle::get_slidables(&row_segment_colors);
        assert_slidable_positions_equal(result, vec![]);
    }

    // Test with a row full of background elements (0), where no slidable segments exist.
    // Expected result: No slidable positions, so an empty Vec should be returned.
    #[test]
    fn get_slidables_from_background_row() {
        let row_segment_colors = vec![0, 0, 0, 0, 0];
        let result = NonogramPuzzle::get_slidables(&row_segment_colors);
        assert_slidable_positions_equal(result, vec![]);
    }

    // Test with a row containing a single segment.
    // Expected result: The segment can slide in both directions, so two valid sliding positions are expected.
    #[test]
    fn get_slidables_from_single_segment() {
        let row_segment_colors = vec![0, 1, 1, 0];
        let result = NonogramPuzzle::get_slidables(&row_segment_colors);
        assert_slidable_positions_equal(result, vec![(0, 2), (1, 3)]);
    }

    // Test with a row containing multiple segments that each can slide.
    // Expected result: Each segment can slide in both directions, so four valid sliding positions are expected.
    #[test]
    fn get_slidables_from_multiple_segments() {
        let row_segment_colors = vec![0, 1, 1, 0, 2, 2, 0];
        let result = NonogramPuzzle::get_slidables(&row_segment_colors);
        assert_slidable_positions_equal(result, vec![(0, 2), (1, 3), (3, 5), (4, 6)]);
    }

    // Test with adjacent segments, where both outer segments can slide once.
    // Expected result: Two valid sliding positions.
    #[test]
    fn get_slidables_from_adjacent_segments() {
        let row_segment_colors = vec![0, 1, 2, 1, 0];
        let result = NonogramPuzzle::get_slidables(&row_segment_colors);
        assert_slidable_positions_equal(result, vec![(0, 1), (3, 4)]);
    }

    // Test with end segments (edge cases).
    // Expected result: Both end segments can slide once, while the inner segment can slide in both directions.
    #[test]
    fn get_slidables_from_end_segments() {
        let row_segment_colors = vec![1, 0, 2, 0, 1];
        let result = NonogramPuzzle::get_slidables(&row_segment_colors);
        assert_slidable_positions_equal(result, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    }

    // Test with segments of the same color
    // Expected result: Zero segments can slide, since they will fuse into a single segment otherwise
    #[test]
    fn get_slidables_from_same_color_segments() {
        let row_segment_colors = vec![1, 0, 1, 0, 1];
        let result = NonogramPuzzle::get_slidables(&row_segment_colors);
        assert_slidable_positions_equal(result, vec![]);
    }

    // Test mutation of a candidate puzzle solution and ensure that the row_constraints remain intact after mutation.
    #[test]
    fn same_puzzle_after_mutation() {
        let puzzle = crate::nonogram::puzzles::tree_nonogram_puzzle();
        let mut rng = rand::SeedableRng::seed_from_u64(0);

        // Create the initial candidate solution based on the puzzle
        let mut candidate = puzzle.new_chromosome_solution(&mut rng);
        println!("Candidate: {:?}", candidate.solution_grid);

        // Mutate the candidate solution
        puzzle.chromosome_mutation(&mut candidate, 0.5, 5, &mut rng);

        // Convert the mutated candidate back into a puzzle
        let mutated = NonogramPuzzle::from_solution(&candidate);

        // Assert that the mutated puzzle has the same row_constraints as the original one
        assert_eq!(puzzle.row_constraints, mutated.row_constraints);
    }

    // Test the uniform crossover between two parent puzzle solutions and ensure both children's row_constraints remain intact.
    #[test]
    fn same_puzzle_after_cross() {
        let puzzle = crate::nonogram::puzzles::tree_nonogram_puzzle();
        let mut rng = rand::SeedableRng::seed_from_u64(0);

        // Create two initial candidate solutions based on the puzzle
        let ancestor_1 = puzzle.new_chromosome_solution(&mut rng);
        let ancestor_2 = puzzle.new_chromosome_solution(&mut rng);

        // Perform a uniform cross between the two ancestors
        let (child_1, child_2) = puzzle.uniform_cross(&ancestor_1, &ancestor_2, 0.5, &mut rng);

        // Convert the children back into puzzles
        let mutated_1 = NonogramPuzzle::from_solution(&child_1);
        let mutated_2 = NonogramPuzzle::from_solution(&child_2);

        // Assert that both children have the same row_constraints as the original puzzle
        assert_eq!(puzzle.row_constraints, mutated_1.row_constraints);
        assert_eq!(puzzle.row_constraints, mutated_2.row_constraints);
    }

    // Test the combination of mutation and crossover in one operation, ensuring that the row_constraints are preserved.
    #[test]
    fn same_puzzle_after_mutation_and_cross() {
        let puzzle = crate::nonogram::puzzles::tree_nonogram_puzzle();
        let mut rng = rand::SeedableRng::seed_from_u64(0);

        // Create two initial candidate solutions based on the puzzle
        let mut ancestor_1 = puzzle.new_chromosome_solution(&mut rng);
        let ancestor_2 = puzzle.new_chromosome_solution(&mut rng);

        // Mutate the first ancestor
        puzzle.chromosome_mutation(&mut ancestor_1, 0.5, 5, &mut rng);

        // Perform a uniform cross between the mutated ancestor_1 and ancestor_2
        let (child_1, child_2) = puzzle.uniform_cross(&ancestor_1, &ancestor_2, 0.5, &mut rng);

        // Convert the children back into puzzles
        let mutated_1 = NonogramPuzzle::from_solution(&child_1);
        let mutated_2 = NonogramPuzzle::from_solution(&child_2);

        // Assert that both children have the same row_constraints as the original puzzle
        assert_eq!(puzzle.row_constraints, mutated_1.row_constraints);
        assert_eq!(puzzle.row_constraints, mutated_2.row_constraints);
    }
}
