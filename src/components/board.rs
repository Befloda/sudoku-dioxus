use super::app::GameState;
use super::cell::CellComponent;
use crate::sudoku::SudokuBoard;
use super::styles_tw4::*; 
use dioxus::prelude::*;

#[component]
pub fn Board(
    board: Signal<Option<SudokuBoard>>,
    selected: Signal<Option<(usize, usize)>>,
    game_state: Signal<GameState>,
    errors: Signal<u32>,
) -> Element {
    let board_ref = board.read();
    let Some(b) = board_ref.as_ref() else {
        return rsx! { div { "Chargement..." } };
    };

    let selected_pos = *selected.read();

    // Calcule les cellules à surligner (même ligne, colonne, boîte, valeur)
    let highlighted: std::collections::HashSet<(usize, usize)> =
        if let Some((sr, sc)) = selected_pos {
            let mut h = std::collections::HashSet::new();
            // Même ligne
            for c in 0..9 {
                h.insert((sr, c));
            }
            // Même colonne
            for r in 0..9 {
                h.insert((r, sc));
            }
            // Même boîte 3x3
            let box_r = (sr / 3) * 3;
            let box_c = (sc / 3) * 3;
            for r in box_r..box_r + 3 {
                for c in box_c..box_c + 3 {
                    h.insert((r, c));
                }
            }
            h
        } else {
            std::collections::HashSet::new()
        };

    // Valeur sélectionnée pour surligner les mêmes valeurs
    let selected_value = selected_pos.and_then(|(r, c)| b.cells[r][c].value);

    rsx! {
        div { class: "{ST_BOARD}",
            for row in 0..9usize {
                for col in 0..9usize {
                    {
                        let cell = b.cells[row][col].clone();
                        let is_selected = selected_pos == Some((row, col));
                        let is_highlighted = highlighted.contains(&(row, col));
                        let is_same_value = selected_value.is_some()
                            && cell.value.is_some()
                            && cell.value == selected_value
                            && !is_selected;

                        // Bordures des boîtes 3x3
                        let border_class = get_border_class(row, col);

                        rsx! {
                            CellComponent {
                                key: "{row}-{col}",
                                row,
                                col,
                                cell,
                                is_selected,
                                is_highlighted,
                                is_same_value,
                                border_class,
                                on_click: move |_| {
                                    if *game_state.read() == GameState::Playing {
                                        let mut sel = selected.write();
                                        if *sel == Some((row, col)) {
                                            *sel = None;
                                        } else {
                                            *sel = Some((row, col));
                                        }
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

fn get_border_class(row: usize, col: usize) -> &'static str {
    match (row % 3 == 0, col % 3 == 0, row == 8, col == 8) {
        (true, true, _, _) => " border-top-left",
        (true, _, _, true) => " border-top-right",
        (_, true, true, _) => " border-bottom-left",
        (_, _, true, true) => " border-bottom-right",
        (true, _, _, _)    => " border-top",
        (_, true, _, _)    => " border-left",
        (_, _, true, _)    => " border-bottom",
        (_, _, _, true)    => " border-right",
        _                  => "",
    }
}
