use std::{
    io::{stdout, Stdout},
    thread,
};

use crossbeam_channel::Receiver;
use crossterm::{cursor, event::Event, execute, terminal};

pub fn setup() -> crossterm::Result<()> {
    let mut stdout = stdout();

    execute!(stdout, terminal::EnterAlternateScreen)?;
    execute!(stdout, cursor::Hide)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    terminal::enable_raw_mode()?;
    Ok(())
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
