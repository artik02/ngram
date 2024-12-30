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

use std::fs::File;
use std::io::Write;

use crate::nonogram::definitions::{NonogramData, NonogramPalette};
use crate::nonogram::puzzles::*;

use super::definitions::{NonogramFile, NonogramPuzzle, NonogramSolution, DEFAULT_PALETTE};
use dioxus::{
    logger::tracing::{error, info},
    prelude::*,
};
use dioxus_elements::input_data::MouseButton;
use dioxus_free_icons::icons::fa_solid_icons::{
    FaArrowDown, FaArrowLeft, FaArrowRight, FaArrowUp, FaDeleteLeft, FaPlus,
};
use dioxus_free_icons::Icon;
use dioxus_i18n::t;
use rand::Rng;

#[cfg(any(target_os = "android", target_os = "ios"))]
fn get_block_size() -> usize {
    30
}
#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn get_block_size() -> usize {
    40
}

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
        info!("Initializing nonogram editor state");
        Signal::new(NonogramData {
            filename: String::from("tree.ngram"),
            block_size: get_block_size(),
            start: None,
            end: None,
        })
    });

    rsx! {
        main { class: "flex flex-col gap-10 items-center min-h-screen mb-20",
            h1 { class: "text-4xl font-bold my-10 text-center", {t!("title_nonogram_solver")} }
            SolverToolbar {}
            SolverNonogram {}
        }
    }
}

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
                FileLoadButton {}
                FileInput { readonly: true }
            }
            div { class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-6",
                ClearSolutionButton {}
                SlideSolutionButtons {}
            }
            div { class: "flex flex-wrap justify-items-center justify-center items-center gap-6",
                ColorPalette { readonly: true }
            }
        }
    }
}

