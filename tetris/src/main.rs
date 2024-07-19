#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    // Urls are relative to your Cargo.toml file
    const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

    rsx! {
        div { class: "grid grid-cols-10 grid-rows-20 gap-1 h-screen w-screen",
            for _ in 0..200 {
                div { class: "h-8 w-8 bg-green-700" }
            }
        }
    }
}
