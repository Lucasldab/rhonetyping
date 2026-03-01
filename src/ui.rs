use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
};

use crate::app::{App, CharState, Screen};
use crate::snippets::MENU_OPTIONS;

// ── Palette ──────────────────────────────────────────────────────────────────
const FG: Color        = Color::Rgb(220, 215, 205); // warm off-white
const DIM: Color       = Color::Rgb(90, 85, 80);    // untyped grey
const GREEN: Color     = Color::Rgb(130, 195, 120); // correct
const RED: Color       = Color::Rgb(210, 90, 80);   // wrong
const YELLOW: Color    = Color::Rgb(220, 185, 80);  // accent / cursor
const BG: Color        = Color::Rgb(22, 20, 18);    // dark background
const BORDER: Color    = Color::Rgb(60, 55, 50);    // subtle border
const TITLE_FG: Color  = Color::Rgb(200, 160, 90);  // warm gold title

pub fn draw(f: &mut Frame, app: &App) {
    // Full-screen dark background
    let area = f.area();
    f.render_widget(
        Block::default().style(Style::default().bg(BG)),
        area,
    );

    match app.screen {
        Screen::Menu    => draw_menu(f, app, area),
        Screen::Typing  => draw_typing(f, app, area),
        Screen::Results => draw_results(f, app, area),
    }
}

// ── Menu ─────────────────────────────────────────────────────────────────────

fn draw_menu(f: &mut Frame, app: &App, area: Rect) {
    let chunks = centered_rect(50, 60, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BORDER))
        .title(Span::styled(
            "  rhonetyping  ",
            Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);

    f.render_widget(block, chunks);

    let inner = inner_rect(chunks, 2);

    // Subtitle
    let subtitle = Paragraph::new("select a mode and press enter")
        .style(Style::default().fg(DIM))
        .alignment(Alignment::Center);
    f.render_widget(subtitle, inner);

    // Menu items - vertically centered
    let item_height = MENU_OPTIONS.len() as u16 * 2;
    let start_y = inner.y + (inner.height.saturating_sub(item_height)) / 2 + 2;

    for (i, lang) in MENU_OPTIONS.iter().enumerate() {
        let y = start_y + i as u16 * 2;
        if y >= inner.y + inner.height { break; }

        let item_area = Rect { x: inner.x, y, width: inner.width, height: 1 };

        let (prefix, style) = if i == app.selected_menu {
            (
                "▶  ",
                Style::default().fg(YELLOW).add_modifier(Modifier::BOLD),
            )
        } else {
            (
                "   ",
                Style::default().fg(DIM),
            )
        };

        let line = Paragraph::new(format!("{}{}", prefix, lang.label()))
            .style(style)
            .alignment(Alignment::Center);
        f.render_widget(line, item_area);
    }

    // Footer hint
    let footer_y = chunks.y + chunks.height.saturating_sub(2);
    let footer_area = Rect { x: chunks.x, y: footer_y, width: chunks.width, height: 1 };
    let footer = Paragraph::new("↑↓ navigate   enter select   q quit")
        .style(Style::default().fg(DIM))
        .alignment(Alignment::Center);
    f.render_widget(footer, footer_area);
}

// ── Typing ────────────────────────────────────────────────────────────────────

fn draw_typing(f: &mut Frame, app: &App, area: Rect) {
    let outer = centered_rect(85, 80, area);

    // Stats bar at top
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // stats
            Constraint::Min(6),     // snippet
            Constraint::Length(3),  // progress bar
            Constraint::Length(1),  // hint
        ])
        .split(outer);

    draw_stats_bar(f, app, layout[0]);
    draw_snippet(f, app, layout[1]);
    draw_progress(f, app, layout[2]);

    let hint = Paragraph::new("esc → menu   backspace → delete")
        .style(Style::default().fg(DIM))
        .alignment(Alignment::Center);
    f.render_widget(hint, layout[3]);
}

