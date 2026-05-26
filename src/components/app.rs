use super::board::Board;
use super::controls::Controls;
use super::numpad::NumPad;
use crate::sudoku::{Difficulty, SudokuBoard};
use super::themes;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Won,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InputMode {
    Value,
    Notes,
}

pub fn app() -> Element {
    let game_state = use_signal(|| GameState::Menu);
    let board = use_signal(|| Option::<SudokuBoard>::None);
    let selected = use_signal(|| Option::<(usize, usize)>::None);
    let difficulty = use_signal(|| Difficulty::Medium);
    let input_mode = use_signal(|| InputMode::Value);
    let hint_used = use_signal(|| 0u32);
    let errors = use_signal(|| 0u32);
    let timer = use_signal(|| 0u64);

    // Timer (incrémenté via JS setInterval simulé avec signal)
    // Note: pour un vrai timer, on utiliserait use_future ou gloo-timers

    rsx! {
        //On inclut le CSS global ici pour que les styles soient dans la compilation finale de l'application
        style { {include_str!("../tailwind.css")} }

        div { class: "app",
            // Header
            header { class: "header",
                div { class: "header-title",
                    span { class: "title-icon", "⬛" }
                    h1 { "SUDOKU" }
                }
                if *game_state.read() == GameState::Playing {
                    div { class: "header-stats",
                        div { class: "stat",
                            span { class: "stat-label", "ERREURS" }
                            span { class: "stat-value", "{errors}" }
                        }
                        div { class: "stat",
                            span { class: "stat-label", "INDICES" }
                            span { class: "stat-value", "{hint_used}" }
                        }
                        div { class: "stat",
                            span { class: "stat-label", "NIVEAU" }
                            span { class: "stat-value", "{difficulty.read().label()}" }
                        }
                    }
                }
            }

            main { class: "main",
                match *game_state.read() {
                    GameState::Menu => rsx! {
                        MenuScreen {
                            difficulty: difficulty.clone(),
                            game_state: game_state.clone(),
                            board: board.clone(),
                            errors: errors.clone(),
                            hint_used: hint_used.clone(),
                            timer: timer.clone(),
                            selected: selected.clone(),
                        }
                    },
                    GameState::Playing => rsx! {
                        PlayScreen {
                            board: board.clone(),
                            selected: selected.clone(),
                            game_state: game_state.clone(),
                            input_mode: input_mode.clone(),
                            difficulty: difficulty.clone(),
                            hint_used: hint_used.clone(),
                            errors: errors.clone(),
                            timer: timer.clone(),
                        }
                    },
                    GameState::Won => rsx! {
                        WonScreen {
                            game_state: game_state.clone(),
                            board: board.clone(),
                            difficulty: difficulty.clone(),
                            errors: errors.clone(),
                            hint_used: hint_used.clone(),
                            timer: timer.clone(),
                            selected: selected.clone(),
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn MenuScreen(
    difficulty: Signal<Difficulty>,
    game_state: Signal<GameState>,
    board: Signal<Option<SudokuBoard>>,
    errors: Signal<u32>,
    hint_used: Signal<u32>,
    timer: Signal<u64>,
    selected: Signal<Option<(usize, usize)>>,
) -> Element {
    rsx! {
        div { class: "menu",
            div { class: "menu-hero",
                div { class: "grid-preview",
                    for _ in 0..81 {
                        div { class: "grid-preview-cell" }
                    }
                }
            }

            div { class: "menu-content",
                h2 { class: "menu-subtitle", "Choisissez votre niveau" }

                div { class: "difficulty-selector",
                    for diff in [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard] {
                        {
                            let is_selected = *difficulty.read() == diff;
                            let diff_clone = diff.clone();
                            rsx! {
                                button {
                                    class: if is_selected { "diff-btn selected" } else { "diff-btn" },
                                    onclick: move |_| { *difficulty.write() = diff_clone; },
                                    div { class: "diff-label", "{diff.label()}" }
                                    div { class: "diff-info",
                                        match diff {
                                            Difficulty::Easy => "45 indices",
                                            Difficulty::Medium => "35 indices",
                                            Difficulty::Hard => "25 indices",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                button {
                    class: "start-btn",
                    onclick: move |_| {
                        let new_board = SudokuBoard::new(*difficulty.read());
                        *board.write() = Some(new_board);
                        *errors.write() = 0;
                        *hint_used.write() = 0;
                        *timer.write() = 0;
                        *selected.write() = None;
                        *game_state.write() = GameState::Playing;
                    },
                    "NOUVELLE PARTIE"
                }
            }
        }
    }
}

#[component]
fn PlayScreen(
    board: Signal<Option<SudokuBoard>>,
    selected: Signal<Option<(usize, usize)>>,
    game_state: Signal<GameState>,
    input_mode: Signal<InputMode>,
    difficulty: Signal<Difficulty>,
    hint_used: Signal<u32>,
    errors: Signal<u32>,
    timer: Signal<u64>,
) -> Element {
    let progress = {
        if let Some(b) = board.read().as_ref() {
            (b.filled_count() as f32 / 81.0 * 100.0) as u32
        } else {
            0
        }
    };

    rsx! {
        div { class: "play-screen",
            // Barre de progression
            div { class: "progress-bar",
                div { class: "progress-fill", style: "width: {progress}%" }
            }

            // Grille
            Board {
                board: board.clone(),
                selected: selected.clone(),
                game_state: game_state.clone(),
                errors: errors.clone(),
            }

            // Mode de saisie
            div { class: "mode-toggle",
                button {
                    class: if *input_mode.read() == InputMode::Value { "mode-btn active" } else { "mode-btn" },
                    onclick: move |_| { *input_mode.write() = InputMode::Value; },
                    "✏️ Valeur"
                }
                button {
                    //class: if *input_mode.read() == InputMode::Notes { "mode-btn active" } else { "mode-btn" },
                    class: if *input_mode.read() == InputMode::Notes { "mode-btn active" } else {themes::MODE_BTN},
                    onclick: move |_| { *input_mode.write() = InputMode::Notes; },
                    "📝 Notes"
                }
            }

            // Pavé numérique
            NumPad {
                board: board.clone(),
                selected: selected.clone(),
                game_state: game_state.clone(),
                input_mode: input_mode.clone(),
                errors: errors.clone(),
            }

            // Contrôles
            Controls {
                board: board.clone(),
                selected: selected.clone(),
                game_state: game_state.clone(),
                hint_used: hint_used.clone(),
                difficulty: difficulty.clone(),
                errors: errors.clone(),
                timer: timer.clone(),
            }
        }
    }
}

#[component]
fn WonScreen(
    game_state: Signal<GameState>,
    board: Signal<Option<SudokuBoard>>,
    difficulty: Signal<Difficulty>,
    errors: Signal<u32>,
    hint_used: Signal<u32>,
    timer: Signal<u64>,
    selected: Signal<Option<(usize, usize)>>,
) -> Element {
    rsx! {
        div { class: "won-screen",
            div { class: "won-content",
                div { class: "won-emoji", "🎉" }
                h2 { "Félicitations !" }
                p { "Vous avez résolu le Sudoku !" }

                div { class: "won-stats",
                    div { class: "won-stat",
                        span { class: "won-stat-label", "Niveau" }
                        span { class: "won-stat-value", "{difficulty.read().label()}" }
                    }
                    div { class: "won-stat",
                        span { class: "won-stat-label", "Erreurs" }
                        span { class: "won-stat-value", "{errors}" }
                    }
                    div { class: "won-stat",
                        span { class: "won-stat-label", "Indices" }
                        span { class: "won-stat-value", "{hint_used}" }
                    }
                }

                div { class: "won-buttons",
                    button {
                        class: "start-btn",
                        onclick: move |_| {
                            let new_board = SudokuBoard::new(*difficulty.read());
                            *board.write() = Some(new_board);
                            *errors.write() = 0;
                            *hint_used.write() = 0;
                            *selected.write() = None;
                            *game_state.write() = GameState::Playing;
                        },
                        "REJOUER"
                    }
                    button {
                        class: "won-btn-menu",
                        onclick: move |_| {
                            *game_state.write() = GameState::Menu;
                        },
                        "MENU PRINCIPAL"
                    }
                }
            }
        }
    }
}
