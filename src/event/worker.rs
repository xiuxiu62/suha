use std::fmt::Debug;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use crossterm::cursor;
use crossterm::event::{Event, EventStream, KeyCode};
use futures::{select, FutureExt, StreamExt};
use futures_timer::Delay;

use super::Command;

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
            // Start listening
            loop {
                let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
                let mut event = reader.next().fuse();

                select! {
                    _ = delay => {},
                    maybe_event = event => {
                        match maybe_event {
                            Some(Ok(event)) => {
                                // Send key over worker channel
                                sender_clone.send(event).unwrap();

                                // Stop listening if escape key is pressed
                                if event == Event::Key(KeyCode::Esc.into()) {
                                    break;
                                }
                            },
                            Some(Err(e)) => eprintln!("Error: {:?}\n", e),
                            None => break,
                        }
                    },
                }
            }
        });

        Self { sender, receiver }
    }

    // Parse events sent from worker listener
    pub fn parse_event(&self) -> Option<Command> {
        // try to recieve an event, returning nothing on error
        match self.receiver.try_recv() {
            Ok(event) => match event {
                Event::Key(key) => match key.code {
                    //
                    KeyCode::Esc => Some(Command::Exit),
                    KeyCode::Char('c') => {
                        let body = format!("Cursor position: {:?}\r", cursor::position());
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
    pub fn handle_event(&self) -> bool {
        if let Some(command) = self.parse_event() {
            match command {
                Command::Exit => return true,
                Command::Debug(s) => println!("{}", s),
                _ => {}
            }
        }

        false
    }

    pub fn clone_receiver(self) -> Receiver<Event> {
        self.receiver
    }
}
