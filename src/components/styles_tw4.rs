/* ══════════════════════════════════════════════
   LAYOUT
══════════════════════════════════════════════ */
pub const ST_APP: &'static str = r#"ST-APP min-h-screen flex flex-col max-w-120 mx-auto"#;

/* ── Header ── */
pub const ST_HEADER: &'static str = r#"ST-HEADER flex items-center justify-between px-5 pt-4 pb-3 border-b border-cc-border bg-cc-surface"#;

pub const ST_TITLE: &'static str = r#"ST-TITLE flex items-center gap-2.5"#;
pub const ST_TITLE_ICON: &'static str = r#"ST-TITLE-ICON text-xl animate-pulse-icon"#;
pub const ST_TITLE_TEXT: &'static str = r#"ST-TITLE-TEXT font-display text-[32px] tracking-[4px] text-cc-accent-2 [text-shadow:_0_0_20px_var(--cc-accent-glow)]"#;

pub const ST_STATS: &'static str = r#"ST-STATS flex gap-4"#;
pub const ST_STAT: &'static str = r#"ST-STAT flex flex-col items-center gap-0.5"#;
pub const ST_STAT_LABEL: &'static str = r#"ST-STAT-LABEL text-[9px] text-cc-text-muted tracking-[1px]"#;
pub const ST_STAT_VALUE: &'static str = r#"ST-STAT-VALUE text-base font-bold text-cc-accent-2"#;

/* ── Main ── */
pub const ST_MAIN: &'static str = r#"ST-MAIN flex-1 flex flex-col p-4"#;

/* ══════════════════════════════════════════════
   MENU
══════════════════════════════════════════════ */
pub const ST_MENU: &'static str = r#"ST-MENU flex-1 flex flex-col gap-6"#;

pub const ST_GRID_PREVIEW: &'static str = r#"ST-GRID-PREVIEW grid grid-cols-9 gap-0.5 opacity-[.15] p-4"#;

pub const ST_GRID_PREVIEW_CELL: &'static str = r#"ST-GRID-PREVIEW-CELL 
   aspect-square bg-cc-accent rounded-[2px] animate-flicker
   [animation-delay:_calc(var(--i,0)*0.05s)]
"#;


pub const ST_MENU_CONTENT: &'static str = r#"ST-MENU-CONTENT flex flex-col gap-5 px-1"#;

pub const ST_MENU_SUBTITLE: &'static str = r#"ST-MENU-SUBTITLE 
   font-mono text-xs text-cc-text-muted uppercase tracking-[2px] text-center
"#;


/* ── Sélecteur de difficulté ── */
pub const ST_DIFFICULTY_SELECTOR: &'static str = r#"ST-DIFFICULTY-SELECTOR flex flex-col gap-2"#;

pub const ST_DIFF_BTN: &'static str = r#"ST-DIFF-BTN 
   bg-cc-surface border border-cc-border text-cc-text py-3.5 px-4.5 rounded-lg
   cursor-pointer flex items-center justify-between
   transition-all duration-200 font-mono
   hover:border-cc-accent hover:bg-cc-surface-2
"#;


pub const ST_DIFF_BTN_SELECTED: &'static str = r#"ST-DIFF-BTN-SELECTED 
   border-cc-accent bg-cc-selected
   shadow-[0_0_12px_var(--cc-accent-glow)]
"#;


pub const ST_DIFF_LABEL: &'static str = r#"ST-DIFF-LABEL font-bold text-sm tracking-[1px]"#;

pub const ST_DIFF_INFO: &'static str = r#"ST-DIFF-INFO text-[11px] text-cc-text-muted"#;

/* ── Bouton Start ── */
pub const ST_START_BTN: &'static str = r#"ST-START-BTN 
   bg-cc-accent border-0 text-white py-4 rounded-[10px]
   font-display text-[22px] tracking-[3px]
   cursor-pointer transition-all duration-200
   shadow-[0_4px_20px_var(--cc-accent-glow)]
"#;


pub const ST_START_BTN_HOVER: &'static str = r#"ST-START-BTN-HOVER 
   bg-cc-accent-2 -translate-y-0.5
   shadow-[0_6px_24px_var(--cc-accent-glow)]
"#;


pub const ST_START_BTN_ACTIVE: &'static str = r#"ST-START-BTN-ACTIVE translate-y-0"#;

/* ══════════════════════════════════════════════
   ÉCRAN DE JEU
══════════════════════════════════════════════ */
pub const ST_PLAY_SCREEN: &'static str = r#"ST-PLAY-SCREEN flex flex-col items-center gap-3.5"#;

/* ── Barre de progression ── */
pub const ST_PROGRESS_BAR: &'static str = r#"ST-PROGRESS-BAR w-full h-0.75 bg-cc-surface-2 rounded-sm overflow-hidden"#;

