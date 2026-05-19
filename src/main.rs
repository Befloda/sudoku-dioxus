mod components;
mod sudoku;

//use dioxus::prelude::*;
use components::app::app;

fn main() {
    dioxus::launch(app);
}
