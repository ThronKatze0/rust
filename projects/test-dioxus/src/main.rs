#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 1);

    rsx! {
        h1 {
            "Counter: {count}"
        }
        h2 {
            {
                match count() {
                    1 => "Yeah",
                    2 => "No",
                    3 => "What?",
                    4 => "No really?",
                    5 => "I don't want",
                    6 => "Fuck you",
                    7 => "Leave me alone",
                    8 => "Rust is better than Go",
                    9 => "Go is for 5 year olds",
                    10 => "The end",
                    _ => "Hacker",
                }
            }
        }
        button {
            onclick: move |_| {
                if count() + 1 > 10 {return}
                count += 1;
            },
            "Increase"
        }
        button {
            onclick: move |_| {
                if count() - 1 < 1 {return}
                count -= 1;
            },
            "Decrease"
        }
    }
}
