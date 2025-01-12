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

use super::definitions::{NonogramPuzzle, NonogramSolution};
use dioxus::logger::tracing::info;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

const POPULATION_SIZE: usize = 500;
const CROSS_PROBABILITY: f64 = 0.9;
const MUTATION_PROBABILITY: f64 = 0.1;
const TOURNAMENT_SIZE: usize = 3;
const MAX_ITERATIONS: usize = 300;
const SLIDE_TRIES: usize = 5;

pub fn solve_nonogram(puzzle: NonogramPuzzle) -> History {
    let mut rng = StdRng::from_entropy();
    let history = evolutive_search(
        POPULATION_SIZE,
        &puzzle,
        CROSS_PROBABILITY,
        MUTATION_PROBABILITY,
        TOURNAMENT_SIZE,
        SLIDE_TRIES,
        MAX_ITERATIONS,
        &mut rng,
    );
    match &history.winner {
        Ok(winner) => info!("Nonogram Solution:\n{}", winner),
        Err(approach) => info!(
            "Best score: {}\nBest Solution:\n{}",
            puzzle.score(approach),
            approach
        ),
    }
    history
}

type NewPopulation = Vec<NonogramSolution>;
type Population = Vec<(NonogramSolution, usize)>;

#[derive(Debug, Clone)]
pub struct History {
    pub iterations: usize,
    pub best: Vec<usize>,
    pub median: Vec<f64>,
    pub worst: Vec<usize>,
    pub winner: Result<NonogramSolution, NonogramSolution>,
}

impl History {
    pub fn new(puzzle: &NonogramPuzzle, rng: &mut StdRng) -> Self {
        Self {
            iterations: 0,
            best: Vec::new(),
            median: Vec::new(),
            worst: Vec::new(),
            winner: Err(puzzle.new_chromosome_solution(rng)),
        }
    }

    pub fn push(&mut self, population: &Population) {
        let population_size = population.len();
        self.iterations += 1;
        self.best.push(population[0].1);
        self.median
            .push(Self::get_median(population, population_size));
        self.worst.push(population[population_size - 1].1);
    }

    pub fn get_median(population: &Population, population_size: usize) -> f64 {
        if population_size % 2 == 0 {
            let mid1 = population_size / 2 - 1;
            let mid2 = population_size / 2;
            (population[mid1].1 + population[mid2].1) as f64 / 2.0
        } else {
            let mid = population_size / 2;
            population[mid].1 as f64
        }
    }

    pub fn winner(&mut self, population: &Population) -> bool {
        if population[0].1 == 0 {
            self.winner = Ok(population[0].0.clone());
            return true;
        }
        false
    }

    pub fn loser(&mut self, population: &Population) {
        if self.winner.is_err() {
            self.winner = Err(population[0].0.clone());
        }
    }
}

/// Applies an evolutive search to minimize the score of the solution to a nonogram
pub fn evolutive_search(
    population_size: usize,
    puzzle: &NonogramPuzzle,
    cross_probability: f64,
    mutation_probability: f64,
    tournament_size: usize,
    slide_tries: usize,
    max_iterations: usize,
    rng: &mut StdRng,
) -> History {
    let mut population = initial_population(puzzle, population_size, rng);
    let mut history = History::new(puzzle, rng);
    while history.iterations < max_iterations {
        // Save results
        history.push(&population);
        // Stop criteria
        if history.winner(&population) {
            break;
        }
        // Recombinate
        let mut offspring =
            recombinate_population(puzzle, &population, cross_probability, tournament_size, rng);
        // Mutation
        mutate_population(
            puzzle,
            &mut offspring,
            mutation_probability,
            slide_tries,
            rng,
        );
        // Select best
        population = preserve_elite_population(puzzle, population, offspring);
    }
    history.loser(&population);
    history
}

fn initial_population(
    puzzle: &NonogramPuzzle,
    population_size: usize,
    rng: &mut StdRng,
) -> Population {
    (0..population_size)
        .map(|_| {
            let solution = puzzle.new_chromosome_solution(rng);
            let score = puzzle.score(&solution);
            (solution, score)
        })
        .collect()
}

fn recombinate_population(
    puzzle: &NonogramPuzzle,
    population: &Population,
    cross_probability: f64,
    tournament_size: usize,
    rng: &mut StdRng,
) -> NewPopulation {
    let mut new_population = Vec::with_capacity(population.len());
    while new_population.len() < population.len() {
        let ancestor_1 = tournament_selection(population, tournament_size, rng);
        let ancestor_2 = tournament_selection(population, tournament_size, rng);
        let (descendant_1, descendant_2) = if rng.gen_bool(0.5) {
            puzzle.uniform_cross(ancestor_1, ancestor_2, cross_probability, rng)
        } else {
            puzzle.two_point_cross(ancestor_1, ancestor_2, cross_probability, rng)
        };
        new_population.push(descendant_1);
        new_population.push(descendant_2);
    }
    new_population
}

/// Helper function for tournament selection
fn tournament_selection<'population_scope>(
    population: &'population_scope Population,
    tournament_size: usize,
    rng: &mut StdRng,
) -> &'population_scope NonogramSolution {
    let tournament = population.choose_multiple(rng, tournament_size);
    &tournament
        .into_iter()
        .min_by_key(|&(_, score)| score)
        .expect("The tournament it's empty")
        .0
}

fn mutate_population(
    puzzle: &NonogramPuzzle,
    offspring: &mut NewPopulation,
    mutation_probability: f64,
    slide_tries: usize,
    rng: &mut StdRng,
) {
    offspring.iter_mut().for_each(|descendant| {
        puzzle.chromosome_mutation(descendant, mutation_probability, slide_tries, rng)
    });
}

fn preserve_elite_population(
    puzzle: &NonogramPuzzle,
    population: Population,
    offspring: NewPopulation,
) -> Population {
    let population_size = population.len();
    let mut combined_population: Vec<(NonogramSolution, usize)> = population
        .into_iter()
        .chain(offspring.into_iter().map(|solution| {
            let score = puzzle.score(&solution);
            (solution, score)
        }))
        .collect();
    combined_population.sort_by_key(|(_, score)| *score);
    combined_population.truncate(population_size);
    combined_population
}
