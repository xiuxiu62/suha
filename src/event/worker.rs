use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

use crossbeam_channel::{Receiver, SendError, Sender, TryRecvError};
use crossterm::cursor;
use crossterm::event::{Event, EventStream, KeyCode};
use futures::{select, FutureExt, StreamExt};
use futures_timer::Delay;
use tokio::sync::Mutex;

use super::command::Movement;
use super::Command;

type EventChannel = (Sender<Event>, Receiver<Event>);
pub type CommandChannel = Arc<Mutex<(Sender<Command>, Receiver<Command>)>>;

pub type SendResult<T, E> = Result<T, SendError<E>>;

pub type TryReceiveResult<T> = Result<T, TryRecvError>;

#[derive(Debug, Clone)]
pub struct Worker {
    event_channel: EventChannel,
    pub command_channel: CommandChannel,
}

impl Worker {
    pub fn new() -> Self {
        let event_channel = crossbeam_channel::unbounded();
        let command_channel = Arc::new(Mutex::new(crossbeam_channel::unbounded()));

        let mut reader = EventStream::new();
        let event_sender = event_channel.0.clone();

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
                                event_sender.send(event).unwrap();

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

        Self {
            event_channel,
            command_channel,
        }
    }

    // Parse events sent from worker listener
    pub fn parse_event(&self) -> Option<Command> {
        // try to recieve an event, returning nothing on error
        match self.event_channel.1.try_recv() {
            Ok(event) => match event {
                Event::Key(key) => match key.code {
                    //
                    KeyCode::Esc => Some(Command::Exit),
                    KeyCode::Char(char) => match char {
                        'm' => Some(Command::Mark),
                        'y' => Some(Command::Copy),
                        'd' => Some(Command::Cut),
                        'p' => Some(Command::Paste),
                        'u' => Some(Command::Undo),

                        'h' => Some(Command::Move(Movement::Left)),
                        'j' => Some(Command::Move(Movement::Down)),
                        'k' => Some(Command::Move(Movement::Up)),
                        'l' => Some(Command::Move(Movement::Right)),

                        'c' => {
                            let body = format!("Cursor position: {:?}\r", cursor::position());
                            Some(Command::Debug(body))
                        }

                        _ => {
                            let body = format!("\rChar({})\r", char);
                            Some(Command::Debug(body))
                        }
                    },
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
    pub async fn handle_event(&self) -> SendResult<bool, Command> {
        if let Some(command) = self.parse_event() {
            match command {
                Command::Exit => return Ok(true),
                command => self.send_command(command).await?,
            }
        }
        Ok(false)
    }

    async fn send_command(&self, command: Command) -> SendResult<(), Command> {
        self.command_channel.lock().await.0.send(command)
    }

    pub async fn receive_command(&self) -> TryReceiveResult<Command> {
        self.command_channel.lock().await.1.try_recv()
    }
}
