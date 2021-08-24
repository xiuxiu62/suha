use std::fmt::Debug;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use crossterm::cursor::position;
use crossterm::event::{Event, EventStream, KeyCode};
use futures::{select, FutureExt, StreamExt};
use futures_timer::Delay;

pub enum Command {
    Exit,
    Move,
    Copy,
    Cut,
    Paste,
    Undo,
    Debug(String),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Worker {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
}

impl Worker {
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let mut reader = EventStream::new();
        let sender_clone = sender.clone();

        tokio::spawn(async move {
            let mut exit = false;

            // Start listening
            loop {
                let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
                let mut event = reader.next().fuse();

                select! {
                    _ = delay => {},
                    maybe_event = event => {
                        match maybe_event {
                            Some(Ok(event)) => {
                                sender_clone.send(event).unwrap();
                                if event == Event::Key(KeyCode::Esc.into()) {
                                    exit = true;
                                }
                            },
                            Some(Err(e)) => eprintln!("Error: {:?}\r", e),
                            None => break,
                        }
                    },
                }

                // Stop if exit flag is set
                if exit {
                    break;
                }
            }
        });

        Self { sender, receiver }
    }

    pub fn clone_receiver(self) -> Receiver<Event> {
        self.receiver.clone()
    }

    // pub async fn handle(self) -> Option<Command> {
    //     if let Ok(event) = self.receiver.clone().try_recv() {
    //         return match event {
    //             Event::Key(key) => match key.code {
    //                 KeyCode::Esc => Some(Command::Exit),
    //                 KeyCode::Char('c') => {
    //                     let body = format!("Cursor position: {:?}\r", position());
    //                     Some(Command::Debug(body))
    //                 }
    //                 _ => {
    //                     let body = format!("\rEvent::{:?}", key);
    //                     Some(Command::Debug(body))
    //                 }
    //             },
    //             _ => None,
    //         };
    //     };

    //     None
    // }
}

pub async fn handle_event(receiver: Receiver<Event>) -> Option<Command> {
    if let Ok(event) = receiver.try_recv() {
        return match event {
            Event::Key(key) => match key.code {
                KeyCode::Esc => Some(Command::Exit),
                KeyCode::Char('c') => {
                    let body = format!("Cursor position: {:?}\r", position());
                    Some(Command::Debug(body))
                }
                _ => {
                    let body = format!("\rEvent::{:?}", key);
                    Some(Command::Debug(body))
                }
            },
            _ => None,
        };
    };

    None
}
