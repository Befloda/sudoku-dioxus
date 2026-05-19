use super::app::GameState;
use crate::sudoku::{Difficulty, SudokuBoard};
use dioxus::prelude::*;

#[component]
pub fn Controls(
    board: Signal<Option<SudokuBoard>>,
    selected: Signal<Option<(usize, usize)>>,
    game_state: Signal<GameState>,
    hint_used: Signal<u32>,
    difficulty: Signal<Difficulty>,
    errors: Signal<u32>,
    timer: Signal<u64>,
) -> Element {
    rsx! {
        div { class: "controls",
            // Bouton indice
            button {
                class: "ctrl-btn hint-btn",
                onclick: move |_| {
                    let mut b = board.write();
                    let Some(board_ref) = b.as_mut() else { return };
                    if let Some(_pos) = board_ref.reveal_hint() {
                        *hint_used.write() += 1;
                    }
                    if board_ref.is_solved() {
                        drop(b);
                        *game_state.write() = GameState::Won;
                    }
                },
                span { "💡" }
                "Indice"
            }

            // Bouton nouvelle partie
            button {
                class: "ctrl-btn new-btn",
                onclick: move |_| {
                    let new_board = SudokuBoard::new(*difficulty.read());
                    *board.write() = Some(new_board);
                    *errors.write() = 0;
                    *hint_used.write() = 0;
                    *timer.write() = 0;
                    *selected.write() = None;
                },
                span { "🔄" }
                "Nouveau"
            }

            // Bouton menu
            button {
                class: "ctrl-btn menu-btn-ctrl",
                onclick: move |_| {
                    *game_state.write() = GameState::Menu;
                },
                span { "🏠" }
                "Menu"
            }
        }
    }
}
