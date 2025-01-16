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

// Import necessary definitions for working with Nonogram puzzles and solutions.
use super::definitions::{NonogramFile, NonogramPuzzle, NonogramSolution, DEFAULT_PALETTE};

// Import the `History` structure from the `evolutive` module for tracking evolution-related data.
use super::evolutive::History;

// Import specific definitions from the Nonogram module to manage Nonogram data and palettes.
use crate::nonogram::definitions::{NonogramData, NonogramPalette};

// Import functions from the Nonogram evolutive module for solving puzzles and statistical analysis.
use crate::nonogram::evolutive::{anova, solve_nonogram};

// Import predefined puzzles from the Nonogram puzzles module for creating or managing puzzles.
use crate::nonogram::puzzles::*;

// Import Dioxus libraries for UI rendering and logging, allowing asynchronous and reactive UI components.
use dioxus::{
    logger::tracing::{error, info},
    prelude::*,
};

// Import mouse button data from Dioxus elements to handle input events.
use dioxus_elements::input_data::MouseButton;

// Import icons from `dioxus_free_icons` for displaying Font Awesome solid icons in the UI.
use dioxus_free_icons::icons::fa_solid_icons::{
    FaArrowDown, FaArrowLeft, FaArrowRight, FaArrowUp, FaDeleteLeft, FaPlus,
};

// Import the `Icon` struct from `dioxus_free_icons` for easily managing and displaying icons.
use dioxus_free_icons::Icon;

// Import internationalization support from `dioxus_i18n` to handle translations within the UI.
use dioxus_i18n::t;

// Import random number generation utilities from the `rand` crate to provide randomness in solving Nonograms.
use rand::{rngs::StdRng, Rng, SeedableRng};

/// The main component for the Nonogram Solver page.
///
/// This component initializes various contexts and providers for handling a Nonogram puzzle.
/// It sets up contexts for the puzzle, palette, solution, score state, and history. Additionally,
/// it renders a user interface with tools like the toolbar, nonogram display, and graphical solution.
///
/// # Context Initialization:
/// - `tree_nonogram_puzzle()`: Initializes the Nonogram puzzle.
/// - `tree_nonogram_palette()`: Initializes the color palette for the Nonogram.
/// - `tree_empty_nonogram_solution()`: Initializes an empty Nonogram solution grid.
/// - `tree_nonogram_file()`: Initializes a preview Nonogram file.
/// - `tree_nonogram_puzzle().score(&tree_nonogram_file().solution)`: Sets up the Nonogram score state.
/// - `History::new(&tree_nonogram_puzzle(), &mut StdRng::from_entropy())`: Initializes Nonogram history with a random number generator.
/// - `NonogramData`: Stores Nonogram editor data such as filename and block size.
///
/// # UI Rendering:
/// - The component returns a structured layout with various UI elements including a toolbar, nonogram display,
///   and solution visualizations.
#[component]
pub fn Solver() -> Element {
    std::panic::set_hook(Box::new(|info| {
        error!("Panic: {}", info);
    }));
    use_context_provider(|| {
        info!("Initializing nonogram puzzle");
        Signal::new(tree_nonogram_puzzle())
    });
    use_context_provider(|| {
        info!("Initializing nonogram palette");
        Signal::new(tree_nonogram_palette())
    });
    use_context_provider(|| {
        info!("Initializing empty nonogram solution");
        Signal::new(tree_empty_nonogram_solution())
    });
    use_context_provider(|| {
        info!("Initializing nonogram file for preview");
        Signal::new(tree_nonogram_file())
    });
    use_context_provider(|| {
        info!("Initializing nonogram score state");
        Signal::new(tree_nonogram_puzzle().score(&tree_nonogram_file().solution))
    });
    use_context_provider(|| {
        info!("Initializing nonogram history");
        let mut rng = StdRng::from_entropy();
        Signal::new(History::new(&tree_nonogram_puzzle(), &mut rng))
    });
    use_context_provider(|| {
        info!("Initializing nonogram editor state");
        Signal::new(NonogramData {
            filename: String::from("tree.ngram"),
            block_size: 30,
            completed: false,
        })
    });

    rsx! {
        main { class: "flex flex-col gap-10 items-center min-h-screen mb-20",
            h1 { class: "text-4xl font-bold my-10 text-center", {t!("title_nonogram_solver")} }
            SolverToolbar {}
            SolverNonogram {}
            ConvergeGraphic {}
        }
    }
}

/// A toolbar component for the Nonogram Solver.
///
/// This component contains various controls and input fields used to manage the Nonogram puzzle.
/// It includes inputs for row and column configurations, file loading, solving, and editing options.
///
/// # UI Elements:
/// - `RowsInput`: Read-only input for row configuration.
/// - `ColumnsInput`: Read-only input for column configuration.
/// - `BlockSizeInput`: Input for adjusting the size of blocks in the Nonogram.
/// - `FileLoadInput`: Input for loading Nonogram puzzle files.
/// - `SolveButton`: Button to solve the Nonogram puzzle.
/// - `AnovaButton`: Button to perform Anova analysis on the puzzle.
/// - `ClearSolutionButton`: Button to clear the current solution.
/// - `SlideSolutionButtons`: Buttons to navigate through possible solutions.
/// - `ColorPalette`: Displays the color palette used in the Nonogram.
#[component]
fn SolverToolbar() -> Element {
    rsx! {
        section { class: "container flex flex-col space-y-6 p-6 rounded-lg shadow-lg bg-gray-900",
            div { class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-6",
                RowsInput { readonly: true }
                ColumnsInput { readonly: true }
                BlockSizeInput {}
            }
            div { class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-6",
                FileLoadInput {}
                SolveButton {}
                AnovaButton {}
            }
            div { class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-6",
                ClearSolutionButton {}
                SlideSolutionButtons {}
            }
            div { class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-6",
                ColorPalette { readonly: true }
            }
        }
    }
}

