use crate::fs::Cache;

use crossterm::{cursor, execute, terminal};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::Paragraph,
};

use std::{
    io::{stdout, Stdout},
    path::Path,
};

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
            let body = cache.get(path).unwrap().to_string();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(frame.size());

            frame.render_widget(Paragraph::new(body), chunks[0]);
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
