use ratatui::style::{Color, Modifier, Style};

// ── Header ─────────────────────────────────────────────────────
pub const HEADER: Style = Style::new().fg(Color::White).add_modifier(Modifier::BOLD);
// ── Tabs ───────────────────────────────────────────────────────
pub const TAB_ACTIVE: Style = Style::new()
    .fg(Color::White)
    .add_modifier(Modifier::BOLD)
    .add_modifier(Modifier::UNDERLINED);
pub const TAB_INACTIVE: Style = Style::new().fg(Color::DarkGray);
pub const TAB_NUMBER: Style = Style::new().fg(Color::DarkGray);

// ── Borders ────────────────────────────────────────────────────
pub const BORDER: Style = Style::new().fg(Color::DarkGray);

// ── Text ───────────────────────────────────────────────────────
pub const TEXT: Style = Style::new().fg(Color::White);
pub const TEXT_DIM: Style = Style::new().fg(Color::Gray);
pub const TEXT_MUTED: Style = Style::new().fg(Color::DarkGray);
pub const TEXT_BOLD: Style = Style::new().fg(Color::White).add_modifier(Modifier::BOLD);

// ── Links ──────────────────────────────────────────────────────
pub const LINK: Style = Style::new()
    .fg(Color::Blue)
    .add_modifier(Modifier::UNDERLINED);

// ── Projects ───────────────────────────────────────────────────
pub const PROJECT_NAME: Style = Style::new().fg(Color::White).add_modifier(Modifier::BOLD);
pub const PROJECT_DESC: Style = Style::new().fg(Color::Gray);
pub const PROJECT_TECH: Style = Style::new().fg(Color::DarkGray);

pub const CATEGORY_HEADER: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);

// ── Skills ─────────────────────────────────────────────────────
pub const SKILL_GROUP: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
pub const SKILL_ITEM: Style = Style::new().fg(Color::White);

// ── Contact ────────────────────────────────────────────────────
pub const CONTACT_LABEL: Style = Style::new()
    .fg(Color::DarkGray)
    .add_modifier(Modifier::BOLD);
// ── Footer ─────────────────────────────────────────────────────
pub const KEY_HINT: Style = Style::new().fg(Color::White).add_modifier(Modifier::BOLD);
pub const KEY_ACTION: Style = Style::new().fg(Color::DarkGray);

// ── Scroll indicator ───────────────────────────────────────────
pub const SCROLL_INDICATOR: Style = Style::new().fg(Color::DarkGray);

// ── Intro animation ────────────────────────────────────────────
pub const INTRO_CURSOR: Style = Style::new()
    .fg(Color::White)
    .add_modifier(Modifier::RAPID_BLINK);