/// Displays the Nonogram puzzle and its solution.
///
/// This component manages the display of the Nonogram puzzle grid alongside its constraints and solution.
/// It updates the solution state and compares it with the puzzle to check if it is completed.
///
/// # Contexts Used:
/// - `Signal<NonogramPuzzle>`: Provides the current state of the puzzle.
/// - `Signal<NonogramSolution>`: Provides the current state of the solution.
/// - `Signal<NonogramData>`: Manages Nonogram-related data including completion state.
///
/// # UI Elements:
/// - `RowsConstraints`: Displays row constraints of the puzzle.
/// - `ColumnsConstraints`: Displays column constraints of the puzzle.
/// - `SolutionPreview`: Shows a preview of the solution.
/// - `Solution`: Displays the solution grid.
#[component]
fn SolverNonogram() -> Element {
    let use_puzzle = use_context::<Signal<NonogramPuzzle>>();
    let use_solution = use_context::<Signal<NonogramSolution>>();
    let mut use_data = use_context::<Signal<NonogramData>>();
    use_effect(move || {
        let current_puzzle = NonogramPuzzle::from_solution(&use_solution());
        use_data.write().completed = use_puzzle() == current_puzzle;
    });
    rsx! {
        section { class: "mb-20",
            if use_data().completed {
                h2 { class: "text-6xl font-bold my-10 text-center", {t!("completed")} }
            }
            table { class: "border-separate border-spacing-4",
                thead {
                    tr { class: "align-baseline",
                        th { class: "h-full align-bottom flex justify-end", SolutionPreview {} }
                        th { class: "align-bottom",
                            ColumnsConstraints { puzzle: use_puzzle() }
                        }
                    }
                }
                tbody {
                    tr {
                        th { class: "flex justify-end",
                            RowsConstraints { puzzle: use_puzzle() }
                        }
                        td { Solution {} }
                    }
                }
            }
        }
    }
}

/// The main component for the Nonogram Editor page.
///
/// This component initializes contexts necessary for editing a Nonogram puzzle.
/// It sets up contexts for the palette, solution, puzzle, and editor-specific data.
/// The editor allows users to modify the solution directly, unlike the `Solver` component,
/// which focuses solely on displaying solutions.
///
/// # Context Initialization:
/// - `tree_nonogram_palette()`: Initializes the color palette for editing the Nonogram.
/// - `tree_empty_nonogram_solution()`: Initializes an empty Nonogram solution for editing.
/// - `tree_nonogram_puzzle()`: Sets up the Nonogram puzzle.
/// - `NonogramData`: Manages the state of the Nonogram editor including filename, block size, and completion status.
///
/// # UI Rendering:
/// - The component renders a structured layout with a toolbar and a Nonogram grid, allowing users to edit and visualize solutions.
///
/// # Example
/// ```rust
/// Editor {}
/// ```
#[component]
pub fn Editor() -> Element {
    std::panic::set_hook(Box::new(|info| {
        error!("Panic: {}", info);
    }));
    use_context_provider(|| {
        info!("Initializing nonogram palette");
        Signal::new(tree_nonogram_palette())
    });
    use_context_provider(|| {
        info!("Initializing empty nonogram solution");
        Signal::new(tree_empty_nonogram_solution())
    });
    // TODO: Split Solution component into edit and solution modes
    use_context_provider(|| {
        // Unused in Editor
        Signal::new(0usize)
    });
    use_context_provider(|| {
        // Unused in Editor
        Signal::new(tree_nonogram_puzzle())
    });
    use_context_provider(|| {
        info!("Initializing nonogram editor state");
        Signal::new(NonogramData {
            filename: String::new(),
            block_size: 30,
            completed: false,
        })
    });

    rsx! {
        main { class: "flex flex-col gap-10 items-center min-h-screen mb-20",
            h1 { class: "text-4xl font-bold my-10 text-center", {t!("title_nonogram_editor")} }
            EditorToolbar {}
            EditorNonogram {}
        }
    }
}

/// A toolbar component for the Nonogram Editor.
///
/// This component provides various controls and input fields for editing the Nonogram puzzle.
/// It allows users to modify the Nonogram structure and manage file operations such as loading
/// and saving files.
///
/// # UI Elements:
/// - `RowsInput`: Input for row configuration with editing capabilities.
/// - `ColumnsInput`: Input for column configuration with editing capabilities.
/// - `BlockSizeInput`: Input for adjusting the block size.
/// - `FileInput`: Input for loading Nonogram files.
/// - `FileSaveButton`: Button for saving the current Nonogram.
/// - `FileLoadEditInput`: Input for editing the Nonogram by loading from a file.
/// - `ClearSolutionButton`: Button to clear the current solution.
/// - `SlideSolutionButtons`: Buttons for navigating through solutions.
/// - `NewColorButton`: Button to add new colors to the palette.
/// - `ColorPalette`: Displays and allows modification of the color palette.
///
/// # Example
/// ```rust
/// EditorToolbar {}
/// ```
#[component]
fn EditorToolbar() -> Element {
    rsx! {
        section { class: "container flex flex-col space-y-6 p-6 rounded-lg shadow-lg bg-gray-900",
            div { class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-6",
                RowsInput { readonly: false }
                ColumnsInput { readonly: false }
                BlockSizeInput {}
            }
            div { class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-6",
                FileInput { readonly: false }
                FileSaveButton {}
            }
            div { class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-6",
                FileLoadEditInput {}
            }
            div { class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-6",
                ClearSolutionButton {}
                SlideSolutionButtons {}
                NewColorButton {}
            }
            div { class: "flex flex-wrap justify-items-center justify-center items-center gap-6",
                ColorPalette { readonly: false }
            }
        }
    }
}

/// Allows editing the Nonogram puzzle solution.
///
/// This component manages the display and interaction for editing a Nonogram puzzle.
/// It provides a grid layout where users can directly edit the solution, including colors
/// and constraints for rows and columns.
///
/// # Contexts Used:
/// - `Signal<NonogramSolution>`: Provides the current state of the Nonogram solution for editing.
///
/// # UI Elements:
/// - `ColorInput`: Allows users to edit the color used in the Nonogram.
/// - `ColumnsConstraints`: Displays column constraints for the puzzle.
/// - `RowsConstraints`: Displays row constraints for the puzzle.
/// - `Solution`: Provides the solution grid for direct editing.
///
/// # Example
/// ```rust
/// EditorNonogram {}
/// ```
#[component]
fn EditorNonogram() -> Element {
    let use_solution = use_context::<Signal<NonogramSolution>>();
    let current_puzzle = NonogramPuzzle::from_solution(&use_solution());
    rsx! {
        section { class: "mb-20",
            table { class: "border-separate border-spacing-4",
                thead {
                    tr {
                        th { class: "align-bottom flex justify-end", ColorInput {} }
                        th { class: "align-bottom",
                            ColumnsConstraints { puzzle: current_puzzle.clone() }
                        }
                    }
                }
                tbody {
                    tr {
                        th { class: "flex justify-end",
                            RowsConstraints { puzzle: current_puzzle }
                        }
                        td { Solution {} }
                    }
                }
            }
        }
    }
}

