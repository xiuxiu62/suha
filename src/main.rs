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

use crossbeam_channel::Receiver;
use crossterm::cursor::{self, position};
use crossterm::event::{Event, EventStream, KeyCode};
use crossterm::{execute, terminal};
use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;
use tui::{backend::CrosstermBackend, widgets::Paragraph};

use crate::fs::Cache;
use crate::option::DisplayOptions;

const HELP: &'static str = r#"help"#;

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

async fn print_events() {
    let mut reader = EventStream::new();

    loop {
        let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = reader.next().fuse();

        select! {
            _ = delay => {},
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        println!("Event::{:?}\r", event);

                        if event == Event::Key(KeyCode::Char('c').into()) {
                            println!("Cursor position: {:?}\r", position());
                        }

                        if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };
    }
}

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

fn try_run(stdout: &mut Stdout, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let session_cache = Arc::from(Mutex::from(Cache::new()));
    let options = DisplayOptions::new(false, false);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    session_cache
        .lock()
        .unwrap()
        .populate_to_root(path, &options)?;

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

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = setup()?;
    let args: Vec<String> = env::args().collect();
    if let 2 = args.len() {
        match args[1].as_str() {
            "-h" | "--help" => println!("{}", HELP),
            s => {
                try_run(&mut stdout, Path::new(s))?;
                print_events().await;
                cleanup(&mut stdout)?;
                return Ok(());
            }
        }
    }

    eprintln!("please provide a valid file path");
    Ok(())
}
