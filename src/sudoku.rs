use rand::prelude::*;
//use rand::seq::SliceRandom;
//use rand::thread_rng;

pub const GRID_SIZE: usize = 9;
pub const BOX_SIZE: usize = 3;

/// Représente l'état d'une cellule
#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub value: Option<u8>, // None = vide, Some(1-9) = valeur
    pub is_given: bool,    // true = donné au départ (non modifiable)
    pub is_invalid: bool,  // true = conflit détecté
    pub notes: [bool; 9],  // notes (candidats) de 1 à 9
}

impl Cell {
    pub fn empty() -> Self {
        Cell {
            value: None,
            is_given: false,
            is_invalid: false,
            notes: [false; 9],
        }
    }

    pub fn given(value: u8) -> Self {
        Cell {
            value: Some(value),
            is_given: true,
            is_invalid: false,
            notes: [false; 9],
        }
    }
}

/// Niveau de difficulté
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn clues_count(&self) -> usize {
        match self {
            Difficulty::Easy => 45,
            Difficulty::Medium => 35,
            Difficulty::Hard => 25,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Facile",
            Difficulty::Medium => "Moyen",
            Difficulty::Hard => "Difficile",
        }
    }
}

/// Grille de Sudoku
#[derive(Clone, Debug)]
pub struct SudokuBoard {
    pub cells: [[Cell; GRID_SIZE]; GRID_SIZE],
    pub solution: [[u8; GRID_SIZE]; GRID_SIZE],
}

impl SudokuBoard {
    pub fn new(difficulty: Difficulty) -> Self {
        let solution = generate_solution();
        let cells = create_puzzle(&solution, difficulty.clues_count());
        SudokuBoard { cells, solution }
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: Option<u8>) {
        if self.cells[row][col].is_given {
            return;
        }
        self.cells[row][col].value = value;
        self.cells[row][col].notes = [false; 9];
        self.validate_all();
    }

    pub fn toggle_note(&mut self, row: usize, col: usize, note: u8) {
        if self.cells[row][col].is_given || self.cells[row][col].value.is_some() {
            return;
        }
        let idx = (note - 1) as usize;
        self.cells[row][col].notes[idx] = !self.cells[row][col].notes[idx];
    }

    pub fn validate_all(&mut self) {
        // Réinitialise les invalides
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                self.cells[row][col].is_invalid = false;
            }
        }

        // Vérifie les lignes
        for row in 0..GRID_SIZE {
            self.validate_group(self.get_row_positions(row));
        }

        // Vérifie les colonnes
        for col in 0..GRID_SIZE {
            self.validate_group(self.get_col_positions(col));
        }

        // Vérifie les boîtes 3x3
        for box_row in 0..BOX_SIZE {
            for box_col in 0..BOX_SIZE {
                self.validate_group(self.get_box_positions(box_row, box_col));
            }
        }
    }

    fn validate_group(&mut self, positions: Vec<(usize, usize)>) {
        let mut seen: std::collections::HashMap<u8, Vec<(usize, usize)>> =
            std::collections::HashMap::new();

        for (r, c) in &positions {
            if let Some(v) = self.cells[*r][*c].value {
                seen.entry(v).or_default().push((*r, *c));
            }
        }

        for positions in seen.values() {
            if positions.len() > 1 {
                for (r, c) in positions {
                    self.cells[*r][*c].is_invalid = true;
                }
            }
        }
    }

    fn get_row_positions(&self, row: usize) -> Vec<(usize, usize)> {
        (0..GRID_SIZE).map(|c| (row, c)).collect()
    }

    fn get_col_positions(&self, col: usize) -> Vec<(usize, usize)> {
        (0..GRID_SIZE).map(|r| (r, col)).collect()
    }

    fn get_box_positions(&self, box_row: usize, box_col: usize) -> Vec<(usize, usize)> {
        let start_row = box_row * BOX_SIZE;
        let start_col = box_col * BOX_SIZE;
        let mut positions = Vec::new();
        for r in start_row..start_row + BOX_SIZE {
            for c in start_col..start_col + BOX_SIZE {
                positions.push((r, c));
            }
        }
        positions
    }

    pub fn is_solved(&self) -> bool {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                match self.cells[row][col].value {
                    None => return false,
                    Some(v) if v != self.solution[row][col] => return false,
                    _ => {}
                }
                if self.cells[row][col].is_invalid {
                    return false;
                }
            }
        }
        true
    }

    pub fn reveal_hint(&mut self) -> Option<(usize, usize)> {
        // Trouve une cellule vide et révèle sa solution
        let mut empty_cells: Vec<(usize, usize)> = Vec::new();
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if self.cells[row][col].value.is_none() && !self.cells[row][col].is_given {
                    empty_cells.push((row, col));
                }
            }
        }

        if empty_cells.is_empty() {
            return None;
        }

        let mut rng = rand::rng();
        let &(row, col) = empty_cells.choose(&mut rng).unwrap();
        self.cells[row][col].value = Some(self.solution[row][col]);
        self.cells[row][col].is_given = true; // Marque comme indice
        self.validate_all();
        Some((row, col))
    }

    pub fn filled_count(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .filter(|c| c.value.is_some())
            .count()
    }
}

