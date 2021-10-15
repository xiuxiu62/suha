use crate::{context::Context, fs::Cache};
use crossterm::{cursor, execute, terminal};
use std::{
    io::{self, Stdout},
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::time::sleep;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::Paragraph,
    Terminal,
};

// initializes the terminal
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

// cleans up the terminal
pub fn cleanup(stdout: &mut Stdout) -> crossterm::Result<()> {
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

// TODO: push into ui module
fn draw(
    terminal: &mut Terminal<CrosstermBackend<&mut Stdout>>,
    cache: &Cache,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|frame| {
        let body = cache.inner.get(path).unwrap().to_string();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame.size());

        frame.render_widget(Paragraph::new(body), chunks[0]);
    })?;

    Ok(())
}

// executes the event loop
pub async fn run(
    stdout: &mut Stdout,
    file_path: PathBuf,
    fps: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize crossterm terminal
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // Create context and clone a worker receiver
    let mut context = Context::new();

    // Initialize cache from path
    context
        .cache
        .populate_to_root(&file_path, &context.config)?;

    loop {
        draw(&mut terminal, &context.cache, &file_path)?;

        if context.worker.handle_event() {
            break;
        };

        // Draw `fps` frames / second
        sleep(Duration::from_millis(1_000 / fps)).await;
    }

    Ok(())
}
