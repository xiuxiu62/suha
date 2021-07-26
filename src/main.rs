#![allow(unused_imports)]

use std::io::{self, Stdout, Write};
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant};
use std::{process, thread};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders};
use tui::{Frame, Terminal};

pub mod event;

// type TermFrame<'f> = Frame<'f, TermionBackend<RawTerminal<Stdout>>>;

fn main() -> Result<(), io::Error> {
    if let Err(e) = build() {
        eprintln!("{}", e);
        process::exit(1);
    }
    Ok(())
}

fn build() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    // loop {
    //     terminal.draw(|f| {
    //         let chunks = Layout::default()
    //             .direction(Direction::Vertical)
    //             .margin(0)
    //             .constraints(
    //                 [
    //                     Constraint::Percentage(10),
    //                     Constraint::Percentage(80),
    //                     Constraint::Percentage(10),
    //                 ]
    //                 .as_ref(),
    //             )
    //             .split(f.size());
    //         render_block(f, "Block 1", chunks[0]);
    //         render_block(f, "Block 2", chunks[1]);
    //         render_block(f, "Block 3", chunks[2]);
    //     })?;
    //     thread::sleep(TICK_RATE);
    // }

    // fn render_block(frame: &mut TermFrame, title: &str, chunk: Rect) {
    //     let block = Block::default()
    //         .title(title)
    //         .borders(Borders::ALL)
    //         .border_style(Style::default().fg(Color::Cyan));
    //     frame.render_widget(block, chunk);
    // }
    Ok(())
}
