use crate::{context::Context, event::Command, fs::Cache, ui::Component};

use crossterm::{cursor, execute, terminal};

use tokio::sync::Mutex;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

use std::{
    error::Error,
    io::{stdout, Stdout},
    path::Path,
    sync::Arc,
};

macro_rules! constraints {
	( $( $constraint:expr ),* ) => ([ $( Constraint::Percentage( $constraint ), )* ].as_ref())
}

pub type Terminal = tui::Terminal<CrosstermBackend<Stdout>>;

pub struct Painter {
    terminal: Terminal,
    context: Arc<Mutex<Context>>,
    command: Command,
}

impl Painter {
    pub fn new(context: Arc<Mutex<Context>>) -> crossterm::Result<Self> {
        let mut stdout = stdout();
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All)
        )?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        terminal::enable_raw_mode()?;

        Ok(Self {
            terminal,
            context,
            command: Command::None,
        })
    }

    pub async fn update(&mut self) {
        if let Ok(command) = self.context.lock().await.worker.receive_command().await {
            self.command = command
        };
    }

    pub async fn render(&mut self, cache: &Cache, path: &Path) -> Result<(), Box<dyn Error>> {
        let command = self.command.to_string();
        let config = self.context.lock().await.config.clone();

        self.as_mut().draw(|frame| {
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(constraints![95, 5])
                .split(frame.size());

            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints![30, 40, 30])
                .split(vertical_chunks[0]);

            // 0'th because all vertical chunk heights are the same
            // let general_chunk_height = vertical_chunks[0].height as usize - 3;

            let default_block = Block::default().borders(Borders::ALL);

            let directory = cache.get(path).unwrap();
            let title = &directory.path;
            let body = directory.to_string();

            let widget = directory.parent(&config).unwrap().unwrap().draw();
            frame.render_widget(widget, horizontal_chunks[0]);

            let widget = directory.draw();
            frame.render_widget(widget, horizontal_chunks[1]);
            frame.render_widget(
                default_block.clone().title("[ Preview ]"),
                horizontal_chunks[2],
            );
            frame.render_widget(
                Paragraph::new(command).block(default_block.clone().title("[ Command ]")),
                vertical_chunks[1],
            )
        })?;
        Ok(())
    }

    pub async fn cleanup(&mut self) -> crossterm::Result<()> {
        execute!(self.as_mut().backend_mut(), terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}

impl AsRef<Terminal> for Painter {
    fn as_ref(&self) -> &Terminal {
        &self.terminal
    }
}

impl AsMut<Terminal> for Painter {
    fn as_mut(&mut self) -> &mut Terminal {
        &mut self.terminal
    }
}
