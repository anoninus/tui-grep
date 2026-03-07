
mod tui;
mod engine;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "rgrep",
    about = "The fzf line implementation for grep in my own style",
allow_external_subcommands = true, // <- ignores unknown args
)]
struct Cli {
    #[arg(short, long)]
    recursive: bool,

    #[arg(short, long)]
    ignore_case: bool,
}


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    if args.recursive {
        println!("Trigger recursive tui");
    }
    if args.ignore_case {
        println!("Trigger ignore_case tui");
    }
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;

    let backend = ratatui::prelude::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    crate::tui::tui_core::run(&mut terminal)?;

    // disable
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture,
    )?;

    terminal.show_cursor()?;

    Ok(())
}

