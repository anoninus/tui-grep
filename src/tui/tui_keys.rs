use crate::tui::tui_structure::App;
use crate::engine::search::search_files;
use std::error::Error;

use crossterm::event::KeyCode;
use crossterm::{event, event::Event};

// we use bool to control the flow accurately without breaks or continue;
pub fn key_code(app: &mut App, dir: &str) -> Result<bool, Box<dyn Error>> {
    #[allow(clippy::collapsible_if)]
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    app.input.push(c);
                    app.result = search_files(&app.input, dir)?;
                }
                KeyCode::Backspace => {
                    app.input.pop();
                    app.result = search_files(&app.input, dir)?;
                }

                // break through true
                KeyCode::Esc => return Ok(true),
                KeyCode::Enter => {
                    if let Some(selected) = app.result.get(app.selected) {
                        println!("You selected: {}", selected);
                        return Ok(true);
                    }
                }

                KeyCode::Up => {
                    if app.selected > 0 {
                        app.selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if app.selected + 1 < app.result.len() {
                        app.selected += 1;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(false) // continue loop
}

