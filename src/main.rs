#![allow(dead_code)]

// mod event;
mod fs;
mod option;
// mod ui;

use std::env;
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

pub fn cleanup(stdout: &mut io::Stdout) -> crossterm::Result<()> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => try_run(&args[1]),
        _ => {
            eprintln!("please provide a valid file path");
            Ok(())
        }
    }
}

fn try_run(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = setup()?;
    {
        let backend = CrosstermBackend::new(&mut stdout);
        let mut terminal = Terminal::new(backend)?;

        let session_cache = Arc::from(Mutex::from(Cache::new()));
        let options = DisplayOptions::new(false, true);
        let path = Path::new(path);

        {
            session_cache
                .lock()
                .unwrap()
                .populate_to_root(path, &options)?;
        }

        terminal.draw(|f| {
            let body = session_cache
                .lock()
                .unwrap()
                .to_string()
                .trim_end()
                .to_string()
                + "\n\n";
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());
            f.render_widget(Paragraph::new(body), chunks[0]);
        })?;
    }

    thread::sleep(Duration::from_secs(3));
    Ok(cleanup(&mut stdout)?)
}

// let dir = session_cache.inner.get(&path.to_path_buf()).unwrap();
// println!("{:?}\n", path);
// dir.inner.iter().for_each(|e| {
//     if let Ok(preview) = e.preview(1000) {
//         println!("\"{}\"\n{}\n", e.label, preview);
//     }
// });
