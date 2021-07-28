use std::io;
use std::process;

use event::Event;
use event::Events;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::layout::Constraint;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::widgets::Paragraph;
use tui::{backend::TermionBackend, Terminal};

mod event;

fn main() -> Result<(), io::Error> {
    if let Err(e) = build() {
        eprintln!("{}", e);
        process::exit(1);
    }
    Ok(())
}

fn build() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = AlternateScreen::from(MouseTerminal::from(io::stdout().into_raw_mode()?));
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new(None);
    let mut buf = String::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());
            let text = Paragraph::new(format!("{}", buf));
            f.render_widget(text, chunks[0]);
        });

        // Handle input
        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') => break,
                Key::Char(key) => {
                    buf.push(key);
                }
                _ => {}
            }
        }
    }
    Ok(())
}