pub const ST_PROGRESS_FILL: &'static str = r#"ST-PROGRESS-FILL 
   h-full rounded-sm transition-[width] duration-[400ms] ease-out
   bg-gradient-to-r from-[var(--color-cc-accent)] to-[var(--color-cc-accent-2)]
"#;


/* ── Plateau ── */
pub const ST_BOARD: &'static str = r#"ST-BOARD 
   grid grid-cols-[repeat(9,var(--cell-size))] grid-rows-[repeat(9,var(--cell-size))]
   border-2 border-cc-border-box rounded-md overflow-hidden
   shadow-[0_0_40px_rgba(124,106,247,0.1)]
"#;


/* ── Cellule ── */
pub const ST_CELL: &'static str = r#"ST-CELL 
   w-[var(--cell-size)] h-[var(--cell-size)]
   flex items-center justify-center bg-cc-surface border border-cc-border
   cursor-pointer relative transition-colors duration-150 select-none
"#;


/* Bordures épaisses des boîtes 3×3 */
pub const ST_CELL_BORDER_TOP: &'static str = r#"ST-CELL-BORDER-TOP border-t-2 border-solid border-cc-border-box"#;
pub const ST_CELL_BORDER_TOP_LEFT: &'static str = r#"ST-CELL-BORDER-TOP-LEFT border-t-2 border-solid border-cc-border-box"#;
pub const ST_CELL_BORDER_TOP_RIGHT: &'static str = r#"ST-CELL-BORDER-TOP-RIGHT border-t-2 border-solid border-cc-border-box"#;

pub const ST_CELL_BORDER_LEFT: &'static str = r#"ST-CELL-BORDER-LEFT border-l-2 border-solid border-cc-border-box"#;
pub const ST_CELL_BORDER_LEFT_TOP: &'static str = r#"ST-CELL-BORDER-LEFT-TOP border-l-2 border-solid border-cc-border-box"#;
pub const ST_CELL_BORDER_LEFT_BOTTOM: &'static str = r#"ST-CELL-BORDER-LEFT-BOTTOM border-l-2 border-solid border-cc-border-box"#;

pub const ST_CELL_BORDER_BOTTOM: &'static str = r#"ST-CELL-BORDER-BOTTOM border-b-2 border-solid border-cc-border-box"#;
pub const ST_CELL_BORDER_BOTTOM_LEFT: &'static str = r#"ST-CELL-BORDER-BOTTOM-LEFT border-b-2 border-solid border-cc-border-box"#;
pub const ST_CELL_BORDER_BOTTOM_RIGHT: &'static str = r#"ST-CELL-BORDER-BOTTOM-RIGHT border-b-2 border-solid border-cc-border-box"#;

pub const ST_CELL_BORDER_RIGHT: &'static str = r#"ST-CELL-BORDER-RIGHT border-r-2 border-solid border-cc-border-box"#;
pub const ST_CELL_BORDER_RIGHT_TOP: &'static str = r#"ST-CELL-BORDER-RIGHT-TOP border-r-2 border-solid border-cc-border-box"#;
pub const ST_CELL_BORDER_RIGHT_BOTTOM: &'static str = r#"ST-CELL-BORDER-RIGHT-BOTTOM border-r-2 border-solid border-cc-border-box"#;

pub const ST_CELL_HOVER: &'static str = r#"ST-CELL-HOVER bg-cc-surface-2"#;
pub const ST_CELL_HIGHLIGHTED: &'static str = r#"ST-CELL-HIGHLIGHTED bg-cc-highlight"#;
pub const ST_CELL_SAME_VALUE: &'static str = r#"ST-CELL-SAME-VALUE bg-cc-same-value"#;

pub const ST_CELL_SELECTED: &'static str = r#"ST-CELL-SELECTED bg-cc-selected shadow-md ring-2 ring-cc-accent ring-inset"#;

/* ── Valeurs de la cellule ── */
pub const ST_CELL_VALUE: &'static str = r#"ST-CELL-VALUE font-mono font-bold text-cc-accent-2 text-[calc(var(--cell-size)*0.44)] leading-none"#;

pub const ST_CELL_INVALID: &'static str = r#"ST-CELL-INVALID bg-[rgba(248,113,113,0.08)]"#;
pub const ST_CELL_INVALID_VALUE: &'static str = r#"ST-CELL-INVALID-VALUE [&_.cell-value]:text-cc-invalid"#;

pub const ST_CELL_GIVEN_VALUE: &'static str = r#"ST-CELL-GIVEN-VALUE [&_.cell-value]:text-cc-given"#;

/* ── Notes ── */
pub const ST_NOTES_GRID: &'static str = r#"ST-NOTES-GRID grid grid-cols-3 grid-rows-3 w-full h-full p-px"#;

pub const ST_NOTE: &'static str = r#"ST-NOTE flex items-center justify-center text-cc-text-muted text-[calc(var(--cell-size)*0.2)] leading-none"#;

