use super::styles_tw4::*;
use crate::sudoku::Cell;
//use dioxus::{html::g::format, prelude::*};
use dioxus::prelude::*;

#[component]
pub fn CellComponent(
    row: usize,
    col: usize,
    cell: Cell,
    is_selected: bool,
    is_highlighted: bool,
    is_same_value: bool,
    border_class: &'static str,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    // On prépare les classes Cell et Cell-Value car elles ont un peut de traitement spécifique
 
    // On traite le bg- (fond) car il diffère en fonction de la situation de la cellule
    let cell_class = format!(
        "{} {} {}",
        ST_CELL,
        match (is_selected, is_same_value, is_highlighted, cell.is_invalid) {
            (true, _, _, false)         => "selected",
            (false, true, _, false)     => "same-value",
            (false, false, true, false) => "highlighted",
            (false, _, _, true)         => "invalid",
            (true, _, _, true)          => "invalid-selected",
            _                           => "default",
        },
        border_class
    );

    // On traite le text- (couleur) car il diffère en fonction de la situation de la cellule
    let cell_value_class = format!(
        "{} {}",
        ST_CELL_VALUE,
        match (cell.is_given, cell.is_invalid) {
            (true, false) => "given",
            (_, true) => "invalid",
            _ => "default",
        }
    );

    // ------------------------------------------
    rsx! {
        div {
            class: cell_class,
            onclick: move |e| on_click.call(e),

            if let Some(value) = cell.value {
                span { class: cell_value_class, "{value}" }
            } else if cell.notes.iter().any(|&n| n) {
                div { class: "{ST_NOTES_GRID}",
                    for i in 0..9usize {
                        div { class: "{ST_NOTE}",
                            if cell.notes[i] {
                                "{i + 1}"
                            }
                        }
                    }
                }
            }
        }
    }
}
