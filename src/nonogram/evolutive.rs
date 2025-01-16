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

// Import necessary definitions
use super::definitions::{NonogramPuzzle, NonogramSolution};

// Import logging and random number generation utilities
use dioxus::logger::tracing::info;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

/// Type alias for a new population, where each element is a `NonogramSolution`.
type NewPopulation = Vec<NonogramSolution>;

/// Type alias for a population consisting of tuples, where each tuple contains a `NonogramSolution`
/// and its corresponding score (fitness value).
type Population = Vec<(NonogramSolution, usize)>;

// Constants for genetic algorithm
/// Defines the population size for the genetic algorithm.
const POPULATION_SIZE: usize = 500;

/// Defines the probability of crossover between individuals.
const CROSS_PROBABILITY: f64 = 0.6;

/// Defines the probability of mutation in the population.
const MUTATION_PROBABILITY: f64 = 0.1;

/// Defines the tournament size used for selection.
const TOURNAMENT_SIZE: usize = 3;

/// Defines the maximum number of iterations for the genetic algorithm.
const MAX_ITERATIONS: usize = 300;

/// Defines the number of tries for sliding window mutations.
const SLIDE_TRIES: usize = 3;

/// Defines the seed value for random number generation.
const SEED: u64 = 23;

/// Performs an Analysis of Variance (ANOVA) approach to optimize a Nonogram puzzle solution
///
/// This function tests various combinations of crossover probabilities, mutation probabilities,
/// sliding window sizes, and random seeds to determine the best set of parameters that yield
/// the optimal solution for a given Nonogram puzzle.
///
/// # Arguments
///
/// * `puzzle` - A `NonogramPuzzle` representing the puzzle to be solved.
///
/// # Constants
///
/// - `ANOVA_POPULATION_SIZE`: Defines the size of the population used in the genetic algorithm.
/// - `ANOVA_TOURNAMENT_SIZE`: Defines the size of the tournament used for selection in the genetic algorithm.
/// - `ANOVA_MAX_ITERATIONS`: Specifies the maximum number of iterations for the evolutionary search.
///
/// # Logic
///
/// The function iterates through all combinations of crossover probabilities, mutation probabilities,
/// slide window sizes, and random seeds. For each combination:
/// - A random number generator is seeded with the chosen seed.
/// - The `evolutive_search` function is called to perform the search with the given parameters.
/// - After obtaining scores, it compares them to determine the best set of parameters.
///
/// # Returns
///
/// Logs the best score and its corresponding parameters or indicates that no valid combination was found.
pub fn anova(puzzle: NonogramPuzzle) {
    let cross_probabilities = vec![0.3, 0.6, 0.9];
    let mutation_probabilities = vec![0.1, 0.2, 0.3];
    let slides = vec![3, 5, 7];
    let seeds = vec![11, 13, 17, 19, 23, 29, 31, 37, 41, 43];
    const ANOVA_POPULATION_SIZE: usize = 500;
    const ANOVA_TOURNAMENT_SIZE: usize = 3;
    const ANOVA_MAX_ITERATIONS: usize = 300;

    let mut best_score = usize::MAX;
    let mut best_parameters = None;

    // Iterate over all combinations of parameters
    for &cross_probability in &cross_probabilities {
        for &mutation_probability in &mutation_probabilities {
            for &slide_tries in &slides {
                for &seed in &seeds {
                    let mut rng = StdRng::seed_from_u64(seed);
                    info!(
                        "Testing parameters: cross_prob = {}, mut_prob = {}, slide_tries = {}, seed = {}...",
                        cross_probability, mutation_probability, slide_tries, seed
                    );

                    // Perform evolutionary search with the given parameters
                    let history = evolutive_search(
                        ANOVA_POPULATION_SIZE,
                        &puzzle,
                        cross_probability,
                        mutation_probability,
                        ANOVA_TOURNAMENT_SIZE,
                        slide_tries,
                        ANOVA_MAX_ITERATIONS,
                        &mut rng,
                    );

                    info!("Obtained a score of: {}", history.best.last().unwrap());

                    // Update the best score and parameters if the current score is better
                    if let Some(&current_best) = history.best.last() {
                        if current_best < best_score {
                            best_score = current_best;
                            best_parameters = Some((
                                ANOVA_POPULATION_SIZE,
                                cross_probability,
                                mutation_probability,
                                ANOVA_TOURNAMENT_SIZE,
                                slide_tries,
                                ANOVA_MAX_ITERATIONS,
                                seed,
                            ));
                        }
                    }
                }
            }
        }
    }

    // Log the best parameters if found
    if let Some(parameters) = best_parameters {
        info!(
            "The best score was {} with the parameters: {:?}",
            best_score, parameters
        );
    } else {
        info!("A valid combination wasn't found");
    }
}