#[component]
fn SolverNonogram() -> Element {
    let use_puzzle = use_context::<Signal<NonogramPuzzle>>();
    //let use_solution = use_context::<Signal<NonogramSolution>>();
    //let current_puzzle = NonogramPuzzle::from_solution(&use_solution());
    //let diff_puzzle = current_puzzle.diff(&use_puzzle());
    rsx! {
        section { class: "mb-20",
            table { class: "border-separate border-spacing-4",
                thead {
                    tr {
                        th { class: "align-bottom", ColorInput {} }
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
    use_context_provider(|| {
        info!("Initializing nonogram editor state");
        Signal::new(NonogramData {
            filename: String::new(),
            block_size: get_block_size(),
            start: None,
            end: None,
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

#[component]
fn EditorNonogram() -> Element {
    let use_solution = use_context::<Signal<NonogramSolution>>();
    let current_puzzle = NonogramPuzzle::from_solution(&use_solution());
    rsx! {
        section { class: "mb-20",
            table { class: "border-separate border-spacing-4",
                thead {
                    tr {
                        th { class: "align-bottom", ColorInput {} }
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

#[component]
fn ClearSolutionButton() -> Element {
    let mut use_solution = use_context::<Signal<NonogramSolution>>();
    rsx! {
        button {
            class: "flex justify-center items-center w-10 h-10 rounded-full border border-gray-400 bg-gray-700 hover:bg-blue-800 hover:scale-125 active:scale-150 transition-transform transform",
            onclick: move |event| {
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

#[component]
fn SlideSolutionButtons() -> Element {
    let mut use_solution = use_context::<Signal<NonogramSolution>>();
    rsx! {
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

#[component]
fn ColorPalette(readonly: bool) -> Element {
    let mut use_palette = use_context::<Signal<NonogramPalette>>();
    rsx! {
        for (i , color) in use_palette().color_palette.iter().enumerate() {
            button {
                key: "brush-{i}",
                style: "background-color: {color}",
                class: "w-10 h-10 rounded-full hover:bg-blue-800 hover:scale-125 active:scale-150 transition-transform transform",
                onclick: move |event| {
                    if readonly || !(event.modifiers().ctrl() || event.modifiers().shift())
                        || use_palette().len() == 1
                    {
                        use_palette.write().set_brush(i);
                    } else {
                        info!("Removing brush color: {} -> {}", i, use_palette().get(i));
                        use_palette.write().remove_color(i);
                    }
                    info!("Changed brush color: {}", use_palette().show_brush());
                },
            }
        }
    }
}

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

#[component]
fn FileLoadButton() -> Element {
    let mut use_puzzle = use_context::<Signal<NonogramPuzzle>>();
    let mut use_solution = use_context::<Signal<NonogramSolution>>();
    let mut use_palette = use_context::<Signal<NonogramPalette>>();
    let mut use_data = use_context::<Signal<NonogramData>>();
    let load_nonogram_onclick = move |event: FormEvent| async move {
        info!("Loading nonogram...");
        match &event.files() {
            Some(file_engine) => {
                let files = file_engine.files();
                match files.get(0) {
                    Some(file) => match file_engine.read_file_to_string(file).await {
                        Some(json) => match serde_json::from_str::<NonogramFile>(&json) {
                            Ok(nonogram_file) => {
                                use_solution.write().clear();
                                *use_puzzle.write() = nonogram_file.puzzle;
                                *use_palette.write() = nonogram_file.palette;
                                use_data.write().filename = file.clone();
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
        button { class: "px-4 py-1 max-h-min font-bold rounded border border-gray-500 bg-gray-800 text-white hover:bg-blue-800 hover:scale-110 active:scale-125 transition-transform transform cursor-pointer",
            {t!("button_load_nonogram")}
            input {
                class: "absolute top-0 left-0 h-full w-full opacity-0 rounded border hover:scale-110 active:scale-125 transition-transform transform cursor-pointer",
                r#type: "file",
                accept: ".ngram",
                multiple: false,
                onchange: load_nonogram_onclick,
            }
        }
    }
}

#[component]
fn FileSaveButton() -> Element {
    let use_solution = use_context::<Signal<NonogramSolution>>();
    let use_palette = use_context::<Signal<NonogramPalette>>();
    let use_data = use_context::<Signal<NonogramData>>();
    // TODO!: ADD support for web an mobile (file engines)
    let save_nonogram_onclick = move |_| async move {
        info!("Saving nonogram...");
        let puzzle = NonogramPuzzle::from_solution(&use_solution());
        let mut palette = use_palette().clone();
        palette.brush = 0;
        let file = NonogramFile { puzzle, palette };

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
                // TODO!: Allow users to save where they want (file engine)
                let filename = format!("artifacts/{}{}", filename, extension);
                match File::create(&filename) {
                    Ok(mut file) => match file.write(json.as_bytes()) {
                        Ok(_) => {
                            info!("Nonogram '{}' saved successfully!", filename);
                        }
                        Err(err) => {
                            error!("Failed to write to file '{}': {}", filename, err);
                        }
                    },
                    Err(err) => {
                        error!("Failed to create the file '{}': {}", filename, err);
                    }
                }
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
                                    style: "background-color: {use_palette().color_palette[segment.segment_color]}; min-width: {use_data().block_size}px; max-width: {use_data().block_size}px; height: {use_data().block_size}px; font-size: {use_data().block_size/2}px; color: {use_palette().text_color(segment.segment_color)}",
                                    border_color: use_palette().border_color(segment.segment_color),
                                    "{segment.segment_length}"
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
                                    style: "background-color: {use_palette().color_palette[segment.segment_color]}; min-width: {use_data().block_size}px; max-width: {use_data().block_size}px; height: {use_data().block_size}px; font-size: {use_data().block_size/2}px; color: {use_palette().text_color(segment.segment_color)}",
                                    border_color: use_palette().border_color(segment.segment_color),
                                    "{segment.segment_length}"
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

#[component]
fn Solution() -> Element {
    let mut use_solution = use_context::<Signal<NonogramSolution>>();
    let use_palette = use_context::<Signal<NonogramPalette>>();
    let mut use_data = use_context::<Signal<NonogramData>>();
    let solution_grid = use_solution().solution_grid.clone();
    let mut current_hover = use_signal(|| None);

    rsx! {
        table {
            class: "min-w-full min-h-full border-4",
            border_width: "3px",
            border_color: "#9ca3af",
            draggable: false,
            tbody {
                for (i , row_data) in solution_grid.iter().enumerate() {
                    tr {
                        for (j , cell) in row_data.iter().enumerate() {
                            // TODO!: FIX mouse over for mobile
                            td {
                                key: "cell-{i}-{j}",
                                class: "border select-none cursor-pointer border-gray-400",
                                style: "background-color: {use_palette().color_palette[*cell]}; min-width: {use_data().block_size}px; height: {use_data().block_size}px;",
                                border_color: if use_solution().in_line(use_data().start, use_data().end, (i, j))
    || current_hover() == Some((i, j)) { String::from("red") } else { use_palette().border_color(*cell) },
                                border_width: if use_solution().in_line(use_data().start, use_data().end, (i, j))
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
                                        use_data.write().start = Some((i, j));
                                        use_data.write().end = Some((i, j));
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
                                        } else if use_data().start.is_some() {
                                            use_data.write().end = Some((i, j));
                                        }
                                    } else {
                                        *current_hover.write() = Some((i, j));
                                        use_data.write().start = None;
                                        use_data.write().end = None;
                                    }
                                },
                                onmouseleave: move |_| {
                                    *current_hover.write() = None;
                                },
                                onmouseup: move |_| {
                                    if use_data().start.is_some() {
                                        info!("Exit press on ({}, {})", i + 1, j + 1);
                                        let color = use_palette().brush;
                                        let start = use_data().start.unwrap();
                                        use_solution.write().draw_line(start, (i, j), color);
                                        *current_hover.write() = None;
                                        use_data.write().start = None;
                                        use_data.write().end = None;
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
