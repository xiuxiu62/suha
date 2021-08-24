#![allow(dead_code)]

mod event;
mod fs;
mod option;
// mod ui;

use event::handle_event;

use std::io::{self, Stdout};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, thread};

use crossterm::cursor;
use crossterm::{execute, terminal};
use structopt::{clap::Shell, StructOpt};
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;
use tui::{backend::CrosstermBackend, widgets::Paragraph};

#[derive(Debug, StructOpt)]
#[structopt(name = "suha")]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

use crate::event::{Command, Worker};
use crate::fs::Cache;
use crate::option::DisplayOptions;

const FPS: u64 = 120;

pub fn setup() -> crossterm::Result<io::Stdout> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )?;

    terminal::enable_raw_mode()?;
    Ok(stdout)
}

pub fn cleanup(stdout: &mut Stdout) -> crossterm::Result<()> {
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

async fn try_run(stdout: &mut Stdout, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let session_cache = Arc::from(Mutex::from(Cache::new()));
    let options = DisplayOptions::new(false, true);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let event_worker = Worker::new();

    // Fill cache to from provided path to the root directory
    session_cache
        .lock()
        .unwrap()
        .populate_to_root(path, &options)?;

    // Clone worker receiver for async sending
    let receiver = event_worker.clone_receiver();
    loop {
        // Draw frame buffer
        terminal.draw(|frame| {
            let body = session_cache
                .lock()
                .unwrap()
                .inner
                .get(path)
                .unwrap()
                .to_string();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(frame.size());

            frame.render_widget(Paragraph::new(body), chunks[0]);
        })?;

        // Handle events
        if let Some(command) = handle_event(receiver.clone()).await {
            match command {
                Command::Exit => break,
                Command::Debug(s) => println!("{}", s),
                _ => {}
            }
        };

        // draw FPS frames / second
        thread::sleep(Duration::from_millis(1_000 / FPS));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // generate `bash` completions in "target" directory
    Opt::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");

    let opts = Opt::from_args();
    match opts.file {
        Some(path) => {
            let mut stdout = setup()?;
            if let Err(e) = try_run(&mut stdout, path.as_path()).await {
                eprintln!("{}", e)
            };
            cleanup(&mut stdout)?;
        }
        None => eprintln!("\nplease provide a valid file path"),
    }

    Ok(())
}
