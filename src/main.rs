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

mod nonogram {
    pub mod component;
    pub mod definitions;
    pub mod editor;
    pub mod genetic;
    pub mod implementations;
    pub mod macros;
}

use nonogram::component::Editor;

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
    Home {},
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

#[component]
fn Header() -> Element {
    let mut i18n = i18n();

    let change_language = move |event: FormEvent| {
        info!("Change language event: {:?}", event);
        match event.value().as_str() {
            "en-US" => i18n.set_language(EN_US),
            "es-MX" => i18n.set_language(ES_MX),
            _ => {}
        }
    };

    rsx! {
        div { class: "container mx-auto flex items-center justify-between py-4 px-6 bg-gray-800",
            div { class: "text-white text-2xl font-bold",
                Link { to: Route::Home {}, "NGRAM" }
            }
            div { class: "flex-1 mx-4 overflow-x-auto whitespace-nowrap flex items-center gap-2",
                Link {
                    to: Route::Home {},
                    class: "inline-block text-white text-xl",
                    {t!("title_home")}
                }
                span { class: "text-white", "|" }
                Link {
                    to: Route::Editor {},
                    class: "inline-block text-white text-xl",
                    {t!("title_nonogram_editor")}
                }
            }
            div {
                select {
                    class: "appearance-none bg-gray-700 text-white border border-gray-600 rounded-md p-2 hover:bg-gray-600 transition ease-in-out duration-200",
                    onchange: change_language,
                    option { value: "en-US", {t!("lang_en_US")} }
                    option { value: "es-MX", {t!("lang_es_MX")} }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "container mx-auto flex items-center justify-center min-h-screen",
            h1 { class: "text-4xl text-center font-bold", {t!("hello_world")} }
        }
    }
}