/// A component for inputting the number of rows in the Nonogram solution.
///
/// This component allows the user to set the number of rows for the Nonogram puzzle.
/// It validates the input to ensure it is within a reasonable range (2 to 40) and updates the Nonogram solution.
///
/// # Parameters:
/// - `readonly`: A boolean flag to indicate whether the input field should be read-only.
///
/// # Context:
/// - `Signal<NonogramSolution>`: Provides access to and updates for the current Nonogram solution.
#[component]
fn RowsInput(readonly: bool) -> Element {
    let mut use_solution = use_context::<Signal<NonogramSolution>>();
    rsx! {
        div { class: "flex flex-row justify-items-center justify-center items-center gap-3",
            label {
                r#for: "rows-input",
                class: "py-2 text-gray-200 font-semibold cursor-pointer select-none",
                pointer_events: if readonly { "none" },
                color: if readonly { "darkgray" },
                {t!("label_rows")}
                ":"
            }
            input {
                id: "rows-input",
                class: "appearance-none px-4 py-1 w-20 rounded border border-gray-500 bg-gray-800 text-white hover:bg-blue-800 hover:scale-110 active:scale-125 focus:ring focus:ring-blue-500 focus:outline-none transition-transform transform",
                pointer_events: if readonly { "none" },
                color: if readonly { "darkgray" },
                readonly,
                r#type: "number",
                min: "2",
                max: "40",
                onchange: move |event| {
                    if let Ok(rows) = event.value().parse::<usize>() {
                        if (2..=40).contains(&rows) {
                            use_solution.write().set_rows(rows);
                        }
                    }
                },
                value: use_solution().rows(),
            }
        }
    }
}

/// A component for inputting the number of columns in the Nonogram solution.
///
/// This component allows the user to set the number of columns for the Nonogram puzzle.
/// It validates the input to ensure it is within a reasonable range (2 to 40) and updates the Nonogram solution.
///
/// # Parameters:
/// - `readonly`: A boolean flag to indicate whether the input field should be read-only.
///
/// # Context:
/// - `Signal<NonogramSolution>`: Provides access to and updates for the current Nonogram solution.
#[component]
fn ColumnsInput(readonly: bool) -> Element {
    let mut use_solution = use_context::<Signal<NonogramSolution>>();
    rsx! {
        div { class: "flex flex-row justify-items-center justify-center items-center gap-3",
            label {
                r#for: "cols-input",
                class: "py-2 text-gray-200 font-semibold cursor-pointer select-none",
                pointer_events: if readonly { "none" },
                color: if readonly { "darkgray" },
                {t!("label_columns")}
                ":"
            }
            input {
                id: "cols-input",
                class: "appearance-none px-4 py-1 w-20 rounded border border-gray-500 bg-gray-800 text-white hover:bg-blue-800 hover:scale-110 active:scale-125 focus:ring focus:ring-blue-500 focus:outline-none transition-transform transform",
                pointer_events: if readonly { "none" },
                color: if readonly { "darkgray" },
                readonly,
                r#type: "number",
                min: "2",
                max: "40",
                onchange: move |event: FormEvent| {
                    if let Ok(cols) = event.value().parse::<usize>() {
                        if (2..=40).contains(&cols) {
                            use_solution.write().set_cols(cols);
                        }
                    }
                },
                value: use_solution().cols(),
            }
        }
    }
}

/// A component for inputting the block size of the Nonogram grid.
///
/// This component allows the user to set the block size used in the Nonogram puzzle grid.
/// The value is validated within a reasonable range (10 to 100) and updates the Nonogram data accordingly.
///
/// # Context:
/// - `Signal<NonogramData>`: Provides access to and updates for the Nonogram editor state, including block size.
#[component]
fn BlockSizeInput() -> Element {
    let mut use_data = use_context::<Signal<NonogramData>>();
    rsx! {
        div { class: "flex flex-row justify-items-center justify-center items-center gap-3",
            label {
                r#for: "size-input",
                class: "py-2 text-gray-200 font-semibold cursor-pointer select-none",
                {t!("label_size")}
                ":"
            }
            input {
                id: "size-input",
                class: "appearance-none px-4 py-1 w-20 rounded border border-gray-500 bg-gray-800 text-white hover:bg-blue-800 hover:scale-110 active:scale-125 focus:ring focus:ring-blue-500 focus:outline-none transition-transform transform",
                r#type: "number",
                min: "10",
                max: "100",
                step: "5",
                value: use_data().block_size,
                onchange: move |event| {
                    if let Ok(size) = event.value().parse::<usize>() {
                        if (10..=100).contains(&size) {
                            use_data.write().block_size = size;
                        }
                    }
                },
            }
        }
    }
}

/// A button component for solving the Nonogram puzzle.
///
/// This component initiates the process of solving the Nonogram puzzle by running a solution algorithm.
/// It updates the Nonogram solution based on the result and handles a loading state during the process.
///
/// # Context:
/// - `Signal<NonogramPuzzle>`: Provides access to the current Nonogram puzzle.
/// - `Signal<History>`: Updates the history of Nonogram solving attempts.
/// - `Signal<NonogramSolution>`: Updates the Nonogram solution based on the solving result.
#[component]
fn SolveButton() -> Element {
    let use_puzzle = use_context::<Signal<NonogramPuzzle>>();
    let mut use_history = use_context::<Signal<History>>();
    let mut use_solution = use_context::<Signal<NonogramSolution>>();
    let mut use_running = use_signal(|| false);
    rsx! {
        button {
            class: "px-4 py-1 font-bold rounded border border-gray-500 bg-gray-800 text-white hover:bg-blue-800 hover:scale-110 active:scale-125 transition-transform transform",
            onmousedown: move |_| {},
            onclick: move |_| async move {
                if use_running() {
                    info!("Already solving nonogram!");
                } else {
                    *use_running.write() = true;
                    info!("Solving nonogram...");
                    let history = solve_nonogram(use_puzzle().clone());
                    match &history.winner {
                        Ok(winner) => {
                            *use_solution.write() = winner.clone();
                            info!("Nonogram solved!");
                        }
                        Err(loser) => {
                            *use_solution.write() = loser.clone();
                            info!("Nonogram not solved!");
                        }
                    }
                    *use_history.write() = history;
                    *use_running.write() = false;
                }
            },
            {t!("button_solve_nonogram")}
        }
    }
}

