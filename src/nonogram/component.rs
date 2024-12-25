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

use super::definitions::NonogramSolution;
use super::editor::NonogramEditor;
use crate::npalette;
use dioxus::{logger::tracing::info, prelude::*};
use dioxus_i18n::t;

#[component]
pub fn Editor() -> Element {
    use_context_provider(|| {
        Signal::new(NonogramEditor {
            palette: npalette!("#1e90ff", "#32cd32", "#a52a2a"),
            nonogram: NonogramSolution {
                solution_grid: vec![vec![0; 3]; 3],
            },
        })
    });
    rsx! {
        main { class: "flex flex-col items-center gap-4 min-h-screen py-4",
            h1 { class: "text-2xl font-bold", {t!("title_nonogram_editor")} }
            Toolbar {}
            Nonogram {}
        }
    }
}

#[component]
fn Toolbar() -> Element {
    let mut editor = use_context::<Signal<NonogramEditor>>();

    let save_onclick = move |_| {
        info!("Saving nonogram");
    };

    let load_onclick = move |_| {
        info!("Loading nonogram");
    };

    rsx! {
        section { class: "flex flex-row items-center p-4 rounded shadow-md space-x-4",
            div { class: "flex flex-row items-center space-x-2",
                for (i , color) in editor.read().palette.color_palette.iter().enumerate() {
                    button {
                        class: "w-8 h-8 rounded-full border border-gray-300",
                        style: format!("background-color: {}", color),
                        onclick: move |_| {
                            info!("Brush color: {} -> {}", i + 1, editor.read().palette.color_palette[i]);
                            editor.write().palette.brush_color = i;
                        },
                    }
                }
            }
            div { class: "flex flex-row items-center space-x-2",
                button {
                    class: "px-4 py-2 bg-blue-500 text-white text-bold rounded hover:bg-blue-600",
                    onclick: save_onclick,
                    {t!("button_save_nonogram")}
                }
                button {
                    class: "px-4 py-2 bg-green-500 text-white text-bold rounded hover:bg-green-600",
                    onclick: load_onclick,
                    {t!("button_load_nonogram")}
                }
            }
        }
    }
}

#[component]
fn Nonogram() -> Element {
    let editor = use_context::<Signal<NonogramEditor>>();
    rsx! {
        section { class: "",
            table { class: "",
                thead { class: "",
                    tr {
                        th { class: "flex justify-center items-center",
                            div {
                                class: "w-10 h-10 rounded-full",
                                style: "background-color: {editor.read().palette.color_palette[editor.read().palette.brush_color]}",
                            }
                        }
                        th { class: "", ColConstraints {} }
                    }
                }
                tbody { class: "",
                    tr { class: "",
                        th { class: "", RowConstraints {} }
                        td { Solution {} }
                    }
                }
            }
        }
    }
}

#[component]
fn ColConstraints() -> Element {
    let editor = use_context::<Signal<NonogramEditor>>();
    let col_constraints = editor.read().nonogram.col_constraints();
    let max_table_rows = col_constraints
        .iter()
        .map(|segments| segments.len())
        .max()
        .unwrap_or(0);

    rsx! {
        table { class: "min-w-full min-h-full border-separate border-spacing-1",
            tbody {
                for i in 0..max_table_rows {
                    tr {
                        for (j , segments) in col_constraints.iter().enumerate() {
                            if let Some(segment) = segments
                                .get((segments.len() as isize - max_table_rows as isize + i as isize) as usize)
                            {
                                td {
                                    key: "col-{i}-{j}",
                                    class: "w-10 h-10 border",
                                    style: "color: {editor.read().palette.color_palette[segment.segment_color]}; border-color: {editor.read().palette.color_palette[segment.segment_color]}",
                                    "{segment.segment_length}"
                                }
                            }
                            if segments.len() as isize - max_table_rows as isize + i as isize >= 0 {

                            } else {
                                td { class: "w-10 h-10" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RowConstraints() -> Element {
    let editor = use_context::<Signal<NonogramEditor>>();
    let row_constraints = editor.read().nonogram.row_constraints();
    let max_table_cols = row_constraints
        .iter()
        .map(|segments| segments.len())
        .max()
        .unwrap_or(0);

    rsx! {
        table { class: "min-w-full min-h-full border-separate border-spacing-1",
            tbody {
                for (i , segments) in row_constraints.iter().enumerate() {
                    tr {
                        for j in 0..max_table_cols {
                            if let Some(segment) = segments
                                .get((segments.len() as isize - max_table_cols as isize + j as isize) as usize)
                            {
                                td {
                                    key: "row-{i}-{j}",
                                    class: "w-10 h-10 border",
                                    style: "color: {editor.read().palette.color_palette[segment.segment_color]}; border-color: {editor.read().palette.color_palette[segment.segment_color]}",
                                    "{segment.segment_length}"
                                }
                            } else {
                                td { class: "w-10 h-10" }
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
    let mut editor = use_context::<Signal<NonogramEditor>>();

    let solution_grid = editor.read().nonogram.solution_grid.clone();

    rsx! {
        table { class: "min-w-full min-h-full border-separate border-spacing-1",
            tbody {
                for (i , row_data) in solution_grid.iter().enumerate() {
                    tr {
                        for (j , cell) in row_data.iter().enumerate() {
                            td {
                                key: "cell-{i}-{j}",
                                class: "w-10 h-10 border",
                                style: "background-color: {editor.read().palette.color_palette[*cell]}; border-color: {editor.read().palette.color_palette[*cell]}",
                                onclick: move |_| {
                                    let color = editor.read().palette.brush_color;
                                    info!(
                                        "Changed cell ({}, {}) with color {} -> {}", i + 1, j + 1, color, editor
                                        .read().palette.color_palette[color]
                                    );
                                    editor.write().nonogram.solution_grid[i][j] = color;
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
