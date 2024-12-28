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

use super::definitions::{NonogramEditor, NonogramSolution};
use super::definitions::{NonogramFile, NonogramPuzzle, DEFAULT_PALETTE};
use dioxus::{
    logger::tracing::{error, info},
    prelude::*,
};
use dioxus_elements::input_data::MouseButton;
use dioxus_free_icons::icons::fa_solid_icons::FaPlus;
use dioxus_free_icons::Icon;
use dioxus_i18n::t;
use rand::Rng;

#[component]
pub fn Editor() -> Element {
    use_context_provider(|| {
        info!("Initializing nonogram");
        Signal::new(NonogramEditor {
            palette: DEFAULT_PALETTE.take(5),
            nonogram: NonogramSolution {
                solution_grid: vec![vec![0; 5]; 5],
            },
            size: 40,
            start: None,
            end: None,
        })
    });

    std::panic::set_hook(Box::new(|info| {
        println!("Panic: {}", info);
    }));

    rsx! {
        main { class: "flex flex-col gap-10 items-center min-h-screen mb-20",
            h1 { class: "text-4xl font-bold my-10 text-center", {t!("title_nonogram_editor")} }
            Toolbar {}
            Nonogram {}
        }
    }
}

#[component]
fn Toolbar() -> Element {
    let mut editor = use_context::<Signal<NonogramEditor>>();
    let mut filename = use_signal(|| String::from("nonogram"));

    // TODO!: ADD support for web an mobile (file engines)
    let save_nonogram_onclick = move |_| {
        info!("Saving nonogram...");
        let puzzle = NonogramPuzzle::from_solution(&editor.read().nonogram);
        let mut palette = editor.read().palette.clone();

        async move {
            palette.brush_color = 0;
            let file = NonogramFile { puzzle, palette };

            match serde_json::to_string(&file) {
                Ok(json) => {
                    let filename = if filename.read().is_empty() {
                        String::from("nonogram")
                    } else {
                        filename.read().to_string()
                    };
                    let filename = format!("artifacts/{filename}.ngram");
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
        }
    };

    rsx! {
        section { class: "container flex flex-col space-y-6 p-6 rounded-lg shadow-lg bg-gray-900",
            div { class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-6",
                div { class: "flex flex-row justify-items-center justify-center items-center gap-2",
                    label {
                        r#for: "rows-input",
                        class: "py-2 text-gray-200 font-semibold cursor-pointer",
                        {t!("label_rows")}
                        ":"
                    }
                    input {
                        id: "rows-input",
                        class: "appearance-none px-4 py-1 w-20 rounded border border-gray-500 bg-gray-800 text-white focus:ring focus:ring-blue-500 focus:outline-none",
                        r#type: "number",
                        min: "2",
                        onchange: move |event| {
                            if let Ok(rows) = event.value().parse::<usize>() {
                                editor.write().nonogram.set_rows(rows);
                            }
                        },
                        value: editor.read().nonogram.rows(),
                    }
                }
                div { class: "flex flex-row justify-items-center justify-center items-center gap-2",
                    label {
                        r#for: "cols-input",
                        class: "py-2 text-gray-200 font-semibold cursor-pointer",
                        {t!("label_columns")}
                        ":"
                    }
                    input {
                        id: "cols-input",
                        class: "appearance-none px-4 py-1 w-20 rounded border border-gray-500 bg-gray-800 text-white focus:ring focus:ring-blue-500 focus:outline-none",
                        r#type: "number",
                        min: "2",
                        onchange: move |event: FormEvent| {
                            if let Ok(cols) = event.value().parse::<usize>() {
                                editor.write().nonogram.set_cols(cols);
                            }
                        },
                        value: editor.read().nonogram.cols(),
                    }
                }
                div { class: "flex flex-row justify-items-center justify-center items-center gap-2",
                    label {
                        r#for: "size-input",
                        class: "py-2 text-gray-200 font-semibold cursor-pointer",
                        {t!("label_size")}
                        ":"
                    }
                    input {
                        id: "size-input",
                        class: "appearance-none px-4 py-1 w-20 rounded border border-gray-500 bg-gray-800 text-white focus:ring focus:ring-blue-500 focus:outline-none",
                        r#type: "number",
                        min: "10",
                        onchange: move |event: FormEvent| {
                            if let Ok(size) = event.value().parse::<usize>() {
                                editor.write().size = size;
                            }
                        },
                        value: editor.read().size,
                    }
                }
            }
            div { class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-6",
                div { class: "flex flex-row flex-wrap justify-items-center justify-center items-center gap-2",
                    label {
                        r#for: "name-input",
                        class: " py-2 text-gray-200 font-semibold cursor-pointer",
                        {t!("label_save_nonogram")}
                        ":"
                    }
                    input {
                        id: "name-input",
                        class: "appearance-none px-4 py-1 rounded border border-gray-500 bg-gray-800 text-white focus:ring focus:ring-blue-500 focus:outline-none",
                        r#type: "text",
                        placeholder: t!("label_save_nonogram"),
                        onchange: move |event| {
                            *filename.write() = event.value();
                        },
                        value: "{filename.read()}",
                    }
                }
                button {
                    class: "px-4 py-1 font-bold rounded border border-gray-500 bg-gray-800 text-white hover:bg-blue-800  focus:outline-none focus:ring focus:ring-blue-300",
                    onclick: save_nonogram_onclick,
                    {t!("button_save_nonogram")}
                }
            }
            div { class: "flex flex-wrap justify-items-center justify-center items-center gap-6",
                for (i , color) in editor.read().palette.color_palette.iter().enumerate() {
                    button {
                        key: "brush-{i}",
                        style: "background-color: {color}",
                        class: "w-10 h-10 rounded-full border border-gray-400 hover:bg-gray-600 transition-transform transform hover:scale-125",
                        onclick: move |event| {
                            if event.modifiers().shift()
                                || event.modifiers().ctrl() && editor.read().palette.len() > 1
                            {
                                info!("Deleted brush color: {}", editor.read().palette.show_brush());
                                editor.write().palette.remove_color(i);
                                info!("Changed brush color: {}", editor.read().palette.show_brush());
                            } else {
                                editor.write().palette.brush_color = i;
                                info!("Changed brush color: {}", editor.read().palette.show_brush());
                            }
                        },
                    }
                }
                button {
                    class: "flex justify-center items-center w-10 h-10 rounded-full border border-gray-400 bg-gray-700 hover:bg-gray-600 transition-transform transform hover:scale-125",
                    onclick: move |_| {
                        let palette_len = editor.read().palette.len();
                        let getter = if palette_len < DEFAULT_PALETTE.len() {
                            editor
                                .write()
                                .palette
                                .push_color(String::from(DEFAULT_PALETTE.get(palette_len)));
                            "default"
                        } else {
                            let mut rng = rand::thread_rng();
                            let random_color = format!(
                                "#{:02x}{:02x}{:02x}",
                                rng.gen_range(0..256),
                                rng.gen_range(0..256),
                                rng.gen_range(0..256),
                            );
                            editor.write().palette.push_color(random_color);
                            "random"
                        };
                        editor.write().palette.set(palette_len);
                        info!("New {} palette color: {}", getter, editor.read().palette.show_brush());
                    },
                    Icon {
                        class: "w-full h-full",
                        fill: "rgb(156, 163, 175)",
                        icon: FaPlus,
                    }
                }
            }
        }
    }
}

// TODO!: Change color based in the RGB instead of only white, maybe change_color(rgb: String) -> bool
#[component]
fn Nonogram() -> Element {
    let mut editor = use_context::<Signal<NonogramEditor>>();
    rsx! {
        section { class: "mb-20",
            table { class: "border-separate border-spacing-4",
                thead {
                    tr {
                        th { class: "align-bottom",
                            div { class: "flex justify-end",
                                input {
                                    r#type: "color",
                                    class: "appearance-none w-10 h-10 border outline-none transition-transform transform hover:scale-125 focus:ring focus:ring-blue-500 focus:outline-none cursor-pointer",
                                    value: "{editor.read().palette.get_color()}",
                                    onchange: move |event| {
                                        editor.write().palette.set_color(event.value());
                                        info!("Change brush color {}", editor.read().palette.show_brush());
                                    },
                                }
                            }
                        }
                        th { class: "align-bottom", ColConstraints {} }
                    }
                }
                tbody {
                    tr {
                        th { class: "flex justify-end", RowConstraints {} }
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
        table {
            id: "col-constaints-table",
            class: "min-w-full min-h-full pointer-events-none",
            draggable: false,
            tbody {
                for i in 0..max_table_rows {
                    tr {
                        for (j , segments) in col_constraints.iter().enumerate() {
                            if let Some(segment) = segments
                                .get((segments.len() as isize - max_table_rows as isize + i as isize) as usize)
                            {
                                td {
                                    key: "col-{i}-{j}",
                                    class: "border select-none",
                                    style: "background-color: {editor.read().palette.color_palette[segment.segment_color]}; min-width: {editor.read().size}px; height: {editor.read().size}px; font-size: {editor.read().size/2}px",
                                    color: if editor.read().palette.color_palette[segment.segment_color] == "#ffffff" { "#000000" } else { "#ffffff" },
                                    "{segment.segment_length}"
                                }
                            } else {
                                td {
                                    key: "col-{i}-{j}",
                                    style: "min-width: {editor.read().size}px; height: {editor.read().size}px",
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
fn RowConstraints() -> Element {
    let editor = use_context::<Signal<NonogramEditor>>();
    let row_constraints = editor.read().nonogram.row_constraints();
    let max_table_cols = row_constraints
        .iter()
        .map(|segments| segments.len())
        .max()
        .unwrap_or(0);

    rsx! {
        table {
            class: "max-w-min min-h-full pointer-events-none",
            draggable: false,
            tbody {
                for (i , segments) in row_constraints.iter().enumerate() {
                    tr {
                        for j in 0..max_table_cols {
                            if let Some(segment) = segments
                                .get((segments.len() as isize - max_table_cols as isize + j as isize) as usize)
                            {
                                td {
                                    key: "row-{i}-{j}",
                                    class: "border select-none",
                                    style: "background-color: {editor.read().palette.color_palette[segment.segment_color]}; min-width: {editor.read().size}px; max-width: {editor.read().size}px; height: {editor.read().size}px; font-size: {editor.read().size/2}px",
                                    color: if editor.read().palette.color_palette[segment.segment_color] == "#ffffff" { "#000000" } else { "#ffffff" },
                                    "{segment.segment_length}"
                                }
                            } else {
                                td {
                                    key: "row-{i}-{j}",
                                    style: "min-width: {editor.read().size}px; height: {editor.read().size}px",
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
    let mut editor = use_context::<Signal<NonogramEditor>>();

    let solution_grid = editor.read().nonogram.solution_grid.clone();

    rsx! {
        table { class: "min-w-full min-h-full border-collapse", draggable: false,
            tbody {
                for (i , row_data) in solution_grid.iter().enumerate() {
                    tr {
                        for (j , cell) in row_data.iter().enumerate() {
                            // TODO!: FIX mouse over for mobile
                            td {
                                key: "cell-{i}-{j}",
                                class: "border select-none cursor-pointer",
                                style: "background-color: {editor.read().palette.color_palette[*cell]}; min-width: {editor.read().size}px; height: {editor.read().size}px;",
                                border_color: if editor.read().nonogram.in_line(editor.read().start, editor.read().end, (i, j)) { String::from("red") } else if editor.read().palette.color_palette[*cell] == "#ffffff" { String::from("black") } else { String::from("white") },
                                border_width: if editor.read().nonogram.in_line(editor.read().start, editor.read().end, (i, j)) { "3px" } else { "1px" },
                                onmousedown: move |event| {
                                    if event.modifiers().shift() || event.modifiers().ctrl() {
                                        let color = editor.read().palette.brush_color;
                                        info!(
                                            "Changed cell ({}, {}) with color {}", i + 1, j + 1, editor.read()
                                            .palette.show_brush()
                                        );
                                        editor.write().nonogram.solution_grid[i][j] = color;
                                    } else {
                                        info!("Init press on ({}, {})", i + 1, j + 1);
                                        editor.write().start = Some((i, j));
                                        editor.write().end = Some((i, j));
                                    }
                                },
                                onmouseover: move |event| {
                                    if event.held_buttons().contains(MouseButton::Primary) {
                                        info!("Entered press on ({}, {})", i + 1, j + 1);
                                        if event.modifiers().shift() || event.modifiers().ctrl() {
                                            let color = editor.read().palette.brush_color;
                                            info!(
                                                "Changed cell ({}, {}) with color {}", i + 1, j + 1, editor.read()
                                                .palette.show_brush()
                                            );
                                            editor.write().nonogram.solution_grid[i][j] = color;
                                        } else if editor.read().start.is_some() {
                                            editor.write().end = Some((i, j));
                                        }
                                    } else {
                                        editor.write().start = None;
                                        editor.write().end = None;
                                    }
                                },
                                onmouseup: move |_| {
                                    if editor.read().start.is_some() {
                                        info!("Exit press on ({}, {})", i + 1, j + 1);
                                        let color = editor.read().palette.brush();
                                        let start = editor.read().start.unwrap();
                                        editor.write().nonogram.draw_line(start, (i, j), color);
                                        editor.write().start = None;
                                        editor.write().end = None;
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