/// A button component for testing ANOVA on the Nonogram puzzle.
///
/// This component calls the ANOVA test for the Nonogram puzzle, analyzing possible parameter configurations.
/// It provides feedback on the completion of the test and handles a loading state during the process.
///
/// # Context:
/// - `Signal<NonogramPuzzle>`: Provides access to the current Nonogram puzzle.
#[component]
fn AnovaButton() -> Element {
    let use_puzzle = use_context::<Signal<NonogramPuzzle>>();
    let mut use_running = use_signal(|| false);
    rsx! {
        button {
            class: "px-4 py-1 font-bold rounded border border-gray-500 bg-gray-800 text-white hover:bg-blue-800 hover:scale-110 active:scale-125 transition-transform transform",
            onmousedown: move |_| {},
            onclick: move |_| async move {
                if use_running() {
                    info!("Already testing ANOVA!");
                } else {
                    *use_running.write() = true;
                    info!("Testing ANOVA...");
                    anova(use_puzzle().clone());
                    info!("Finished testing ANOVA!");
                    *use_running.write() = false;
                }
            },
            {t!("button_anova")}
        }
    }
}

/// A button component for clearing the Nonogram solution grid.
///
/// This component clears the current Nonogram solution grid and provides feedback on the action,
/// with support for control and shift key modifiers to perform the action.
///
/// # Context:
/// - `Signal<NonogramSolution>`: Provides access to and updates the Nonogram solution.
#[component]
fn ClearSolutionButton() -> Element {
    let mut use_solution = use_context::<Signal<NonogramSolution>>();
    rsx! {
        button {
            class: "flex justify-center items-center w-10 h-10 rounded-full border border-gray-400 bg-gray-700 hover:bg-red-800 hover:scale-125 active:scale-150 transition-transform transform",
            ondoubleclick: move |_| {
                use_solution.write().clear();
                info!("Cleared the nonogram solution grid");
            },
            onmousedown: move |event| {
                if event.modifiers().ctrl() || event.modifiers().shift() {
                    use_solution.write().clear();
                    info!("Cleared the nonogram solution grid");
                }
            },
            Icon {
                class: "w-11/12 h-11/12",
                fill: "rgb(156, 163, 175)",
                icon: FaDeleteLeft,
            }
        }
    }
}

/// A component with buttons to slide the Nonogram solution grid in four directions.
///
/// This component provides buttons to slide the Nonogram solution grid left, right, up, or down.
/// The buttons are disabled if the puzzle is marked as completed.
///
/// # Context:
/// - `Signal<NonogramSolution>`: Updates the Nonogram solution.
/// - `Signal<NonogramData>`: Uses the information on whether the puzzle is completed or not.
#[component]
fn SlideSolutionButtons() -> Element {
    let mut use_solution = use_context::<Signal<NonogramSolution>>();
    let use_data = use_context::<Signal<NonogramData>>();
    rsx! {
        div {
            class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-6",
            pointer_events: if use_data().completed { "none" },
            button {
                class: "flex justify-center items-center w-10 h-10 rounded-full border border-gray-400 bg-gray-700 hover:bg-blue-800 hover:scale-125 active:scale-150 transition-transform transform",
                onclick: move |_| {
                    use_solution.write().slide(-1, 0);
                    info!("Sliding the nonogram solution grid left");
                },
                Icon {
                    class: "w-11/12 h-11/12",
                    fill: "rgb(156, 163, 175)",
                    icon: FaArrowLeft,
                }
            }
            button {
                class: "flex justify-center items-center w-10 h-10 rounded-full border border-gray-400 bg-gray-700 hover:bg-blue-800 hover:scale-125 active:scale-150 transition-transform transform",
                onclick: move |_| {
                    use_solution.write().slide(0, -1);
                    info!("Sliding the nonogram solution grid up");
                },
                Icon {
                    class: "w-11/12 h-11/12",
                    fill: "rgb(156, 163, 175)",
                    icon: FaArrowUp,
                }
            }
            button {
                class: "flex justify-center items-center w-10 h-10 rounded-full border border-gray-400 bg-gray-700 hover:bg-blue-800 hover:scale-125 active:scale-150 transition-transform transform",
                onclick: move |_| {
                    use_solution.write().slide(0, 1);
                    info!("Sliding the nonogram solution grid down");
                },
                Icon {
                    class: "w-11/12 h-11/12",
                    fill: "rgb(156, 163, 175)",
                    icon: FaArrowDown,
                }
            }
            button {
                class: "flex justify-center items-center w-10 h-10 rounded-full border border-gray-400 bg-gray-700 hover:bg-blue-800 hover:scale-125 active:scale-150 transition-transform transform",
                onclick: move |_| {
                    use_solution.write().slide(1, 0);
                    info!("Sliding the nonogram solution grid right");
                },
                Icon {
                    class: "w-11/12 h-11/12",
                    fill: "rgb(156, 163, 175)",
                    icon: FaArrowRight,
                }
            }
        }
    }
}

/// A button component for adding a new color to the Nonogram palette.
///
/// This component allows adding a new color to the Nonogram palette, either by selecting a random
/// color or from a default set of colors. It also manages the active brush color.
///
/// # Context:
/// - `Signal<NonogramPalette>`: Updates and manages the Nonogram palette.
#[component]
fn NewColorButton() -> Element {
    let mut use_palette = use_context::<Signal<NonogramPalette>>();
    rsx! {
        button {
            class: "flex justify-center items-center w-10 h-10 rounded-full border border-gray-400 bg-gray-700 hover:bg-blue-800 hover:scale-125 active:scale-150 transition-transform transform",
            onclick: move |_| {
                let palette_len = use_palette().len();
                let getter = if palette_len < DEFAULT_PALETTE.len() {
                    use_palette
                        .write()
                        .add_color(String::from(DEFAULT_PALETTE.get(palette_len)));
                    "default"
                } else {
                    let mut rng = rand::thread_rng();
                    let random_color = format!(
                        "#{:02x}{:02x}{:02x}",
                        rng.gen_range(0..256),
                        rng.gen_range(0..256),
                        rng.gen_range(0..256),
                    );
                    use_palette.write().add_color(random_color);
                    "random"
                };
                use_palette.write().brush = palette_len;
                info!("New {} palette color: {}", getter, use_palette().show_brush());
            },
            Icon {
                class: "w-11/12 h-11/12",
                fill: "rgb(156, 163, 175)",
                icon: FaPlus,
            }
        }
    }
}