/* ══════════════════════════════════════════════
   CONTRÔLES
══════════════════════════════════════════════ */

/* ── Mode toggle ── */
pub const ST_MODE_TOGGLE: &'static str = r#"ST-MODE-TOGGLE flex gap-2 w-full"#;

pub const ST_MODE_BTN: &'static str = r#"ST-MODE-BTN 
   flex-1 p-2 bg-cc-surface border border-cc-border text-cc-text-muted rounded-lg
   cursor-pointer font-mono text-xs transition-all duration-200
"#;


pub const ST_MODE_BTN_ACTIVE: &'static str = r#"ST-MODE-BTN-ACTIVE bg-cc-selected border-cc-accent text-cc-accent-2"#;

/* ── Numpad ── */
pub const ST_NUMPAD: &'static str = r#"ST-NUMPAD grid grid-cols-5 gap-2 w-full"#;

pub const ST_NUMPAD_BTN: &'static str = r#"ST-NUMPAD-BTN 
   aspect-square bg-cc-surface border border-cc-border text-cc-text font-mono font-bold
   rounded-[10px] cursor-pointer transition-all duration-150
   flex items-center justify-center
   hover:bg-cc-surface-2 hover:border-cc-accent hover:text-cc-accent-2 hover:translate-y-px
   active:translate-y-0
"#;


pub const ST_NUMPAD_BTN_NUMBER: &'static str = r#"ST-NUMPAD-BTN-NUMBER text-cc-text text-[clamp(14px,3.5vw,20px)]"#;
pub const ST_NUMPAD_BTN_DELETE: &'static str = r#"ST-NUMPAD-BTN-DELETE text-cc-text-muted text-[clamp(16px,4vw,40px)]"#;

/* ── Boutons d'action ── */
pub const ST_CONTROLS: &'static str = r#"ST-CONTROLS flex gap-2 w-full"#;

pub const ST_CTRL_BTN: &'static str = r#"ST-CTRL-BTN 
   flex-1 py-2.5 px-2 bg-cc-surface border border-cc-border text-cc-text-muted rounded-lg
   cursor-pointer font-mono text-[11px]
   flex flex-col items-center gap-1 transition-all duration-200
"#;


pub const ST_CTRL_BTN_SPAN: &'static str = r#"ST-CTRL-BTN-SPAN text-lg"#;

pub const ST_CTRL_BTN_HOVER: &'static str = r#"ST-CTRL-BTN-HOVER hover:bg-cc-surface-2 hover:text-cc-text hover:border-cc-border-box"#;

pub const ST_HINT_BTN_HOVER: &'static str = r#"ST-HINT-BTN-HOVER hover:border-[#fbbf24] hover:text-[#fbbf24]"#;

/* ══════════════════════════════════════════════
   ÉCRAN DE VICTOIRE
══════════════════════════════════════════════ */
pub const ST_WON_SCREEN: &'static str = r#"ST-WON-SCREEN flex-1 flex items-center justify-center"#;

pub const ST_WON_CONTENT: &'static str = r#"ST-WON-CONTENT 
   flex flex-col items-center gap-5 text-center py-8 px-6
   bg-cc-surface border border-cc-border-box rounded-2x
   shadow-[0_0_40px_var(--cc-accent-glow)]
"#;


pub const ST_WON_EMOJI: &'static str = r#"ST-WON-EMOJI text-[64px] animate-bounce-won"#;

pub const ST_WON_CONTENT_H2: &'static str = r#"ST-WON-CONTENT-H2 font-display text-[40px] tracking-[3px] text-cc-success"#;
pub const ST_WON_CONTENT_P: &'static str = r#"ST-WON-CONTENT-P text-cc-text-muted text-[13px]"#;

pub const ST_WON_STATS: &'static str = r#"ST-WON-STATS flex gap-6"#;

pub const ST_WON_STAT: &'static str = r#"ST-WON-STAT flex flex-col items-center gap-1"#;

pub const ST_WON_STAT_LABEL: &'static str = r#"ST-WON-STAT-LABEL text-[10px] text-cc-text-muted tracking-[1px] uppercase"#;

pub const ST_WON_STAT_VALUE: &'static str = r#"ST-WON-STAT-VALUE text-2xl font-bold text-cc-accent-2"#;

pub const ST_WON_BUTTONS: &'static str = r#"ST-WON-BUTTONS flex flex-col gap-2.5 w-full"#;

pub const ST_WON_BTN_MENU: &'static str = r#"ST-WON-BTN-MENU 
   bg-transparent border border-cc-border text-cc-text-muted py-3 rounded-[10px]
   font-mono text-xs cursor-pointer transition-all duration-200
   tracking-[2px] uppercase
   hover:border-cc-accent hover:text-cc-accent-2
"#;
