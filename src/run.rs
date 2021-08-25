use std::io::{self, Stdout};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crossterm::cursor;
use crossterm::{execute, terminal};
use tokio::time::sleep;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;
use tui::{backend::CrosstermBackend, widgets::Paragraph};

use crate::context::Context;
use crate::event::handle_event;
use crate::fs::Cache;

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
    file_path: PathBuf,
    config_path: PathBuf,
    fps: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create context
    let context = Arc::new(Context::new(config_path));

    // Initialize crossterm terminal
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Clone worker receiver for async sending
    let receiver = context.event_worker.clone().clone_receiver();

    // Initialize cache from path
    context
        .session_cache
        .lock()
        .unwrap()
        .populate_to_root(&file_path, &context.config)?;

    loop {
        draw(&mut terminal, context.session_cache.clone(), &file_path)?;

        if handle_event(&receiver) {
            break;
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