/// A component for displaying and managing the Nonogram color palette.
///
/// This component allows users to select colors from the Nonogram palette. Colors can be removed
/// if there is more than one color in the palette and if it is not used in the solution grid.
///
/// # Context:
/// - `Signal<NonogramPalette>`: Manages the Nonogram color palette.
/// - `Signal<NonogramSolution>`: Manages the current Nonogram solution grid to check color usage.
#[component]
fn ColorPalette(readonly: bool) -> Element {
    let mut use_palette = use_context::<Signal<NonogramPalette>>();
    let use_solution = use_context::<Signal<NonogramSolution>>();
    rsx! {
        for (i , color) in use_palette().color_palette.iter().enumerate() {
            button {
                key: "brush-{i}",
                style: "background-color: {color}",
                class: "w-10 h-10 rounded-full hover:bg-blue-800 hover:scale-125 active:scale-150 transition-transform transform",
                onclick: move |_| {
                    use_palette.write().set_brush(i);
                    info!("Changed brush color to: {}", use_palette().show_brush());
                },
                ondoubleclick: move |_| {
                    if use_palette().len() > 1
                        && use_solution()
                            .solution_grid
                            .iter()
                            .map(|row| *row.iter().max().unwrap_or(&0))
                            .max()
                            .unwrap_or(0) < i
                    {
                        info!("Removing brush color: {} -> {}", i, use_palette().get(i));
                        use_palette.write().remove_color(i);
                    } else {
                        info!("Cannot remove brush color: {}", use_palette().show_brush());
                    }
                },
            }
        }
    }
}

/// A component for inputting a file to save the current Nonogram solution.
///
/// This component provides an input field to select and save a Nonogram solution to a file.
/// It ensures proper filename format and manages input interaction states.
///
/// # Context:
/// - `Signal<NonogramData>`: Manages the filename and other data for saving.
#[component]
fn FileInput(readonly: bool) -> Element {
    let mut use_data = use_context::<Signal<NonogramData>>();
    rsx! {
        div { class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-3",
            label {
                r#for: "name-input",
                class: " py-2 text-gray-200 font-semibold cursor-pointer select-none",
                pointer_events: if readonly { "none" },
                color: if readonly { "darkgray" },
                {t!("label_save_nonogram")}
                ":"
            }
            div {
                class: "hover:scale-105 active:scale-110 transition-transform transform",
                style: "display: inline-block;",
                pointer_events: if readonly { "none" },
                input {
                    id: "name-input",
                    class: "appearance-none px-4 py-1 pr-16 w-min rounded border border-gray-500 bg-gray-800 text-white hover:bg-blue-800 focus:ring focus:ring-blue-500 focus:outline-none transition-transform transform",
                    pointer_events: if readonly { "none" },
                    color: if readonly { "darkgray" },
                    readonly,
                    r#type: "text",
                    placeholder: t!("label_save_nonogram"),
                    onchange: move |event| {
                        use_data.write().filename = event.value();
                    },
                    value: "{use_data().filename}",
                }
                if !use_data().filename.contains(".ngram") {
                    span {
                        class: "absolute inset-y-0 right-4 flex items-center pointer-events-none text-gray-400",
                        style: "font-family: monospace; color: darkgray;",
                        ".ngram"
                    }
                }
            }
        }
    }
}

/// A component for loading a Nonogram solution from a file.
///
/// This component provides an input field to load a Nonogram solution from a `.ngram` file.
/// It handles file reading, deserialization, and updating the Nonogram state accordingly.
///
/// # Context:
/// - `Signal<NonogramFile>`: Manages the loaded Nonogram file.
/// - `Signal<NonogramPuzzle>`: Updates the Nonogram puzzle based on the file data.
/// - `Signal<NonogramSolution>`: Updates the Nonogram solution based on the loaded data.
/// - `Signal<NonogramPalette>`: Manages the Nonogram palette from the loaded file.
/// - `Signal<NonogramData>`: Updates Nonogram data, including filename and completion status.
#[component]
fn FileLoadInput() -> Element {
    let mut use_file = use_context::<Signal<NonogramFile>>();
    let mut use_puzzle = use_context::<Signal<NonogramPuzzle>>();
    let mut use_solution = use_context::<Signal<NonogramSolution>>();
    let mut use_palette = use_context::<Signal<NonogramPalette>>();
    let mut use_data = use_context::<Signal<NonogramData>>();
    let load_nonogram_onchange = move |event: FormEvent| async move {
        info!("Loading nonogram...");
        match &event.files() {
            Some(file_engine) => {
                let files = file_engine.files();
                match files.get(0) {
                    Some(file) => match file_engine.read_file_to_string(file).await {
                        Some(json) => match serde_json::from_str::<NonogramFile>(&json) {
                            Ok(nonogram_file) => {
                                *use_file.write() = nonogram_file.clone();
                                use_solution.write().clear();
                                *use_puzzle.write() =
                                    NonogramPuzzle::from_solution(&nonogram_file.solution);
                                *use_palette.write() = nonogram_file.palette;
                                use_data.write().filename = file.clone();
                                use_data.write().completed = false;
                                use_solution.write().set_cols(use_puzzle().cols);
                                use_solution.write().set_rows(use_puzzle().rows);
                                info!("Nonogram loaded correctly!");
                            }
                            Err(err) => {
                                error!("Couldn't deserialize file '{file}': {err}");
                            }
                        },
                        None => {
                            error!("Couldn't read file: '{file}'");
                        }
                    },
                    None => {
                        error!("File engine had no attached files");
                    }
                }
            }
            None => {
                error!("Event hadn't a file engine attached: {event:?}");
            }
        }
    };
    rsx! {
        input {
            class: "appearance-none rounded border px-4 py-1 border-gray-500 bg-gray-800 text-white hover:bg-blue-800 hover:scale-110 active:scale-125 transition-transform transform cursor-pointer",
            r#type: "file",
            accept: ".ngram",
            multiple: false,
            onchange: load_nonogram_onchange,
            {t!("button_load_nonogram")}
        }
    }
}

