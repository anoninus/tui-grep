use crate::tui::{
    tui_keys,
    tui_structure::{self},
};
use std::io::Stdout;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

pub fn run(
    term: &mut ratatui::Terminal<ratatui::prelude::CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = tui_structure::App::new();
    let dir = "./";

    loop {
        term.draw(|f| {
            let size = f.area();

            // centered container (fzf-like floating window)
            let outer = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ])
                .split(size);

            let main = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(5),
                    Constraint::Percentage(90),
                    Constraint::Percentage(5),
                ])
                .split(outer[1]);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),  // input
                    Constraint::Length(10), // results
                    Constraint::Min(1),     // preview
                ])
                .split(main[1]);

            /*
            -----------------------
            INPUT BOX
            -----------------------
            */

            let input = Paragraph::new(app.input.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .title(" RustGrep ")
                        .title_style(Style::default().add_modifier(Modifier::BOLD))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                );

            f.render_widget(input, chunks[0]);

            /*
            -----------------------
            RESULT LIST
            -----------------------
            */

            let items: Vec<ListItem> = app
                .result
                .iter()
                .map(|r| ListItem::new(r.as_str()))
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .title(" Results ")
                        .title_style(Style::default().add_modifier(Modifier::BOLD))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("❯ ");

            let mut list_state = ListState::default();
            list_state.select(Some(app.selected));

            f.render_stateful_widget(list, chunks[1], &mut list_state);

            /*
            -----------------------
            PREVIEW PANEL
            -----------------------
            */
            let preview_text = if let Some(entry) = app.result.get(app.selected) {
                let path = entry.split(':').next().unwrap_or(entry);

                if let Ok(content) = std::fs::read_to_string(path) {
                    content.lines().take(40).collect::<Vec<_>>().join("\n")
                } else {
                    format!("Cannot preview file: {}", path)
                }
            } else {
                "No file selected".to_string()
            };

            let preview = Paragraph::new(preview_text)
                .block(
                    Block::default()
                        .title(" Preview ")
                        .title_style(Style::default().add_modifier(Modifier::BOLD))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .wrap(Wrap { trim: true });

            f.render_widget(preview, chunks[2]);
        })?;

        // key handling
        if tui_keys::key_code(&mut app, dir)? {
            break;
        }
    }

    Ok(())
}
