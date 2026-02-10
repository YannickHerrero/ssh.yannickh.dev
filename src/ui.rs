use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::{App, IntroPhase, Tab};
use crate::content;
use crate::theme;

/// Render the entire portfolio UI into the given frame.
pub fn render(app: &App, f: &mut Frame) {
    let area = f.area();

    // Clear the screen first
    f.render_widget(Clear, area);

    if !app.intro_done() {
        render_intro(app, f, area);
        return;
    }

    // ── Outer layout: header, tab bar, content, footer ─────────
    let chunks = Layout::vertical([
        Constraint::Length(banner_height() + 4), // banner + blank + subtitle + border
        Constraint::Length(3),                   // tab bar
        Constraint::Min(6),                      // content
        Constraint::Length(1),                   // footer
    ])
    .split(area);

    render_header(f, chunks[0]);
    render_tabs(app, f, chunks[1]);
    render_content(app, f, chunks[2]);
    render_footer(app, f, chunks[3]);
}

// ── Intro animation ────────────────────────────────────────────

fn render_intro(app: &App, f: &mut Frame, area: Rect) {
    let chars_shown = match app.intro {
        IntroPhase::Typing { chars_shown } => chars_shown,
        IntroPhase::Pause { .. } | IntroPhase::Done => content::banner_char_count(),
    };

    let max_w = banner_width();

    // Build the revealed portion of the banner.
    // Pad each line to the same width so centering keeps alignment.
    let mut lines: Vec<Line> = Vec::new();
    let mut remaining = chars_shown;

    for &banner_line in content::BANNER {
        if remaining == 0 {
            break;
        }
        let show = remaining.min(banner_line.len());
        let revealed = &banner_line[..show];

        // Pad the revealed portion to max banner width
        let padded = format!("{:<width$}", revealed, width = max_w);
        let mut spans = vec![Span::styled(padded, theme::HEADER)];

        // Show a blinking cursor at the end of the current typing line
        if show < banner_line.len() {
            spans.push(Span::styled("\u{2588}", theme::INTRO_CURSOR));
        }

        lines.push(Line::from(spans));

        // consume chars + 1 for the implicit newline
        remaining = remaining.saturating_sub(banner_line.len() + 1);
    }

    // Center the banner vertically
    let banner_h = lines.len() as u16;
    let y_offset = area.height.saturating_sub(banner_h) / 2;

    let banner_area = Rect {
        x: area.x,
        y: area.y + y_offset,
        width: area.width,
        height: banner_h.min(area.height.saturating_sub(y_offset)),
    };

    let text = Paragraph::new(Text::from(lines)).alignment(Alignment::Center);
    f.render_widget(text, banner_area);
}

// ── Header (ASCII banner) ──────────────────────────────────────

fn banner_height() -> u16 {
    content::BANNER.len() as u16
}

fn banner_width() -> usize {
    content::BANNER.iter().map(|l| l.len()).max().unwrap_or(0)
}

fn render_header(f: &mut Frame, area: Rect) {
    let max_w = banner_width();

    // Pad each banner line to the same width so Alignment::Center
    // shifts them as a uniform block instead of centering each
    // line independently (which breaks the ASCII art).
    let lines: Vec<Line> = content::BANNER
        .iter()
        .map(|&l| {
            let padded = format!("{:<width$}", l, width = max_w);
            Line::from(Span::styled(padded, theme::HEADER))
        })
        .collect();

    // Pad subtitle to the same width for consistent centering
    let subtitle_raw = "software engineer  \u{00b7}  France";
    let sub_pad_total = max_w.saturating_sub(subtitle_raw.chars().count());
    let sub_pad_left = sub_pad_total / 2;
    let sub_pad_right = sub_pad_total - sub_pad_left;

    let subtitle = Line::from(vec![
        Span::raw(" ".repeat(sub_pad_left)),
        Span::styled("software engineer", theme::TEXT_DIM),
        Span::styled("  \u{00b7}  ", theme::TEXT_MUTED),
        Span::styled("France", theme::TEXT_DIM),
        Span::raw(" ".repeat(sub_pad_right)),
    ]);

    let mut all_lines = lines;
    all_lines.push(Line::from(format!("{:<width$}", "", width = max_w)));
    all_lines.push(subtitle);

    let text = Paragraph::new(Text::from(all_lines))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(theme::BORDER),
        );
    f.render_widget(text, area);
}