/// A component for loading a Nonogram solution from a file.
///
/// This component provides an input field to load a Nonogram solution from a `.ngram` file.
/// It handles file reading, deserialization, and updating the Nonogram state accordingly.
///
/// # Context:
/// - `Signal<NonogramFile>`: Manages the loaded Nonogram file.
/// - `Signal<NonogramPuzzle>`: Updates the Nonogram puzzle based on the file data.
/// - `Signal<NonogramSolution>`: Updates the Nonogram solution based on the loaded data.
/// - `Signal<NonogramPalette>`: Manages the Nonogram palette from the loaded file.
/// - `Signal<NonogramData>`: Updates Nonogram data, including filename and completion status.
#[component]
fn FileLoadEditInput() -> Element {
    let mut use_solution = use_context::<Signal<NonogramSolution>>();
    let mut use_palette = use_context::<Signal<NonogramPalette>>();
    let mut use_data = use_context::<Signal<NonogramData>>();
    let load_nonogram_onchange = move |event: FormEvent| async move {
        info!("Loading nonogram...");
        match &event.files() {
            Some(file_engine) => {
                let files = file_engine.files();
                match files.get(0) {
                    Some(file) => match file_engine.read_file_to_string(file).await {
                        Some(json) => match serde_json::from_str::<NonogramFile>(&json) {
                            Ok(nonogram_file) => {
                                use_solution.write().set_cols(nonogram_file.solution.cols());
                                use_solution.write().set_rows(nonogram_file.solution.rows());
                                *use_solution.write() = nonogram_file.solution;
                                *use_palette.write() = nonogram_file.palette;
                                use_data.write().filename = file.clone();
                                use_data.write().completed = false;
                                info!("Nonogram loaded correctly!");
                            }
                            Err(err) => {
                                error!("Couldn't deserialize file '{file}': {err}");
                            }
                        },
                        None => {
                            error!("Couldn't read file: '{file}'");
                        }
                    },
                    None => {
                        error!("File engine had no attached files");
                    }
                }
            }
            None => {
                error!("Event hadn't a file engine attached: {event:?}");
            }
        }
    };
    rsx! {
        input {
            class: "appearance-none rounded border px-4 py-1 border-gray-500 bg-gray-800 text-white hover:bg-blue-800 hover:scale-110 active:scale-125 transition-transform transform cursor-pointer",
            r#type: "file",
            accept: ".ngram",
            multiple: false,
            onchange: load_nonogram_onchange,
            {t!("button_load_nonogram")}
        }
    }
}

#[cfg(not(feature = "web"))]
/// A function to save a Nonogram solution to a file.
///
/// This function saves the provided Nonogram solution as a `.ngram` file.
/// Depending on the platform, it behaves differently:
/// - On non-web platforms, it writes the data directly to the file system.
/// - On web platforms, it creates a downloadable data URI link for the Nonogram file.
///   and clicks it programatically (there isn't a standard way to do it).
///
/// # Arguments:
/// - `json`: The Nonogram solution in JSON format.
/// - `filename`: The desired filename for the saved Nonogram file.
fn save_nonogram(json: String, filename: String) {
    use std::fs;
    use std::io::Write;

    let mut file = fs::File::create(&filename).expect("Failed to create ngram file");
    file.write_all(json.as_bytes())
        .expect("Failed to write data to ngram file");
    println!("Nonogram saved to {}", filename);
}

#[cfg(feature = "web")]
/// A function to save a Nonogram solution to a file.
///
/// This function saves the provided Nonogram solution as a `.ngram` file.
/// Depending on the platform, it behaves differently:
/// - On non-web platforms, it writes the data directly to the file system.
/// - On web platforms, it creates a downloadable data URI link for the Nonogram file
///   and clicks it programatically (there isn't a standard way to do it).
///
/// # Arguments:
/// - `json`: The Nonogram solution in JSON format.
/// - `filename`: The desired filename for the saved Nonogram file.
fn save_nonogram(json: String, filename: String) {
    let data_uri = format!(
        "data:application/json;charset=utf-8,{}",
        urlencoding::encode(&json)
    );

    let document = web_sys::window().unwrap().document().unwrap();
    let a = document.create_element("a").unwrap();
    a.set_attribute("href", &data_uri).unwrap();
    a.set_attribute("download", &filename).unwrap();

    let body = document.body().unwrap();
    body.append_child(&a).unwrap();
    let click_event = web_sys::MouseEvent::new("click").unwrap();
    a.dispatch_event(&click_event).unwrap();
    body.remove_child(&a).unwrap();
}

/// Renders a button that allows saving a Nonogram solution.
///
/// The `FileSaveButton` component provides a button to save the current Nonogram solution.
/// When clicked, it serializes the Nonogram solution to JSON and saves it either as a file
/// on non-web platforms or as a downloadable data URI on web platforms.
///
/// # Contexts:
/// - `Signal<NonogramSolution>`: Represents the current Nonogram solution.
/// - `Signal<NonogramPalette>`: Represents the color palette used in the Nonogram.
/// - `Signal<NonogramData>`: Contains additional data like filename.
///
/// # Events:
/// - `onclick`: Initiates the save operation.
///
/// # Error Handling:
/// If serialization of the Nonogram fails, an error is logged.
#[component]
fn FileSaveButton() -> Element {
    let use_solution = use_context::<Signal<NonogramSolution>>();
    let use_palette = use_context::<Signal<NonogramPalette>>();
    let use_data = use_context::<Signal<NonogramData>>();

    let save_nonogram_onclick = move |_| {
        info!("Saving nonogram...");
        let solution = use_solution().clone();
        let palette = use_palette().clone();
        let file = NonogramFile { solution, palette };

        match serde_json::to_string(&file) {
            Ok(json) => {
                let mut filename = use_data().filename.to_string();
                if filename.is_empty() {
                    filename = "nonogram".to_string();
                }
                let extension = if filename.ends_with(".ngram") {
                    ""
                } else {
                    ".ngram"
                };
                let filename = format!("{}{}", filename, extension);

                save_nonogram(json, filename);

                info!("Nonogram prepared for download!");
            }
            Err(err) => {
                error!("Failed to serialize the nonogram: {}", err);
            }
        }
    };

    rsx! {
        button {
            class: "px-4 py-1 font-bold rounded border border-gray-500 bg-gray-800 text-white hover:bg-blue-800 hover:scale-110 active:scale-125 transition-transform transform",
            onclick: save_nonogram_onclick,
            {t!("button_save_nonogram")}
        }
    }
}