/// Génère une solution complète valide
fn generate_solution() -> [[u8; GRID_SIZE]; GRID_SIZE] {
    let mut grid = [[0u8; GRID_SIZE]; GRID_SIZE];
    solve_backtrack(&mut grid, 0, 0);
    grid
}

fn solve_backtrack(grid: &mut [[u8; GRID_SIZE]; GRID_SIZE], row: usize, col: usize) -> bool {
    if row == GRID_SIZE {
        return true;
    }

    let (next_row, next_col) = if col + 1 == GRID_SIZE {
        (row + 1, 0)
    } else {
        (row, col + 1)
    };

    let mut numbers: Vec<u8> = (1..=9).collect();
    let mut rng = rand::rng();
    numbers.shuffle(&mut rng);

    for &num in &numbers {
        if is_safe(grid, row, col, num) {
            grid[row][col] = num;
            if solve_backtrack(grid, next_row, next_col) {
                return true;
            }
            grid[row][col] = 0;
        }
    }

    false
}

fn is_safe(grid: &[[u8; GRID_SIZE]; GRID_SIZE], row: usize, col: usize, num: u8) -> bool {
    // Vérifie la ligne
    if grid[row].contains(&num) {
        return false;
    }

    // Vérifie la colonne
    if (0..GRID_SIZE).any(|r| grid[r][col] == num) {
        return false;
    }

    // Vérifie la boîte 3x3
    let box_row = (row / BOX_SIZE) * BOX_SIZE;
    let box_col = (col / BOX_SIZE) * BOX_SIZE;
    for r in box_row..box_row + BOX_SIZE {
        for c in box_col..box_col + BOX_SIZE {
            if grid[r][c] == num {
                return false;
            }
        }
    }

    true
}

/// Crée un puzzle en retirant des cellules de la solution
fn create_puzzle(
    solution: &[[u8; GRID_SIZE]; GRID_SIZE],
    clues: usize,
) -> [[Cell; GRID_SIZE]; GRID_SIZE] {
    let mut cells: [[Cell; GRID_SIZE]; GRID_SIZE] =
        std::array::from_fn(|r| std::array::from_fn(|c| Cell::given(solution[r][c])));

    let mut positions: Vec<(usize, usize)> = (0..GRID_SIZE)
        .flat_map(|r| (0..GRID_SIZE).map(move |c| (r, c)))
        .collect();

    let mut rng = rand::rng();
    positions.shuffle(&mut rng);

    let cells_to_remove = GRID_SIZE * GRID_SIZE - clues;
    let mut removed = 0;

    for (row, col) in positions {
        if removed >= cells_to_remove {
            break;
        }
        cells[row][col] = Cell::empty();
        removed += 1;
    }

    cells
}