// ── Tab bar ────────────────────────────────────────────────────

fn render_tabs(app: &App, f: &mut Frame, area: Rect) {
    let tabs: Vec<Span> = Tab::ALL
        .iter()
        .enumerate()
        .flat_map(|(i, t)| {
            let num = Span::styled(format!("{}", i + 1), theme::TAB_NUMBER);
            let sep = Span::styled(":", theme::TAB_NUMBER);
            let style = if *t == app.tab {
                theme::TAB_ACTIVE
            } else {
                theme::TAB_INACTIVE
            };
            let label = Span::styled(format!("{}", t.label()), style);
            let spacer = Span::raw("   ");
            vec![num, sep, label, spacer]
        })
        .collect();

    let line = Line::from(tabs);
    let paragraph = Paragraph::new(line).alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

// ── Content area ───────────────────────────────────────────────

fn render_content(app: &App, f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::BORDER)
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(area);
    f.render_widget(block, area);

    match app.tab {
        Tab::About => render_about(app, f, inner),
        Tab::Projects => render_projects(app, f, inner),
        Tab::Skills => render_skills(app, f, inner),
        Tab::Contact => render_contact(app, f, inner),
    }
}

// ── About tab ──────────────────────────────────────────────────

fn render_about(app: &App, f: &mut Frame, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    for (i, &line_str) in content::ABOUT_LINES.iter().enumerate() {
        if i == 0 {
            // First line is the greeting — make it bold
            lines.push(Line::from(Span::styled(line_str, theme::TEXT_BOLD)));
        } else if line_str.is_empty() {
            lines.push(Line::from(""));
        } else {
            lines.push(Line::from(Span::styled(line_str, theme::TEXT)));
        }
    }

    let total_lines = lines.len();
    let viewport_h = area.height as usize;

    let text = Paragraph::new(Text::from(lines))
        .scroll((app.scroll_offset as u16, 0))
        .wrap(Wrap { trim: false });
    f.render_widget(text, area);

    if total_lines > viewport_h {
        render_scroll_indicator(f, area, app.scroll_offset, total_lines, viewport_h);
    }
}

// ── Projects tab (scrollable) ──────────────────────────────────

fn render_projects(app: &App, f: &mut Frame, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    for (cat_idx, cat) in content::PROJECT_CATEGORIES.iter().enumerate() {
        if cat_idx > 0 {
            lines.push(Line::from(""));
        }

        // Category header
        lines.push(Line::from(vec![
            Span::styled("\u{2500}\u{2500} ", theme::BORDER),
            Span::styled(cat.name, theme::CATEGORY_HEADER),
            Span::styled(" \u{2500}\u{2500}", theme::BORDER),
        ]));
        lines.push(Line::from(""));

        for (proj_idx, project) in cat.projects.iter().enumerate() {
            if proj_idx > 0 {
                lines.push(Line::from(""));
            }

            lines.push(Line::from(Span::styled(project.name, theme::PROJECT_NAME)));
            lines.push(Line::from(Span::styled(
                project.description,
                theme::PROJECT_DESC,
            )));
            lines.push(Line::from(Span::styled(project.tech, theme::PROJECT_TECH)));
            lines.push(Line::from(Span::styled(project.url, theme::LINK)));
        }
    }

    let total_lines = lines.len();
    let viewport_h = area.height as usize;

    // Apply scroll offset
    let text = Paragraph::new(Text::from(lines))
        .scroll((app.scroll_offset as u16, 0))
        .wrap(Wrap { trim: false });
    f.render_widget(text, area);

    // Scroll indicator
    if total_lines > viewport_h {
        render_scroll_indicator(f, area, app.scroll_offset, total_lines, viewport_h);
    }
}