/// Displays a visual preview of the Nonogram solution.
///
/// The `SolutionPreview` component shows the solution grid of a Nonogram, using colors
/// defined in the palette. Each cell's color corresponds to its state in the solution grid.
///
/// # Contexts:
/// - `Signal<NonogramFile>`: Provides the Nonogram solution and palette.
/// - `Signal<usize>`: Displays the current score based on the solution.
#[component]
fn SolutionPreview() -> Element {
    let use_file = use_context::<Signal<NonogramFile>>();
    let use_score = use_context::<Signal<usize>>();
    let solution_grid = use_file().solution.solution_grid.clone();
    rsx! {
        div { class: "flex flex-row justify-center justify-items-center items-center",
            label { class: "text-xl px-2",
                {t!("score")}
                ": {use_score()}"
            }
            table { class: "pointer-events-none", draggable: false,
                tbody {
                    for (i , row_data) in solution_grid.iter().enumerate() {
                        tr {
                            for (j , cell) in row_data.iter().enumerate() {
                                td {
                                    key: "cell-{i}-{j}",
                                    class: "select-none",
                                    style: "background-color: {use_file().palette.color_palette[*cell]}; width: 10px; height: 10px;",
                                    border_color: use_file().palette.border_color(*cell),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Provides a color input for selecting brush color in Nonogram editing.
///
/// The `ColorInput` component allows users to select a color from a color picker.
/// The selected color is used to modify cells in the Nonogram grid.
///
/// # Contexts:
/// - `Signal<NonogramPalette>`: Provides access to the current color palette.
#[component]
fn ColorInput() -> Element {
    let mut use_palette = use_context::<Signal<NonogramPalette>>();
    rsx! {
        div { class: "flex justify-end",
            input {
                r#type: "color",
                class: "appearance-none w-10 h-10 border outline-none hover:scale-125 active:scale-150 focus:ring focus:ring-blue-500 focus:outline-none transition-transform transform cursor-pointer",
                value: "{use_palette().get_current()}",
                onchange: move |event| {
                    use_palette.write().set_current(event.value());
                    info!("Change brush color {}", use_palette().show_brush());
                },
            }
        }
    }
}

/// Renders the column constraints of a Nonogram puzzle.
///
/// The `ColumnsConstraints` component visualizes the constraints for columns in a Nonogram puzzle.
/// Each segment of a constraint has a specific color, and the grid is styled according to palette colors.
///
/// # Contexts:
/// - `Signal<NonogramPalette>`: Provides colors for segments.
/// - `Signal<NonogramData>`: Provides block sizes for styling.
#[component]
fn ColumnsConstraints(puzzle: NonogramPuzzle) -> Element {
    let use_data = use_context::<Signal<NonogramData>>();
    let use_palette = use_context::<Signal<NonogramPalette>>();
    let max_table_rows = puzzle
        .col_constraints
        .iter()
        .map(|segments| segments.len())
        .max()
        .unwrap_or(0);
    rsx! {
        table {
            id: "col-constaints-table",
            class: "min-w-full min-h-full pointer-events-none",
            draggable: false,
            tbody {
                for i in 0..max_table_rows {
                    tr {
                        for (j , segments) in puzzle.col_constraints.iter().enumerate() {
                            if let Some(segment) = segments
                                .get((segments.len() as isize - max_table_rows as isize + i as isize) as usize)
                            {
                                td {
                                    key: "col-{i}-{j}",
                                    class: "border select-none",
                                    style: "background-color: {use_palette().color_palette[segment.color]}; min-width: {use_data().block_size}px; max-width: {use_data().block_size}px; height: {use_data().block_size}px; font-size: {use_data().block_size/2}px; color: {use_palette().text_color(segment.color)}",
                                    border_color: use_palette().border_color(segment.color),
                                    "{segment.length}"
                                }
                            } else {
                                td {
                                    key: "col-{i}-{j}",
                                    style: "min-width: {use_data().block_size}px; height: {use_data().block_size}px",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Displays the row constraints of a Nonogram puzzle.
///
/// The `RowsConstraints` component renders the constraints for rows in a Nonogram puzzle.
/// Segments are colored according to the palette, and styled based on their length and position.
///
/// # Contexts:
/// - `Signal<NonogramPalette>`: Supplies color information for each segment.
/// - `Signal<NonogramData>`: Provides block sizes and color styles.
#[component]
fn RowsConstraints(puzzle: NonogramPuzzle) -> Element {
    let use_palette = use_context::<Signal<NonogramPalette>>();
    let use_data = use_context::<Signal<NonogramData>>();
    let max_table_cols = puzzle
        .row_constraints
        .iter()
        .map(|segments| segments.len())
        .max()
        .unwrap_or(0);

    rsx! {
        table {
            class: "max-w-min min-h-full pointer-events-none",
            draggable: false,
            tbody {
                for (i , segments) in puzzle.row_constraints.iter().enumerate() {
                    tr {
                        for j in 0..max_table_cols {
                            if let Some(segment) = segments
                                .get((segments.len() as isize - max_table_cols as isize + j as isize) as usize)
                            {
                                td {
                                    key: "row-{i}-{j}",
                                    class: "border select-none",
                                    style: "background-color: {use_palette().color_palette[segment.color]}; min-width: {use_data().block_size}px; max-width: {use_data().block_size}px; height: {use_data().block_size}px; font-size: {use_data().block_size/2}px; color: {use_palette().text_color(segment.color)}",
                                    border_color: use_palette().border_color(segment.color),
                                    "{segment.length}"
                                }
                            } else {
                                td {
                                    key: "row-{i}-{j}",
                                    style: "min-width: {use_data().block_size}px; height: {use_data().block_size}px",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Displays the interactive Nonogram solution grid with functionality for drawing and modifying cells.
///
/// The `Solution` component provides a grid interface for solving the Nonogram puzzle.
/// Users can click, drag, and modify cells using different brushes and color inputs.
/// It supports shift and control modifications for more advanced interactions.
///
/// # Contexts:
/// - `Signal<usize>`: The current score of the solution.
/// - `Signal<NonogramPuzzle>`: Provides the puzzle structure.
/// - `Signal<NonogramSolution>`: Contains the current solution state.
/// - `Signal<NonogramPalette>`: Defines the color palette used.
/// - `Signal<NonogramData>`: Contains additional data for block sizes and border colors.
#[component]
fn Solution() -> Element {
    let mut use_score = use_context::<Signal<usize>>();
    let use_puzzle = use_context::<Signal<NonogramPuzzle>>();
    let mut use_solution = use_context::<Signal<NonogramSolution>>();
    let use_palette = use_context::<Signal<NonogramPalette>>();
    let use_data = use_context::<Signal<NonogramData>>();
    let solution_grid = use_solution().solution_grid.clone();
    let mut use_start = use_signal(|| None);
    let mut use_end = use_signal(|| None);
    let mut current_hover = use_signal(|| None);
    use_effect(move || {
        *use_score.write() = use_puzzle().score(&use_solution());
    });
    rsx! {
        table {
            class: "min-w-full min-h-full border-4",
            border_width: "3px",
            border_color: "#9ca3af",
            draggable: false,
            pointer_events: if use_data().completed { "none" },
            tbody {
                for (i , row_data) in solution_grid.iter().enumerate() {
                    tr {
                        for (j , cell) in row_data.iter().enumerate() {
                            // TODO!: FIX mouse over for mobile
                            td {
                                key: "cell-{i}-{j}",
                                class: "border select-none cursor-pointer border-gray-400",
                                style: "background-color: {use_palette().color_palette[*cell]}; min-width: {use_data().block_size}px; height: {use_data().block_size}px;",
                                border_color: if use_solution().in_line(use_start(), use_end(), (i, j))
    || current_hover() == Some((i, j)) { String::from("red") } else { use_palette().border_color(*cell) },
                                border_width: if use_solution().in_line(use_start(), use_end(), (i, j))
    || current_hover() == Some((i, j)) { "3px" } else { "1px" },
                                onmousedown: move |event| {
                                    if event.modifiers().shift() || event.modifiers().ctrl() {
                                        let color = use_palette().brush;
                                        info!(
                                            "Changed cell ({}, {}) with color {}", i + 1, j + 1, use_palette()
                                            .show_brush()
                                        );
                                        use_solution.write().solution_grid[i][j] = color;
                                    } else {
                                        info!("Init press on ({}, {})", i + 1, j + 1);
                                        *use_start.write() = Some((i, j));
                                        *use_end.write() = Some((i, j));
                                    }
                                },
                                onmouseover: move |event| {
                                    if event.held_buttons().contains(MouseButton::Primary) {
                                        *current_hover.write() = None;
                                        info!("Entered press on ({}, {})", i + 1, j + 1);
                                        if event.modifiers().shift() || event.modifiers().ctrl() {
                                            let color = use_palette().brush;
                                            info!(
                                                "Changed cell ({}, {}) with color {}", i + 1, j + 1, use_palette()
                                                .show_brush()
                                            );
                                            use_solution.write().solution_grid[i][j] = color;
                                        } else if use_start().is_some() {
                                            *use_end.write() = Some((i, j));
                                        }
                                    } else {
                                        *current_hover.write() = Some((i, j));
                                        *use_start.write() = None;
                                        *use_end.write() = None;
                                    }
                                },
                                onmouseleave: move |_| {
                                    *current_hover.write() = None;
                                },
                                onmouseup: move |_| {
                                    if use_start().is_some() {
                                        info!("Exit press on ({}, {})", i + 1, j + 1);
                                        let color = use_palette().brush;
                                        let start = use_start().unwrap();
                                        use_solution.write().draw_line(start, (i, j), color);
                                        *current_hover.write() = None;
                                        *use_start.write() = None;
                                        *use_end.write() = None;
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(any(target_os = "android", feature = "web"))]
/// Displays nothing on web and mobile platforms due to plotters dependencies conflicts.
#[component]
fn ConvergeGraphic() -> Element {
    rsx! {}
}

#[cfg(not(any(target_os = "android", feature = "web")))]
/// Generates a convergence graph of Nonogram solving progress for non-web platforms.
///
/// This version generates a PNG image of the convergence graph and provides it as a base64-encoded data URI for display.
#[component]
fn ConvergeGraphic() -> Element {
    use base64::prelude::*;
    use image::codecs::png::PngEncoder;
    use image::ImageEncoder;
    use plotters::prelude::*;
    use std::io::Cursor;
    const GRAPH_WIDTH: u32 = 600;
    const GRAPH_HEIGHT: u32 = 400;
    let use_history = use_context::<Signal<History>>();
    let buf_size = (GRAPH_WIDTH * GRAPH_HEIGHT) as usize * 3;
    let mut buf = vec![0u8; buf_size];
    let root = BitMapBackend::with_buffer(buf.as_mut_slice(), (GRAPH_WIDTH, GRAPH_HEIGHT))
        .into_drawing_area();
    root.fill(&WHITE).unwrap();

    let max_score = match use_history().worst.iter().max() {
        Some(max) => *max,
        None => {
            info!("The graph it's empty");
            return rsx! {};
        }
    };

    let mut chart = ChartBuilder::on(&root)
        .caption(t!("title_convergence_graph"), ("sans-serif", 30))
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        .margin(20)
        .margin_right(50)
        .build_cartesian_2d(0..use_history().iterations, 0 as f64..max_score as f64)
        .unwrap();

    chart
        .configure_mesh()
        .x_label_style(("sans-serif", 20).into_font())
        .y_label_style(("sans-serif", 20).into_font())
        .x_desc(t!("iterations"))
        .y_desc(t!("score"))
        .draw()?;

    info!("Best scores: {:?}", use_history().best);
    info!("Median scores: {:?}", use_history().median);
    info!("Worst scores: {:?}", use_history().worst);

    chart
        .draw_series(LineSeries::new(
            use_history().best.iter().map(|&y| y as f64).enumerate(),
            &GREEN,
        ))
        .unwrap()
        .label(t!("best"))
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .draw_series(LineSeries::new(
            use_history().median.iter().map(|&y| y as f64).enumerate(),
            &BLUE,
        ))
        .unwrap()
        .label(t!("median"))
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .draw_series(LineSeries::new(
            use_history().worst.iter().map(|&y| y as f64).enumerate(),
            &RED,
        ))
        .unwrap()
        .label(t!("worst"))
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(RGBColor(178, 178, 178))
        .label_font(("sans-serif", 20).into_font())
        .border_style(&BLACK)
        .position(SeriesLabelPosition::Coordinate(
            GRAPH_WIDTH as i32 / 2,
            GRAPH_HEIGHT as i32 / 6,
        ))
        .draw()?;

    drop(chart);
    drop(root);

    let mut data = vec![0; 0];
    let cursor = Cursor::new(&mut data);
    let encoder = PngEncoder::new(cursor);
    let color = image::ColorType::Rgb8;

    match encoder.write_image(buf.as_slice(), GRAPH_WIDTH, GRAPH_HEIGHT, color.into()) {
        Ok(_) => {
            let buffer_base64 = BASE64_STANDARD.encode(data);
            return rsx! {
                img { src: "data:image/png;base64,{buffer_base64}" }
            };
        }
        Err(e) => {
            info!("The PNG encoder should have written the image: {e}");
            return rsx! {};
        }
    }
}
