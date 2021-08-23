#![allow(dead_code)]

// mod event;
mod fs;
mod option;
// mod ui;

use std::env;
use std::io::Stdout;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, path::Path, thread};

use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;
use tui::{backend::CrosstermBackend, widgets::Paragraph};

use crate::fs::Cache;
use crate::option::DisplayOptions;

use crossbeam_channel::Receiver;
use crossterm::{cursor, event::Event, execute, terminal};

pub fn setup() -> crossterm::Result<io::Stdout> {
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    execute!(stdout, cursor::Hide)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    terminal::enable_raw_mode()?;
    Ok(stdout)
}

pub fn cleanup(stdout: &mut Stdout) -> crossterm::Result<()> {
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

pub struct Listener(Receiver<Event>);

impl Listener {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        thread::spawn(move || loop {
            tx.send(crossterm::event::read().unwrap()).unwrap();
        });
        Self(rx)
    }

    pub fn get(&self) -> &Receiver<Event> {
        &self.0
    }
}

fn try_run(stdout: &mut Stdout, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // let event_listener = Arc::from(Listener::new().get());
    let session_cache = Arc::from(Mutex::from(Cache::new()));
    let options = DisplayOptions::new(false, false);
    let path = Path::new(path);

    session_cache
        .lock()
        .unwrap()
        .populate_to_root(path, &options)?;

    terminal.draw(|frame| {
        let path_str = path.to_str().unwrap();
        let body = format!(
            "\n{}\n{}\n{}\n",
            path_str,
            str::repeat("-", path_str.len()),
            session_cache
                .lock()
                .unwrap()
                .inner
                .get(path)
                .unwrap()
                .to_string()
                .trim_end()
        );

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame.size());

        frame.render_widget(Paragraph::new(body), chunks[0]);
    })?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = setup()?;
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            try_run(&mut stdout, &args[1])?;
            thread::sleep(Duration::from_secs(3));
            cleanup(&mut stdout)?;
        }
        _ => {
            eprintln!("please provide a valid file path");
        }
    }

    Ok(())
}
