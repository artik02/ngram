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

use super::definitions::{NonogramPuzzle, NonogramSolution, BACKGROUND};
use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use std::mem;

impl NonogramSolution {
    pub fn new_chromosome_solution(puzzle: &NonogramPuzzle, rng: &mut StdRng) -> Self {
        let solution_grid = puzzle
            .row_constraints
            .iter()
            .map(|row_segments| {
                let row_segments_length = row_segments
                    .iter()
                    .map(|segment| segment.segment_length)
                    .sum::<usize>();
                let chromosome_length = puzzle.cols;
                let mut remaining_spaces = chromosome_length - row_segments_length;
                let mut row_chromosome = Vec::with_capacity(chromosome_length);
                for segment in row_segments.iter() {
                    let gap_size = rng.gen_range(0..=remaining_spaces);
                    remaining_spaces -= gap_size;
                    if gap_size != 0 {
                        let mut gap_segment = vec![BACKGROUND; gap_size];
                        row_chromosome.append(&mut gap_segment);
                    }
                    let mut segment = vec![segment.segment_color; segment.segment_length];
                    row_chromosome.append(&mut segment);
                }
                row_chromosome
            })
            .collect();
        Self { solution_grid }
    }
}

impl NonogramPuzzle {
    // TODO! Check if raw access "[i]" is more performant that ".get(i)"
    pub fn uniform_cross(
        &self,
        ancestor_1: &NonogramSolution,
        ancestor_2: &NonogramSolution,
        rng: &mut StdRng,
    ) -> (NonogramSolution, NonogramSolution) {
        let mut descendant_1 = Vec::with_capacity(self.rows);
        let mut descendant_2 = Vec::with_capacity(self.rows);

        for i in 0..self.rows {
            if rng.gen_bool(0.5) {
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
        rng: &mut StdRng,
    ) -> (NonogramSolution, NonogramSolution) {
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

    // TODO! from docs: "See also the [Bernoulli] distribution, which may be faster if sampling from the same probability repeatedly."
    // Maybe it's relevant since in each iteration of the evolutive search the probability it's changed once every hundred thousands uses
    pub fn chromosome_mutation(
        &self,
        candidate: &mut NonogramSolution,
        mutation_probability: f64,
        rng: &mut StdRng,
    ) {
        for row_segment_colors in candidate.solution_grid.iter_mut() {
            if rng.gen_bool(mutation_probability) {
                let slidable_segments = Self::get_slidables(row_segment_colors);
                if let Some(&(a, b)) = slidable_segments.choose(rng) {
                    row_segment_colors.swap(a, b);
                }
            }
        }
    }

    pub fn get_slidables(row_segment_colors: &Vec<usize>) -> Vec<(usize, usize)> {
        let mut slidable_segments = Vec::new();

        let mut segment_colors_iter = row_segment_colors.iter().enumerate();

        // We check atleast one element exist
        if let Some((_, previous_segment_color)) = segment_colors_iter.next() {
            let mut previous_segment_color = *previous_segment_color;
            let mut background_end = None;

            // We set a marker in the segment start if needed
            let mut segment_start = if previous_segment_color != 0 {
                Some(0)
            } else {
                None
            };
            for (i, &current_segment_color) in segment_colors_iter {
                match (previous_segment_color, current_segment_color) {
                    // Set the background end and segment start
                    (BACKGROUND, b) if b != BACKGROUND => {
                        background_end = Some(i - 1);
                        segment_start = Some(i);
                    }
                    // Push valid slides of the segment
                    (a, BACKGROUND) if a != BACKGROUND => {
                        if let Some(end) = background_end {
                            slidable_segments.push((end, i - 1 /* segment_a_end */));
                            background_end = None;
                        }

                        slidable_segments.push((
                        segment_start.expect("Couldn't find the segment start, look into setting the segment start and update of it to find the error."),
                        i /* background_start */
                    ));
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
                previous_segment_color = current_segment_color;
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
    use super::*;

    // TODO! Change to use sets so the order doesn't matter
    // Helper function to compare slidables
    fn assert_slidable_positions_equal(actual: Vec<(usize, usize)>, expected: Vec<(usize, usize)>) {
        println!("Actual: {:?}", actual);
        println!("Expected: {:?}", expected);
        assert_eq!(
            actual.len(),
            expected.len(),
            "The number of slidable positions does not match."
        );
        for (a, b) in actual.iter().zip(expected.iter()) {
            assert_eq!(
                a, b,
                "The slidable positions do not match: expected {:?}, but got {:?}",
                a, b
            );
        }
    }

    #[test]
    fn get_slidables_from_empty_row() {
        let row_segment_colors = vec![];
        let result = NonogramPuzzle::get_slidables(&row_segment_colors);
        assert_slidable_positions_equal(result, vec![]);
    }

    #[test]
    fn get_slidables_from_background_row() {
        let row_segment_colors = vec![0, 0, 0, 0, 0];
        let result = NonogramPuzzle::get_slidables(&row_segment_colors);
        assert_slidable_positions_equal(result, vec![]);
    }

    #[test]
    fn get_slidables_from_single_segment() {
        let row_segment_colors = vec![0, 1, 1, 0];
        let result = NonogramPuzzle::get_slidables(&row_segment_colors);
        // The segment can slide in both directions
        assert_slidable_positions_equal(result, vec![(0, 2), (1, 3)]);
    }

    #[test]
    fn get_slidables_from_multiple_segments() {
        let row_segment_colors = vec![0, 1, 1, 0, 2, 2, 0];
        let result = NonogramPuzzle::get_slidables(&row_segment_colors);
        // Both segments can slide in both directions
        assert_slidable_positions_equal(result, vec![(0, 2), (1, 3), (3, 5), (4, 6)]);
    }

    #[test]
    fn get_slidables_from_adjacent_segments() {
        let row_segment_colors = vec![0, 1, 2, 1, 0];
        let result = NonogramPuzzle::get_slidables(&row_segment_colors);
        // Both outer segments can slide once
        assert_slidable_positions_equal(result, vec![(0, 1), (3, 4)]);
    }

    #[test]
    fn get_slidables_from_glued_segments() {
        let row_segment_colors = vec![1, 0, 1, 0, 1];
        let result = NonogramPuzzle::get_slidables(&row_segment_colors);
        // Both outer segments can slide once, the other can slide in both directions
        assert_slidable_positions_equal(result, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    }
}
