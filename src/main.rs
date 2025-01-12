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

use dioxus::{
    logger::tracing::{info, Level},
    prelude::*,
};
use dioxus_i18n::{prelude::*, t};

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub mod nonogram {
    pub mod component;
    pub mod definitions;
    pub mod evolutive;
    pub mod genetic;
    pub mod implementations;
    pub mod macros;
    pub mod puzzles;
}
use nonogram::component::{Editor, Solver};

mod localization {
    use dioxus_i18n::unic_langid::{langid, LanguageIdentifier};

    pub const DEF_LANG: LanguageIdentifier = EN_US;
    pub const EN_US: LanguageIdentifier = langid!("en-US");
    pub const ES_MX: LanguageIdentifier = langid!("es-MX");
}
use localization::*;

#[derive(Routable, Clone)]
enum Route {
    #[layout(Header)]
    #[route("/")]
    Solver {},
    #[route("/editor")]
    Editor {},
}

fn main() {
    dioxus::logger::init(Level::INFO).expect("Dioxus logger failed to init");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_init_i18n(|| {
        info!("Initializing i18n");
        I18nConfig::new(DEF_LANG)
            .with_fallback(ES_MX)
            .with_locale(Locale::new_static(
                EN_US,
                include_str!("../fluent/en-US.ftl"),
            ))
            .with_locale(Locale::new_static(
                ES_MX,
                include_str!("../fluent/es-MX.ftl"),
            ))
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

// TODO! FIX header on mobile or small screens
#[component]
fn Header() -> Element {
    let mut i18n = i18n();

    let change_language = move |event: FormEvent| {
        info!("Change language to: {}", event.value());
        match event.value().as_str() {
            "en-US" => i18n.set_language(EN_US),
            "es-MX" => i18n.set_language(ES_MX),
            _ => {}
        }
    };

    fn get_language(mut i18n: I18n) -> String {
        let lang = i18n.language();
        format!(
            "{}-{}",
            lang.language.as_str(),
            if let Some(l) = lang.region {
                String::from(l.as_str())
            } else {
                String::from("")
            }
        )
    }

    rsx! {
        div { class: "mx-auto flex items-center justify-between py-4 px-6 bg-gray-800",
            div { class: "text-white text-2xl font-bold",
                Link { to: Route::Solver {}, "NGRAM" }
            }
            div { class: "flex-1 mx-4 overflow-x-auto whitespace-nowrap flex items-center gap-2",
                Link {
                    to: Route::Solver {},
                    class: "inline-block text-white text-xl",
                    {t!("title_nonogram_solver")}
                }
                span { class: "text-white", "|" }
                Link {
                    to: Route::Editor {},
                    class: "inline-block text-white text-xl",
                    {t!("title_nonogram_editor")}
                }
            }
            select {
                class: "appearance-none bg-gray-700 text-white border border-gray-600 rounded-md p-2 hover:bg-gray-600 transition ease-in-out duration-200",
                value: "{get_language(i18n)}",
                onchange: change_language,
                option { value: "en-US", {t!("lang_en_US")} }
                option { value: "es-MX", {t!("lang_es_MX")} }
            }
        }
        Outlet::<Route> {}
    }
}