/// Solves a Nonogram puzzle using a genetic algorithm approach.
///
/// This function initializes a random number generator seeded with a fixed value and then
/// performs an evolutionary search to solve the provided `NonogramPuzzle`. The resulting
/// solution or best score is logged, and the history of the solution process is returned.
///
/// # Arguments
///
/// * `puzzle` - A `NonogramPuzzle` instance that represents the puzzle to be solved.
///
/// # Constants
///
/// - `SEED`: The seed used to initialize the random number generator.
/// - `POPULATION_SIZE`: The size of the population used in the genetic algorithm.
/// - `CROSS_PROBABILITY`: The probability of crossover between individuals.
/// - `MUTATION_PROBABILITY`: The probability of mutation applied to the population.
/// - `TOURNAMENT_SIZE`: The size of the tournament used for selection in the genetic algorithm.
/// - `SLIDE_TRIES`: Number of attempts for sliding window mutations.
/// - `MAX_ITERATIONS`: The maximum number of iterations for the evolutionary search.
///
/// # Returns
///
/// A `History` object containing the best solution or best scores from the evolutionary search.
///
/// # Example
///
/// ```rust
/// let puzzle = NonogramPuzzle::new(...);
/// let history = solve_nonogram(puzzle);
/// ```
pub fn solve_nonogram(puzzle: NonogramPuzzle) -> History {
    let mut rng = StdRng::seed_from_u64(SEED);
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

/// A struct representing the evolutionary search history.
///
/// `History` tracks the progress of the genetic algorithm, including the number of iterations,
/// best, median, and worst scores, and the final solution (either the best solution or the worst
/// approach if no optimal solution is found).
///
/// # Fields
///
/// - `iterations`: Number of iterations completed in the evolutionary search.
/// - `best`: A vector of best scores at each iteration.
/// - `median`: A vector of median scores at each iteration.
/// - `worst`: A vector of worst scores at each iteration.
/// - `winner`: A result containing either the best solution (`Ok`) or the worst approach (`Err`).
///
/// # Methods
///
/// - `new(puzzle: &NonogramPuzzle, rng: &mut StdRng) -> Self`: Initializes a new `History` object.
/// - `push(&mut self, population: &Population)`: Adds a new population's scores to the history.
/// - `get_median(population: &Population, population_size: usize) -> f64`: Calculates the median score
///   from the given population.
/// - `winner(&mut self, population: &Population) -> bool`: Checks if the best score in the current
///   population is 0 and sets the best solution as the winner.
/// - `loser(&mut self, population: &Population)`: Sets the worst approach as the winner if no optimal
///   solution was found.
#[derive(Debug, Clone)]
pub struct History {
    pub iterations: usize,
    pub best: Vec<usize>,
    pub median: Vec<f64>,
    pub worst: Vec<usize>,
    pub winner: Result<NonogramSolution, NonogramSolution>,
}

impl History {
    /// Creates a new `History` instance for a given `NonogramPuzzle` and a random number generator.
    ///
    /// # Arguments
    ///
    /// * `puzzle` - A reference to the `NonogramPuzzle` to be solved.
    /// * `rng` - A mutable reference to a `StdRng` for random operations during initialization.
    ///
    /// # Returns
    ///
    /// Returns a new `History` struct initialized with empty vectors for scores and a placeholder
    /// initial solution (`Err` variant of `winner`).
    pub fn new(puzzle: &NonogramPuzzle, rng: &mut StdRng) -> Self {
        Self {
            iterations: 0,
            best: Vec::new(),
            median: Vec::new(),
            worst: Vec::new(),
            winner: Err(puzzle.new_chromosome_solution(rng)),
        }
    }

    /// Updates the history with the latest population's scores.
    ///
    /// # Arguments
    ///
    /// * `population` - A reference to the current population of solutions with their respective scores.
    pub fn push(&mut self, population: &Population) {
        let population_size = population.len();
        self.iterations += 1;
        self.best.push(population[0].1);
        self.median
            .push(Self::get_median(population, population_size));
        self.worst.push(population[population_size - 1].1);
    }

    /// Calculates the median score from the population.
    ///
    /// # Arguments
    ///
    /// * `population` - A reference to the current population of solutions with scores.
    /// * `population_size` - The size of the population (number of solutions).
    ///
    /// # Returns
    ///
    /// Returns the median score as a floating-point value.
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

    /// Checks if the best score in the current population is 0 and sets the solution as the winner.
    ///
    /// # Arguments
    ///
    /// * `population` - A reference to the current population of solutions with scores.
    ///
    /// # Returns
    ///
    /// Returns `true` if the best score is 0 (indicating a winning solution), otherwise `false`.
    pub fn winner(&mut self, population: &Population) -> bool {
        if population[0].1 == 0 {
            self.winner = Ok(population[0].0.clone());
            return true;
        }
        false
    }

    /// Sets the worst approach as the winner if no optimal solution has been found.
    ///
    /// # Arguments
    ///
    /// * `population` - A reference to the current population of solutions with scores.
    pub fn loser(&mut self, population: &Population) {
        if self.winner.is_err() {
            self.winner = Err(population[0].0.clone());
        }
    }
}

/// Applies an evolutionary search (evolutive search) to minimize the score of the solution
/// to a Nonogram puzzle using genetic algorithm techniques.
///
/// This function facilitates the optimization process by simulating the natural evolution of
/// solutions over multiple generations. The goal is to iteratively improve the population
/// of candidate solutions until an optimal solution is found or the maximum number of iterations
/// is reached.
///
/// # Arguments
///
/// * `population_size` - The size of the initial population of solutions.
/// * `puzzle` - A reference to the `NonogramPuzzle` instance that represents the puzzle to be solved.
/// * `cross_probability` - The probability of performing crossover between pairs of solutions.
/// * `mutation_probability` - The probability of applying mutation to solutions in the population.
/// * `tournament_size` - The size of the tournament used for selection during reproduction.
/// * `slide_tries` - The number of tries for applying sliding mutations.
/// * `max_iterations` - The maximum number of generations (iterations) the evolutionary search will run.
/// * `rng` - A mutable reference to the `StdRng` used for generating random values during mutation, crossover, and selection processes.
///
/// # Returns
///
/// Returns a `History` object containing the progress of the evolutionary search, which includes
/// information about the best, median, and worst scores at each generation, as well as the final solution.
///
/// # Process
///
/// 1. **Initialization**: The function begins by creating an initial population of candidate solutions
///    using the `initial_population` function.
///
/// 2. **Evolutionary Loop**:
///    - Iteratively updates the population by applying selection, recombination, and mutation processes.
///    - For each generation (or iteration), the function saves the best, median, and worst scores to the history.
///    - It checks for a winning solution (i.e., a solution with a score of 0), and if found, terminates the loop.
///    - If no optimal solution is found, a loser (best non-optimal solution) is selected after reaching the
///      maximum number of iterations.
///
/// 3. **Selection and Preservation**: At each step, the best solutions are preserved while weaker ones are discarded.
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

/// Generates the initial population for solving a Nonogram puzzle using a genetic algorithm.
///
/// This function creates an initial population of chromosomes, where each chromosome
/// represents a potential solution to the puzzle. The function generates random solutions
/// and calculates their scores using the provided Nonogram puzzle. The resulting population
/// is returned as a collection of tuples, each containing a solution and its corresponding score.
///
/// # Arguments
///
/// * `puzzle` - A reference to a `NonogramPuzzle` representing the puzzle to be solved.
/// * `population_size` - The desired size of the initial population.
/// * `rng` - A mutable reference to a `StdRng` for generating random solutions.
///
/// # Returns
///
/// A `Population`, which is a collection of tuples containing a solution and its score.
fn initial_population(
    puzzle: &NonogramPuzzle,
    population_size: usize,
    rng: &mut StdRng,
) -> Population {
    (0..population_size)
        .map(|_| {
            let solution = puzzle.new_chromosome_solution(rng); // Generate a new random solution
            let score = puzzle.score(&solution); // Calculate the score of the solution
            (solution, score) // Return solution and its score as a tuple
        })
        .collect()
}

/// Generates a new population through recombination (crossover) of the given population.
///
/// This function performs tournament selection to pick parent chromosomes from the current
/// population, and then applies either uniform or two-point crossover (50 percent of the time for each)
/// to create offspring. The resulting children are added to a new population until the desired size is reached.
///
/// # Arguments
///
/// * `puzzle` - A reference to a `NonogramPuzzle` instance used for crossover operations.
/// * `population` - A reference to the current population, a collection of solutions and scores.
/// * `cross_probability` - The probability that crossover will occur between selected parents.
/// * `tournament_size` - The number of individuals participating in the tournament selection.
/// * `rng` - A mutable reference to a `StdRng` used for generating random decisions and solutions.
///
/// # Returns
///
/// A `NewPopulation`, which is a collection of offspring solutions produced through recombination.
fn recombinate_population(
    puzzle: &NonogramPuzzle,
    population: &Population,
    cross_probability: f64,
    tournament_size: usize,
    rng: &mut StdRng,
) -> NewPopulation {
    let mut new_population = Vec::with_capacity(population.len());
    while new_population.len() < population.len() {
        let ancestor_1 = tournament_selection(population, tournament_size, rng); // Select first parent
        let ancestor_2 = tournament_selection(population, tournament_size, rng); // Select second parent
        let (descendant_1, descendant_2) = if rng.gen_bool(0.5) {
            puzzle.uniform_cross(ancestor_1, ancestor_2, cross_probability, rng)
        // Apply uniform crossover
        } else {
            puzzle.two_point_cross(ancestor_1, ancestor_2, cross_probability, rng)
            // Apply two-point crossover
        };
        new_population.push(descendant_1); // Add first child to the new population
        new_population.push(descendant_2); // Add second child to the new population
    }
    new_population
}

/// Selects a single individual from the population using a tournament selection method.
///
/// Tournament selection involves randomly selecting a subset of individuals from the population,
/// and then choosing the best (lowest score) individual from this subset. The chosen individual
/// represents one potential parent for recombination. This approach helps balance exploration
/// and exploitation of the search space by allowing diverse candidates to compete for selection.
///
/// # Arguments
///
/// * `population` - A reference to the current population, which is a collection of solution-score pairs.
/// * `tournament_size` - The number of individuals selected from the population to participate in the tournament.
/// * `rng` - A mutable reference to a `StdRng`, used to randomly select individuals for the tournament.
///
/// # Returns
///
/// A reference to the selected `NonogramSolution` with the best score from the tournament subset.
///
/// # Panics
///
/// This function panics if the tournament subset is empty, ensuring a valid solution exists.
fn tournament_selection<'population_scope>(
    population: &'population_scope Population,
    tournament_size: usize,
    rng: &mut StdRng,
) -> &'population_scope NonogramSolution {
    let tournament = population.choose_multiple(rng, tournament_size); // Select a subset from the population
    &tournament
        .into_iter()
        .min_by_key(|&(_, score)| score) // Choose the individual with the best score
        .expect("The tournament is empty") // Ensure the tournament isn't empty
        .0 // Return the selected solution
}

