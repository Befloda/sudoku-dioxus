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
    border_class: String,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    // On prépare les classes Cell et Cell-Value car elles ont un peut de traitement spécifique
 
    // On traite le bg- (fond) car il diffère en fonction de la situation de la cellule
    let cell_class = format!(
        "{} {} {}",
        ST_CELL,
        match (cell.is_invalid, is_selected, is_same_value, is_highlighted) {
            (true, false, _, _) => "bg-cc-invalid",     // "invalid"
            (true, true, _, _)  => "bg-cc-invalid shadow-md ring-2 ring-cc-invalid-2 ring-inset", //"invalid-selected"
            (_, true, _, _,)    => "bg-cc-selected shadow-md ring-2 ring-cc-accent ring-inset", //"selected"
            (_, _, true, _)     => "bg-cc-same-value",  //"same-value"
            (_, _, _, true)     => "bg-cc-highlight",   //"highlighted"
            _                   => "bg-cc-surface",     //"default"
        },
        border_class
    );

    // On traite le text- (couleur) car il diffère en fonction de la situation de la cellule
    let cell_value_class = format!(
        "{} {}",
        ST_CELL_VALUE,
        match (cell.is_invalid,cell.is_given) {
            (true, _)   => "text-cc-invalid-2", //"invalid"
            (_, true)   => "text-cc-given",     //"given"
            _           => "text-cc-accent-2",  //"default"
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
