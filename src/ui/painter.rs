use crate::fs::Cache;

use crossterm::{cursor, execute, terminal};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

use std::{
    io::{stdout, Stdout},
    path::Path,
};

macro_rules! constraints {
	( $( $constraint:expr ),* ) => ([ $( Constraint::Percentage( $constraint ), )* ].as_ref())
}

pub type Terminal = tui::Terminal<CrosstermBackend<Stdout>>;

pub struct Painter(Terminal);

impl Painter {
    pub fn new() -> crossterm::Result<Self> {
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

        Ok(Self(terminal))
    }

    pub async fn render(&mut self, cache: &Cache, path: &Path) -> crossterm::Result<()> {
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

            frame.render_widget(
                default_block.clone().title("[ Parent ]"),
                horizontal_chunks[0],
            );
            frame.render_widget(
                Paragraph::new(body).block(
                    default_block
                        .clone()
                        .title(format!("[ {} ]", title.to_string_lossy().as_ref())),
                ),
                horizontal_chunks[1],
            );
            frame.render_widget(
                default_block.clone().title("[ Preview ]"),
                horizontal_chunks[2],
            );
            frame.render_widget(
                default_block.clone().title("[ Command ]"),
                vertical_chunks[1],
            );
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
        &self.0
    }
}

impl AsMut<Terminal> for Painter {
    fn as_mut(&mut self) -> &mut Terminal {
        &mut self.0
    }
}
