use std::{error::Error, fs, io::stderr, path::PathBuf};

use clap::Parser;

use app::App;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{
            DisableMouseCapture,
            EnableMouseCapture
        },
        execute,
        terminal::{
            disable_raw_mode,
            enable_raw_mode,
            EnterAlternateScreen,
            LeaveAlternateScreen
        }
    },
    Terminal
};

mod app;


/// Simple program to exercise your punch speed
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// FILE that you want to display
    #[arg(value_name = "FILE")]
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>>{
    let args = Args::parse();
    let file_name = args.file.to_str().unwrap_or("Empty Content").to_owned();
    let content = fs::read_to_string(args.file).unwrap_or_default();

    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen, EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

    App::new(content.lines().collect(), file_name).run(&mut terminal)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;

    Ok(())
}
