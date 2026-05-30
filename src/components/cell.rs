use crate::sudoku::Cell;
use super::styles_tw4::*;
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

   // 1. On prépare une String pour accumuler les flags
    let mut flags = String::new();

    if is_selected { flags.push_str(" selected"); }
    else if is_same_value { flags.push_str(" same-value"); }
    else if is_highlighted { flags.push_str(" highlighted"); }

    if cell.is_given { flags.push_str(" given"); }
    if cell.is_invalid { flags.push_str(" invalid"); }

    // On gère la bordure proprement en utilisant format! si elle n'est pas vide
    if !border_class.is_empty() {
        flags = format!("{} {}", flags, border_class);
    }

    // 2. LA FUSION CRUCIALE : On combine la constante et tous les flags d'un coup
    let final_classes = format!("{} {}", ST_CELL, flags.trim());
    // --- AJOUTE CES DEUX LIGNES POUR TESTER ---
println!("--- DÉBOGAGE CASE ---");
println!("ST_CELL vaut: '{}' | flags vaut: '{}'", ST_CELL, flags);
println!("Chaîne finale envoyée à class: '{}'", final_classes);

// ------------------------------------------
    rsx! {
        div {
            class: final_classes,
            onclick: move |e| on_click.call(e),

            if let Some(value) = cell.value {
                span { class: format!("{} {}", ST_CELL_VALUE, flags.trim()), "{value}" }
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
