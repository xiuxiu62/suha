use std::io::{self, Stdout};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crossterm::cursor;
use crossterm::{execute, terminal};
use tokio::time::sleep;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;
use tui::{backend::CrosstermBackend, widgets::Paragraph};

use crate::event::{handle_event, Command, Worker};
use crate::fs::Cache;
use crate::option::DisplayOptions;

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

fn draw(
    terminal: &mut Terminal<CrosstermBackend<&mut Stdout>>,
    session_cache: Arc<Mutex<Cache>>,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
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

pub async fn run(
    stdout: &mut Stdout,
    path: &Path,
    fps: u64,
) -> Result<(), Box<dyn std::error::Error>> {
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
        draw(&mut terminal, session_cache.clone(), path)?;

        // Handle events
        if let Some(command) = handle_event(receiver.clone()).await {
            match command {
                Command::Exit => break,
                Command::Debug(s) => println!("{}", s),
                _ => {}
            }
        };

        // draw FPS frames / second
        sleep(Duration::from_millis(1_000 / fps)).await;
    }

    Ok(())
}

pub fn cleanup(stdout: &mut Stdout) -> crossterm::Result<()> {
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
