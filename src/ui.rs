use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::{App, Tab};

/// Render the entire portfolio UI into the given frame.
pub fn render(app: &App, f: &mut Frame) {
    let area = f.area();

    // Clear the screen first
    f.render_widget(Clear, area);

    // ── Outer layout: header, content, footer ──────────────────
    let chunks = Layout::vertical([
        Constraint::Length(3), // header / title
        Constraint::Length(3), // tab bar
        Constraint::Min(6),    // content
        Constraint::Length(1), // footer
    ])
    .split(area);

    render_header(f, chunks[0]);
    render_tabs(app, f, chunks[1]);
    render_content(app, f, chunks[2]);
    render_footer(f, chunks[3]);
}

fn render_header(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("yannick herrero")
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    f.render_widget(title, area);
}

fn render_tabs(app: &App, f: &mut Frame, area: Rect) {
    let tabs: Vec<Span> = Tab::ALL
        .iter()
        .map(|t| {
            let style = if *t == app.tab {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            Span::styled(format!("  {}  ", t.label()), style)
        })
        .collect();

    let line = Line::from(tabs);
    let paragraph = Paragraph::new(line).alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn render_content(app: &App, f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(area);
    f.render_widget(block, area);

    match app.tab {
        Tab::About => render_about(app, f, inner),
        Tab::Projects => render_projects(app, f, inner),
        Tab::Contact => render_contact(app, f, inner),
    }
}

fn render_about(app: &App, f: &mut Frame, area: Rect) {
    let text = Paragraph::new(app.about_text())
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });
    f.render_widget(text, area);
}

fn render_projects(app: &App, f: &mut Frame, area: Rect) {
    let projects = app.projects();

    let mut lines: Vec<Line> = Vec::new();

    for (i, project) in projects.iter().enumerate() {
        if i > 0 {
            lines.push(Line::from(""));
        }

        lines.push(Line::from(vec![Span::styled(
            project.name,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]));

        lines.push(Line::from(vec![Span::styled(
            project.description,
            Style::default().fg(Color::Gray),
        )]));

        lines.push(Line::from(vec![Span::styled(
            format!("{}", project.tech),
            Style::default().fg(Color::DarkGray),
        )]));

        lines.push(Line::from(vec![Span::styled(
            project.url,
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::UNDERLINED),
        )]));
    }

    let text = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false });
    f.render_widget(text, area);
}

fn render_contact(app: &App, f: &mut Frame, area: Rect) {
    let text = Paragraph::new(app.contact_text())
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });
    f.render_widget(text, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let help = Paragraph::new(Line::from(vec![
        Span::styled(
            " \u{2190}/\u{2192} ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("navigate", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "  tab ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("next", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "  q ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("quit", Style::default().fg(Color::DarkGray)),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(help, area);
}
