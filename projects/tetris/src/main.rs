#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

struct TetrisBoard {
    board: [[Blocktype; 20]; 10],
}

struct Block {
    blocktype: Blocktype,
    positions: Vec<(u8, u8)>
}

enum Blocktype {
    OrangeRicky,
    BlueRicky,
    ClevelandZ,
    RhodeIslandZ,
    Hero,
    Teewee,
    Smashboy
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        h1 {
            "Hello"
        }
    }
}
