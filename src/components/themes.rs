    /* ══════════════════════════════════════════════
       LAYOUT
    ══════════════════════════════════════════════ */
    pub const APP: &'static str = r#"
        @apply min-h-screen flex flex-col max-w-[480px] mx-auto;
    "#;

    
    /* ── Header ── */
    pub const HEADER: &'static str = r#"
        @apply flex items-center justify-between px-5 pt-4 pb-3
        border-b border--cc_border bg--cc_surface;
    "#;

    
    pub const HEADER_TITLE: &'static str = r#"
        @apply flex items-center gap-2.5;
    "#;

    
    pub const TITLE_ICON: &'static str = r#"
        @apply text-xl animate-pulse-icon;
    "#;

    
    pub const HEADER_H1: &'static str = r#"
        @apply font-display text-[32px] tracking-[4px] text--cc_accent-2;
        text-shadow: 0 0 20px var(---cc_accent-glow);
    "#;

    
    pub const HEADER_STATS: &'static str = r#"
        @apply flex gap-4;
    "#;

    
    pub const STAT: &'static str = r#"
        @apply flex flex-col items-center gap-[2px];
    "#;

    
    pub const STAT_LABEL: &'static str = r#"
        @apply text-[9px] text--cc_text-muted tracking-[1px];
    "#;

    
    pub const STAT_VALUE: &'static str = r#"
        @apply text-base font-bold text--cc_accent-2;
    "#;

    
    /* ── Main ── */
    pub const MAIN: &'static str = r#"
        @apply flex-1 flex flex-col p-4;
    "#;

    
    /* ══════════════════════════════════════════════
       MENU
    ══════════════════════════════════════════════ */
    pub const MENU: &'static str = r#"
        @apply flex-1 flex flex-col gap-6;
    "#;

    
    pub const GRID_PREVIEW: &'static str = r#"
        @apply grid grid-cols-9 gap-[2px] opacity-[.15] p-4;
    "#;

    
    pub const GRID_PREVIEW_CELL: &'static str = r#"
        @apply aspect-square bg--cc_accent rounded-[2px] animate-flicker;
        animation-delay: calc(var(--i, 0) * 0.05s);
    "#;

    
    pub const MENU_CONTENT: &'static str = r#"
        @apply flex flex-col gap-5 px-1;
    "#;

    
    pub const MENU_SUBTITLE: &'static str = r#"
        @apply font-mono text-xs text--cc_text-muted uppercase tracking-[2px] text-center;
    "#;

    
    /* ── Sélecteur de difficulté ── */
    pub const DIFFICULTY_SELECTOR: &'static str = r#"
        @apply flex flex-col gap-2;
    "#;

    
    pub const DIFF_BTN: &'static str = r#"
        @apply bg--cc_surface border border--cc_border text--cc_text py-[14px] px-[18px] rounded-lg
        cursor-pointer flex items-center justify-between
        transition-all duration-200 font-mono;
    "#;

    
    pub const DIFF_BTN_HOVER: &'static str = r#"
        @apply border--cc_accent bg--cc_surface-2;
    "#;

    
    pub const DIFF_BTN_SELECTED: &'static str = r#"
        @apply border--cc_accent bg--cc_selected;
        box-shadow: 0 0 12px var(---cc_accent-glow);
    "#;

    
    pub const DIFF_LABEL: &'static str = r#"
        @apply font-bold text-sm tracking-[1px];
    "#;

    
    pub const DIFF_INFO: &'static str = r#"
        @apply text-[11px] text--cc_text-muted;
    "#;

    
    /* ── Bouton Start ── */
    pub const START_BTN: &'static str = r#"
        @apply bg--cc_accent border-0 text-white py-4 rounded-[10px]
        font-display text-[22px] tracking-[3px]
        cursor-pointer transition-all duration-200;
        box-shadow: 0 4px 20px var(---cc_accent-glow);
    "#;

    
    pub const START_BTN_HOVER: &'static str = r#"
        @apply bg--cc_accent-2 -translate-y-0.5;
        box-shadow: 0 6px 24px var(---cc_accent-glow);
    "#;

    
    pub const START_BTN_ACTIVE: &'static str = r#"
        @apply translate-y-0;
    "#;

    
    /* ══════════════════════════════════════════════
       ÉCRAN DE JEU
    ══════════════════════════════════════════════ */
    pub const PLAY_SCREEN: &'static str = r#"
        @apply flex flex-col items-center gap-[14px];
    "#;

    
    /* ── Barre de progression ── */
    pub const PROGRESS_BAR: &'static str = r#"
        @apply w-full h-[3px] bg--cc_surface-2 rounded-sm overflow-hidden;
    "#;

    
    pub const PROGRESS_FILL: &'static str = r#"
        @apply h-full rounded-sm transition-[width] duration-[400ms] ease-out;
        background: linear-gradient(90deg, var(--color--cc_accent), var(--color--cc_accent-2));
    "#;

    
    /* ── Plateau ── */
    pub const BOARD: &'static str = r#"
        display: grid;
        grid-template-columns: repeat(9, var(--cell-size));
        grid-template-rows:    repeat(9, var(--cell-size));
        @apply border-2 border--cc_border-box rounded-md overflow-hidden;
        box-shadow: 0 0 40px rgba(124, 106, 247, 0.1);
    "#;

    
    /* ── Cellule ── */
    pub const CELL: &'static str = r#"
        width: var(--cell-size);
        height: var(--cell-size);
        @apply flex items-center justify-center bg--cc_surface border border--cc_border
        cursor-pointer relative transition-colors duration-150 select-none;
    "#;

    
    /* Bordures épaisses des boîtes 3×3 */
    //.cell.border-top,
    //.cell.border-top-left,
    pub const CELL_BORDER_TOP_RIGHT: &'static str = r#"border-top:    2px solid var(--color--cc_border-box);"#;
    
    //.cell.border-left,
    //.cell.border-top-left,
    pub const CELL_BORDER_BOTTOM_LEFT: &'static str = r#"border-left:   2px solid var(--color--cc_border-box);"#;
    
    //.cell.border-bottom,
    //.cell.border-bottom-left,
    pub const CELL_BORDER_BOTTOM_RIGHT: &'static str = r#"border-bottom: 2px solid var(--color--cc_border-box);"#;
    
    //.cell.border-right,
    //.cell.border-top-right,
    pub const CELL_BORDER_BOTTOM: &'static str = r#"border-right:  2px solid var(--color--cc_border-box);"#;
    
    pub const CELL_HOVER: &'static str = r#"@apply bg--cc_surface-2;"#;
    pub const CELL_HIGHLIGHTED: &'static str = r#"@apply bg--cc_highlight;"#;
    pub const CELL_SAME_VALUE: &'static str = r#"@apply bg--cc_same-value;"#;
    
    pub const CELL_SELECTED: &'static str = r#"
        @apply bg--cc_selected;
        box-shadow: inset 0 0 0 2px var(--color--cc_accent);
    "#;

    
    pub const CELL_INVALID: &'static str = r#"background: rgba(248, 113, 113, 0.08);"#;
    pub const CELL_INVALID_CELL_VALUE: &'static str = r#"color: var(--color--cc_invalid) !important;"#;
    
    pub const CELL_VALUE: &'static str = r#"
        @apply font-mono font-bold text--cc_accent-2 leading-none;
        font-size: calc(var(--cell-size) * 0.44);
    "#;

    
    pub const CELL_GIVEN_CELL_VALUE: &'static str = r#"
        @apply text--cc_given font-bold;
    "#;

    
    /* ── Notes ── */
    pub const NOTES_GRID: &'static str = r#"
        @apply grid grid-cols-3 grid-rows-3 w-full h-full p-px;
    "#;

    
    pub const NOTE: &'static str = r#"
        @apply flex items-center justify-center text--cc_text-muted leading-none;
        font-size: calc(var(--cell-size) * 0.2);
    "#;

    
    /* ══════════════════════════════════════════════
       CONTRÔLES
    ══════════════════════════════════════════════ */
    
    /* ── Mode toggle ── */
    pub const MODE_TOGGLE: &'static str = r#"
        @apply flex gap-2 w-full;
    "#;

    
    pub const MODE_BTN: &'static str = r#"
        @apply flex-1 p-2 bg--cc_surface border border--cc_border text--cc_text-muted rounded-lg
        cursor-pointer font-mono text-xs transition-all duration-200;
    "#;

    
    pub const MODE_BTN_ACTIVE: &'static str = r#"
        @apply bg--cc_selected border--cc_accent text--cc_accent-2;
    "#;

    
    /* ── Numpad ── */
    pub const NUMPAD: &'static str = r#"
        grid grid-cols-5 gap-2 w-full
    "#;

    
    pub const NUMPAD_BTN: &'static str = r#"
        @apply aspect-square bg--cc_surface border border--cc_border text--cc_text font-mono font-bold
        rounded-[10px] cursor-pointer transition-all duration-150
        flex items-center justify-center
        hover:bg--cc_surface-2 hover:border--cc_accent hover:text--cc_accent-2 hover:translate-y-px
        active:translate-y-2;
    "#;

    
    /* .numpad-btn:hover {
      @apply hover:bg--cc_surface-2 hover:border--cc_accent hover:text--cc_accent-2 hover:translate-y-px;
    }
    
    pub const NUMPAD_BTN_ACTIVE: &'static str = r#"
        @apply translate-y-0;
    "#;

    */ 
    
    pub const NUMPAD_BTN_NUMBER: &'static str = r#"
        @apply text--cc_text;
        font-size: clamp(16px, 4vw, 22px);
    "#;

    pub const NUMPAD_BTN_ERASE: &'static str = r#"
        @apply text--cc_text-muted;
        font-size: clamp(16px, 4vw, 50px);
    "#;

    
    /* ── Boutons d'action ── */
    pub const CONTROLS: &'static str = r#"
        @apply flex gap-2 w-full;
    "#;

    
    pub const CTRL_BTN: &'static str = r#"
        @apply flex-1 py-2.5 px-2 bg--cc_surface border border--cc_border text--cc_text-muted rounded-lg
        cursor-pointer font-mono text-[11px]
        flex flex-col items-center gap-1 transition-all duration-200;
    "#;

    
    pub const CTRL_BTN_SPAN: &'static str = r#"
        @apply text-lg;
    "#;

    
    pub const CTRL_BTN_HOVER: &'static str = r#"
        @apply bg--cc_surface-2 text--cc_text border--cc_border-box;
    "#;

    
    pub const HINT_BTN_HOVER: &'static str = r#"
        @apply border-[#fbbf24] text-[#fbbf24];
    "#;

    
    /* ══════════════════════════════════════════════
       ÉCRAN DE VICTOIRE
    ══════════════════════════════════════════════ */
    pub const WON_SCREEN: &'static str = r#"
        @apply flex-1 flex items-center justify-center;
    "#;

    
    pub const WON_CONTENT: &'static str = r#"
        @apply flex flex-col items-center gap-5 text-center py-8 px-6
        bg--cc_surface border border--cc_border-box rounded-2xl;
        box-shadow: 0 0 40px var(---cc_accent-glow);
    "#;

    
    pub const WON_EMOJI: &'static str = r#"
        @apply text-[64px] animate-bounce-won;
    "#;

    
    pub const WON_CONTENT_H2: &'static str = r#"
        @apply font-display text-[40px] tracking-[3px] text--cc_success;
    "#;

    
    pub const WON_CONTENT_P: &'static str = r#"
        @apply text--cc_text-muted text-[13px];
    "#;

    
    pub const WON_STATS: &'static str = r#"
        @apply flex gap-6;
    "#;

    
    pub const WON_STAT: &'static str = r#"
        @apply flex flex-col items-center gap-1;
    "#;

    
    pub const WON_STAT_LABEL: &'static str = r#"
        @apply text-[10px] text--cc_text-muted tracking-[1px] uppercase;
    "#;

    
    pub const WON_STAT_VALUE: &'static str = r#"
        @apply text-2xl font-bold text--cc_accent-2;
    "#;

    
    pub const WON_BUTTONS: &'static str = r#"
        @apply flex flex-col gap-2.5 w-full;
    "#;

    
    pub const WON_BTN_MENU: &'static str = r#"
        @apply bg-transparent border border--cc_border text--cc_text-muted py-3 rounded-[10px]
        font-mono text-xs cursor-pointer transition-all duration-200
        tracking-[2px] uppercase;
    "#;

    
    pub const WON_BTN_MENU_HOVER: &'static str = r#"
        @apply border--cc_accent text--cc_accent-2;
    "#;
