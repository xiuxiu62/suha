use crossbeam_channel::Receiver;
use crossterm::cursor::position;
use crossterm::event::{Event, KeyCode};

mod command;
mod context;
mod worker;

pub use command::Command;
pub use context::Context;
pub use worker::Worker;

// Parse events sent from worker listener
pub fn parse_event(receiver: &Receiver<Event>) -> Option<Command> {
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

// Handles an event, returing true if the program should exit
pub fn handle_event(receiver: &Receiver<Event>) -> bool {
    if let Some(command) = parse_event(receiver) {
        match command {
            Command::Exit => return true,
            Command::Debug(s) => println!("{}", s),
            _ => {}
        }
    }

    false
}
