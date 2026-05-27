use crate::sudoku::Cell;
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

//   let mut class = String::from("cell");
//
//   if is_selected {
//       class.push_str(" selected");
//   } else if is_same_value {
//       class.push_str(" same-value");
//   } else if is_highlighted {
//       class.push_str(" highlighted");
//   }
//
//   if cell.is_given {
//       class.push_str(" given");
//   }
//
//   if cell.is_invalid {
//       class.push_str(" invalid");
//   }
//
//   if !border_class.is_empty() {
//       class.push(' ');
//       class.push_str(border_class);   // Bordures des boîtes 3x3
//   }


    rsx! {
        div {
            class: format!("{}{}{}{}{}",
                "cell",
                match (is_selected, is_same_value, is_highlighted) {
                    (true, _, _)         => " selected",
                    (false, true, _)     => " same-value",
                    (false, false, true) => " highlighted",
                    _                    => "",
                },
                if cell.is_given   {" cell-given cell-given-value"} else {" "},
                if cell.is_invalid {" cell-invalid cell-invalid-value"} else {" "},
                border_class
            ),
            onclick: move |e| on_click.call(e),

            if let Some(value) = cell.value {
                span { class: "cell-value", "{value}" }
            } else if cell.notes.iter().any(|&n| n) {
                div { class: "notes-grid",
                    for i in 0..9usize {
                        div { class: "note",
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
