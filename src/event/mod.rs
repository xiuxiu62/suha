use crossbeam_channel::Receiver;
use crossterm::cursor::position;
use crossterm::event::{Event, KeyCode};

mod command;
mod context;
mod worker;

pub use command::Command;
pub use context::Context;
pub use worker::Worker;

// Parse key events sent from worker listener
pub async fn parse_event(receiver: Receiver<Event>) -> Option<Command> {
    // try to recieve an event, returning nothing on error
    match receiver.try_recv() {
        Ok(event) => match event {
            Event::Key(key) => match key.code {
                //
                KeyCode::Esc => Some(Command::Exit),
                KeyCode::Char('c') => {
                    let body = format!("Cursor position: {:?}\r", position());
                    Some(Command::Debug(body))
                }
                _ => {
                    let body = format!("\rEvent::{:?}\r", key);
                    Some(Command::Debug(body))
                }
            },
            _ => None,
        },
        Err(_) => None,
    }
}
