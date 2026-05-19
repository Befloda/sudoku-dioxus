use super::app::{GameState, InputMode};
use crate::sudoku::SudokuBoard;
use dioxus::prelude::*;

#[component]
pub fn NumPad(
    board: Signal<Option<SudokuBoard>>,
    selected: Signal<Option<(usize, usize)>>,
    game_state: Signal<GameState>,
    input_mode: Signal<InputMode>,
    errors: Signal<u32>,
) -> Element {
    rsx! {
        div { class: "numpad",
            for num in 1u8..=9 {
                button {
                    class: "numpad-btn",
                    onclick: move |_| {
                        if *game_state.read() != GameState::Playing {
                            return;
                        }
                        let Some((row, col)) = *selected.read() else { return };

                        let mut b = board.write();
                        let Some(board_ref) = b.as_mut() else { return };

                        match *input_mode.read() {
                            InputMode::Value => {
                                // Vérifie si c'est une erreur avant de poser
                                let is_wrong = board_ref.solution[row][col] != num;
                                if is_wrong {
                                    *errors.write() += 1;
                                }
                                board_ref.set_value(row, col, Some(num));
                            }
                            InputMode::Notes => {
                                board_ref.toggle_note(row, col, num);
                            }
                        }

                        if board_ref.is_solved() {
                            drop(b);
                            *game_state.write() = GameState::Won;
                        }
                    },
                    "{num}"
                }
            }
            // Bouton effacer
            button {
                class: "numpad-btn numpad-erase-btn",
                onclick: move |_| {
                    let Some((row, col)) = *selected.read() else { return };
                    let mut b = board.write();
                    let Some(board_ref) = b.as_mut() else { return };
                    board_ref.set_value(row, col, None);
                },
                "⌫"
            }
        }
    }
}