/// Applies mutations to the population by modifying chromosomes based on a given probability.
///
/// This function iterates over each individual in the `offspring` population and applies mutations
/// using the `chromosome_mutation` method from the `NonogramPuzzle` trait. The mutation alters
/// the chromosomes by making small changes, such as flipping bits or sliding values, to potentially
/// improve their scores. The mutation process is controlled by a specified probability and
/// a fixed number of slide tries.
///
/// # Arguments
///
/// * `puzzle` - A reference to a `NonogramPuzzle` instance used to perform mutations on chromosomes.
/// * `offspring` - A mutable reference to a collection of mutated solution chromosomes.
/// * `mutation_probability` - The probability of applying mutation to each individual in the population.
/// * `slide_tries` - The number of attempts to apply sliding mutations.
/// * `rng` - A mutable reference to a `StdRng`, used for generating random mutations.
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

/// Combines the current population with offspring solutions and preserves only the top solutions.
///
/// This function creates a combined population by merging the existing `population` with
/// offspring solutions. Each solution is scored using the provided `puzzle`, and the combined
/// population is sorted based on scores in ascending order. To maintain the desired population
/// size, the lowest-scoring individuals are then removed, ensuring the original population size
/// is preserved.
///
/// # Arguments
///
/// * `puzzle` - A reference to a `NonogramPuzzle` instance used to evaluate the fitness of solutions.
/// * `population` - The current population of solutions represented as a vector of solution-score pairs.
/// * `offspring` - The new population of solutions generated from recombination, which also includes their scores.
///
/// # Returns
///
/// A reduced `Population` containing only the top-performing solutions, preserving the original size.
///
/// # Note
///
/// The function truncates the combined population to ensure only the top `population_size` solutions are retained.
fn preserve_elite_population(
    puzzle: &NonogramPuzzle,
    population: Population,
    offspring: NewPopulation,
) -> Population {
    let population_size = population.len(); // Determine the size of the population
    let mut combined_population: Vec<(NonogramSolution, usize)> = population
        .into_iter()
        .chain(offspring.into_iter().map(|solution| {
            let score = puzzle.score(&solution); // Calculate the score for offspring solutions
            (solution, score) // Pair solution with its score
        }))
        .collect();
    combined_population.sort_by_key(|(_, score)| *score); // Sort by scores in ascending order
    combined_population.truncate(population_size); // Retain only the top-performing solutions
    combined_population
}