fn draw_stats_bar(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BORDER));
    f.render_widget(block, area);

    let inner = inner_rect(area, 1);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(inner);

    let lang_text = Paragraph::new(format!("  {}", app.language.label()))
        .style(Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Left);

    let wpm_text = Paragraph::new(format!("{:.0} wpm", app.wpm))
        .style(Style::default().fg(FG).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);

    let elapsed = app.elapsed().as_secs();
    let acc_text = Paragraph::new(format!("{:.1}% acc   {:02}:{:02}  ", app.accuracy, elapsed / 60, elapsed % 60))
        .style(Style::default().fg(FG))
        .alignment(Alignment::Right);

    f.render_widget(lang_text, chunks[0]);
    f.render_widget(wpm_text, chunks[1]);
    f.render_widget(acc_text, chunks[2]);
}

fn draw_snippet(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BORDER));
    f.render_widget(block.clone(), area);

    let inner = inner_rect(area, 2);

    // Build styled spans from char states
    let mut lines: Vec<Line> = Vec::new();
    let mut current_line: Vec<Span> = Vec::new();

    for (i, ch) in app.chars.iter().enumerate() {
        let style = if i == app.cursor {
            // cursor position
            Style::default().fg(BG).bg(YELLOW)
        } else {
            match app.char_states[i] {
                CharState::Untyped => Style::default().fg(DIM),
                CharState::Correct => Style::default().fg(GREEN),
                CharState::Wrong   => Style::default().fg(RED).add_modifier(Modifier::UNDERLINED),
            }
        };

        if *ch == '\n' {
            // Add a newline marker span then push the line
            if i == app.cursor {
                current_line.push(Span::styled("↵", Style::default().fg(BG).bg(YELLOW)));
            }
            lines.push(Line::from(current_line.clone()));
            current_line.clear();
        } else {
            current_line.push(Span::styled(ch.to_string(), style));
        }
    }

    // Cursor at end of last line
    if app.cursor == app.chars.len() {
        current_line.push(Span::styled(" ", Style::default().bg(YELLOW)));
    }

    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
    }

    let para = Paragraph::new(lines)
        .wrap(Wrap { trim: false });
    f.render_widget(para, inner);
}

fn draw_progress(f: &mut Frame, app: &App, area: Rect) {
    let pct = (app.progress() * 100.0) as u16;
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(BORDER)))
        .gauge_style(Style::default().fg(YELLOW).bg(BG))
        .percent(pct)
        .label(format!("{}%", pct));
    f.render_widget(gauge, area);
}

// ── Results ───────────────────────────────────────────────────────────────────

fn draw_results(f: &mut Frame, app: &App, area: Rect) {
    let panel = centered_rect(48, 55, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(YELLOW))
        .title(Span::styled(
            "  results  ",
            Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);
    f.render_widget(block, panel);

    let inner = inner_rect(panel, 2);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // spacer
            Constraint::Length(1), // wpm
            Constraint::Length(1), // spacer
            Constraint::Length(1), // accuracy
            Constraint::Length(1), // spacer
            Constraint::Length(1), // time
            Constraint::Length(1), // spacer
            Constraint::Length(1), // errors
            Constraint::Min(2),    // spacer
            Constraint::Length(1), // actions
        ])
        .split(inner);

    let elapsed = app.elapsed();
    let secs = elapsed.as_secs_f64();

    let stat_rows: &[(usize, &str, String, Color)] = &[
        (1, "WPM",      format!("{:.0}", app.wpm),      YELLOW),
        (3, "Accuracy", format!("{:.1}%", app.accuracy), GREEN),
        (5, "Time",     format!("{:.1}s", secs),          FG),
        (7, "Errors",   format!("{}", app.errors),        if app.errors == 0 { GREEN } else { RED }),
    ];

    for (idx, label, value, color) in stat_rows {
        let row = Paragraph::new(Line::from(vec![
            Span::styled(format!("{:<12}", label), Style::default().fg(DIM)),
            Span::styled(value.clone(), Style::default().fg(*color).add_modifier(Modifier::BOLD)),
        ])).alignment(Alignment::Center);
        f.render_widget(row, layout[*idx]);
    }

    let actions = Paragraph::new("enter/r retry   n new snippet   esc menu")
        .style(Style::default().fg(DIM))
        .alignment(Alignment::Center);
    f.render_widget(actions, layout[9]);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Returns a centered rectangle of given percentage width/height
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Shrinks rect by given margin on all sides
fn inner_rect(r: Rect, margin: u16) -> Rect {
    Rect {
        x: r.x + margin,
        y: r.y + margin,
        width: r.width.saturating_sub(margin * 2),
        height: r.height.saturating_sub(margin * 2),
    }
}
