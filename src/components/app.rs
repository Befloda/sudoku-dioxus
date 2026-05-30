use super::board::Board;
use super::controls::Controls;
use super::numpad::NumPad;
use crate::sudoku::{Difficulty, SudokuBoard};
use super::styles_tw4::*;
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

        div { class: "{ST_APP}",
            // Header


            header { class: "{ST_HEADER}",
                div { class: "{ST_TITLE}",
                    span { class: "{ST_TITLE_ICON}", "⬛" }
                    h1 { class: "{ST_TITLE_TEXT}", "SUDOKU" }
                }
                if *game_state.read() == GameState::Playing {
                    div { class: "{ST_STATS}",
                        div { class: "{ST_STAT}",
                            span { class: "{ST_STAT_LABEL}", "ERREURS" }
                            span { class: "{ST_STAT_VALUE}", "{errors}" }
                        }
                        div { class: "{ST_STAT}",
                            span { class: "{ST_STAT_LABEL}", "INDICES" }
                            span { class: "{ST_STAT_VALUE}", "{hint_used}" }
                        }
                        div { class: "{ST_STAT}",
                            span { class: "{ST_STAT_LABEL}", "NIVEAU" }
                            span { class: "{ST_STAT_VALUE}", "{difficulty.read().label()}" }
                        }
                    }
                }
            }

            main { class: "{ST_MAIN}",
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
        div { class: "{ST_MENU}",
            div { class: "menu-hero",
                div { class: "{ST_GRID_PREVIEW}",
                    for _ in 0..81 {
                        div { class: "{ST_GRID_PREVIEW_CELL}" }
                    }
                }
            }

            div { class: "{ST_MENU_CONTENT}",
                h2 { class: "{ST_MENU_SUBTITLE}", "Choisissez votre niveau" }

                div { class: "{ST_DIFFICULTY_SELECTOR}",
                    for diff in [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard] {
                        {
                            let is_selected = *difficulty.read() == diff;
                            let diff_clone = diff.clone();
                            rsx! {
                                button {
                                    class: if is_selected {
                                             "{ST_DIFF_BTN} selected"
                                         } else {
                                             "{ST_DIFF_BTN}"
                                         },
                                    onclick: move |_| { *difficulty.write() = diff_clone; },
                                    div { class: "{ST_DIFF_LABEL}", "{diff.label()}" }
                                    div { class: "{ST_DIFF_INFO}",
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
                    class: "{ST_START_BTN}",
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
        div { class: "{ST_PLAY_SCREEN}",
            // Barre de progression
            div { class: "{ST_PROGRESS_BAR}",
                div { class: "{ST_PROGRESS_FILL}", style: "width: {progress}%" }
            }

            // Grille
            Board {
                board: board.clone(),
                selected: selected.clone(),
                game_state: game_state.clone(),
                errors: errors.clone(),
            }

            // Mode de saisie
            div { class: "{ST_MODE_TOGGLE}",
                button {
                    class: if *input_mode.read() == InputMode::Value { "{ST_MODE_BTN} {ST_MODE_BTN_ACTIVE}" } else { "{ST_MODE_BTN}" },
                    onclick: move |_| { *input_mode.write() = InputMode::Value; },
                    "✏️ Valeur"
                }
                button {
                    class: if *input_mode.read() == InputMode::Notes { "{ST_MODE_BTN} {ST_MODE_BTN_ACTIVE}" } else { "{ST_MODE_BTN}" },
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
        div { class: "{ST_WON_SCREEN}",
            div { class: "{ST_WON_CONTENT}",
                div { class: "{ST_WON_EMOJI}", "🎉" }
                h2 { "Félicitations !" }
                p { "Vous avez résolu le Sudoku !" }

                div { class: "{ST_WON_STATS}",
                    div { class: "{ST_WON_STAT}",
                        span { class: "{ST_WON_STAT_LABEL}", "Niveau" }
                        span { class: "{ST_WON_STAT_VALUE}", "{difficulty.read().label()}" }
                    }
                    div { class: "{ST_WON_STAT}",
                        span { class: "{ST_WON_STAT_LABEL}", "Erreurs" }
                        span { class: "{ST_WON_STAT_VALUE}", "{errors}" }
                    }
                    div { class: "{ST_WON_STAT}",
                        span { class: "{ST_WON_STAT_LABEL}", "Indices" }
                        span { class: "{ST_WON_STAT_VALUE}", "{hint_used}" }
                    }
                }

                div { class: "{ST_WON_BUTTONS}",
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
                        class: "{ST_WON_BTN_MENU}",
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
