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
    pub fn new_chromosome_solution(&self, rng: &mut StdRng) -> NonogramSolution {
        let solution_grid = self
            .row_constraints
            .iter()
            .map(|row_segments| {
                let row_segments_length = row_segments
                    .iter()
                    .map(|segment| segment.length)
                    .sum::<usize>();
                let required_spaces = row_segments
                    .windows(2)
                    .filter(|segments| segments[0].color == segments[1].color)
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
                    let color = segment.color;
                    let mut segment = vec![segment.color; segment.length];
                    row_chromosome.append(&mut segment);
                    if let Some(next_segment) = row_segments.get(i + 1) {
                        if next_segment.color == color {
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
                        if cur.color == exp.color {
                            (cur.length as isize - exp.length as isize).abs() as usize
                        } else {
                            cur.length + exp.length
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
                let c: usize = current_segments.iter().map(|segment| segment.length).sum();
                let e: usize = expected_segments.iter().map(|segment| segment.length).sum();
                (c as isize - e as isize).abs() as usize
            })
            .sum::<usize>()
    }

    pub fn normalize_vec(vec: &Vec<NonogramSegment>, len: usize) -> Vec<NonogramSegment> {
        let padding = len.saturating_sub(vec.len());
        let mut normalized_vec = Vec::with_capacity(len);
        normalized_vec.extend(vec![
            NonogramSegment {
                color: 0,
                length: 0
            };
            padding
        ]);
        normalized_vec.extend(vec.iter().cloned());
        normalized_vec
    }

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
    //
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
