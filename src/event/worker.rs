use std::fmt::Debug;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use crossterm::event::{Event, EventStream, KeyCode};
use futures::{select, FutureExt, StreamExt};
use futures_timer::Delay;

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

    pub fn clone_receiver(self) -> Receiver<Event> {
        self.receiver.clone()
    }
}
