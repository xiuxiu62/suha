use std::io;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};
use std::{sync::mpsc, thread};

use termion::event::Event;
use termion::input::TermRead;

const TICK_RATE: Duration = Duration::from_millis(200);

#[derive(Debug, Clone)]
pub enum RuntimeEvent {
    Termion(Event),
    Signal(i32),
    Tick,
}

#[derive(Debug)]
pub struct EventHandler {
    pub event_tx: Sender<RuntimeEvent>,
    pub event_rx: Receiver<RuntimeEvent>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::channel::<RuntimeEvent>();
        Self { event_tx, event_rx }
    }

    pub fn spawn() {
        thread::spawn(move || {
            let worker = Self::new();
            let mut last_tick = Instant::now();

            loop {
                let timeout = TICK_RATE
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                let stdin = io::stdin();
                for c in stdin.keys() {
                    worker
                        .event_tx
                        .send(RuntimeEvent::Termion(Event::Key(c.unwrap())))
                        .expect("can send events");
                }

                if last_tick.elapsed() >= TICK_RATE {
                    if let Ok(_) = worker.event_tx.send(RuntimeEvent::Tick) {
                        last_tick = Instant::now();
                    }
                }
            }
        });
    }
}

// fn handle_key_event(rx: &Receiver<KeyEvent>) {
// 	match rx.recv()? {
// 		                    Key::Char('q') => tx.send(KeyEvent::Exit(0)).expect("can send events"),
//                     Key::Ctrl(c) => {
//                         if c == 'c' || c == 'C' {
//                             tx.send(KeyEvent::Exit(0)).expect("can send events")
//                         }
//                     }
//                     _ => {}
// 		KeyEvent::Exit(code) =>
// 	}
// }
