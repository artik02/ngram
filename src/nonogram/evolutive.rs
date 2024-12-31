use super::definitions::{NonogramPuzzle, NonogramSolution};
use rand::{rngs::StdRng, seq::SliceRandom, Rng};

type NewPopulation = Vec<NonogramSolution>;
type Population = Vec<(NonogramSolution, usize)>;

pub struct History {
    pub iterations: usize,
    pub best: Vec<usize>,
    pub median: Vec<f64>,
    pub worst: Vec<usize>,
    pub winner: Option<NonogramSolution>,
}

impl History {
    pub fn new() -> Self {
        Self {
            iterations: 0,
            best: Vec::new(),
            median: Vec::new(),
            worst: Vec::new(),
            winner: None,
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
            self.winner = Some(population[0].0.clone());
            return true;
        }
        false
    }
}

/// Applies an evolutive search to minimize the badness score of the solution to a nonogram
pub fn evolutive_search(
    population_size: usize,
    puzzle: &NonogramPuzzle,
    cross_probability: f64,
    mutation_probability: f64,
    tournament_size: usize,
    max_iterations: usize,
    rng: &mut StdRng,
) -> History {
    let mut population = initial_population(puzzle, population_size, rng);
    let mut history = History::new();
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
        mutate_population(puzzle, &mut offspring, mutation_probability, rng);
        // Select best
        population = preserve_elite_population(puzzle, population, offspring);
    }
    history.push(&population);
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
    rng: &mut StdRng,
) {
    offspring
        .iter_mut()
        .for_each(|descendant| puzzle.chromosome_mutation(descendant, mutation_probability, rng));
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