// ── Skills tab ─────────────────────────────────────────────────

fn render_skills(app: &App, f: &mut Frame, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    for (i, group) in content::SKILLS.iter().enumerate() {
        if i > 0 {
            lines.push(Line::from(""));
        }

        lines.push(Line::from(Span::styled(group.name, theme::SKILL_GROUP)));

        let items_str = group.items.join("  \u{00b7}  ");
        lines.push(Line::from(Span::styled(items_str, theme::SKILL_ITEM)));
    }

    let total_lines = lines.len();
    let viewport_h = area.height as usize;

    let text = Paragraph::new(Text::from(lines))
        .scroll((app.scroll_offset as u16, 0))
        .wrap(Wrap { trim: false });
    f.render_widget(text, area);

    if total_lines > viewport_h {
        render_scroll_indicator(f, area, app.scroll_offset, total_lines, viewport_h);
    }
}

// ── Contact tab ────────────────────────────────────────────────

fn render_contact(app: &App, f: &mut Frame, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(Span::styled(
        "Want to get in touch? Here's where you can find me:",
        theme::TEXT,
    )));
    lines.push(Line::from(""));

    for entry in content::CONTACT_ENTRIES {
        lines.push(Line::from(vec![
            Span::styled(format!("  {:<10}", entry.label), theme::CONTACT_LABEL),
            Span::styled(entry.value, theme::LINK),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        content::CONTACT_OUTRO,
        theme::TEXT_DIM,
    )));

    let total_lines = lines.len();
    let viewport_h = area.height as usize;

    let text = Paragraph::new(Text::from(lines))
        .scroll((app.scroll_offset as u16, 0))
        .wrap(Wrap { trim: false });
    f.render_widget(text, area);

    if total_lines > viewport_h {
        render_scroll_indicator(f, area, app.scroll_offset, total_lines, viewport_h);
    }
}

// ── Footer ─────────────────────────────────────────────────────

fn render_footer(_app: &App, f: &mut Frame, area: Rect) {
    let spans = vec![
        Span::styled(" h/l ", theme::KEY_HINT),
        Span::styled("navigate", theme::KEY_ACTION),
        Span::styled("  j/k ", theme::KEY_HINT),
        Span::styled("scroll", theme::KEY_ACTION),
        Span::styled("  tab ", theme::KEY_HINT),
        Span::styled("next", theme::KEY_ACTION),
        Span::styled("  q ", theme::KEY_HINT),
        Span::styled("quit", theme::KEY_ACTION),
    ];

    let help = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
    f.render_widget(help, area);
}

// ── Scroll indicator ───────────────────────────────────────────

fn render_scroll_indicator(
    f: &mut Frame,
    area: Rect,
    offset: usize,
    total: usize,
    viewport: usize,
) {
    let max_scroll = total.saturating_sub(viewport);
    if max_scroll == 0 {
        return;
    }

    // Show a small position indicator at the top-right of the content area
    let pct = if max_scroll > 0 {
        (offset * 100) / max_scroll
    } else {
        0
    };

    let indicator = if offset == 0 {
        "\u{2191} top".to_string()
    } else if offset >= max_scroll {
        "\u{2193} end".to_string()
    } else {
        format!("{}%", pct)
    };

    let indicator_area = Rect {
        x: area.x + area.width.saturating_sub(indicator.len() as u16 + 1),
        y: area.y,
        width: (indicator.len() as u16).min(area.width),
        height: 1,
    };

    let text = Paragraph::new(Span::styled(indicator, theme::SCROLL_INDICATOR));
    f.render_widget(text, indicator_area);
}
