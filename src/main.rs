#![allow(dead_code)]

mod event;
mod fs;
mod option;
// mod ui;
mod terminal;

use std::{io, path::Path, process};

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::Paragraph;
use tui::{backend::TermionBackend, Terminal};

use crate::event::{Event, Events};
use crate::fs::Cache;
use crate::option::DisplayOptions;

fn main() {
    let time = std::time::Instant::now();
    match test_fs_module() {
        // match test_crossterm() {
        // match test_ui_module() {
        Err(e) => {
            eprintln!("{}", e);
        }
        Ok(()) => {}
    }
    println!("took {:.5} micros", time.elapsed().as_micros())
}

fn test_ui_module() -> Result<(), Box<dyn std::error::Error>> {
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
            let text = Paragraph::new(format!("{:?}", buf));
            f.render_widget(text, chunks[0]);
        })?;

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

fn test_fs_module() -> Result<(), Box<dyn std::error::Error>> {
    let mut session_cache = Cache::new();
    let options = DisplayOptions::new(false, true);
    let path = Path::new("/home/xiuxiu/development/suha/src");

    session_cache.populate_to_root(path, &options)?;
    println!("{}", session_cache.to_string().trim_end());

    // let dir = session_cache.inner.get(&path.to_path_buf()).unwrap();
    // println!("{:?}\n", path);
    // dir.inner.iter().for_each(|e| {
    //     if let Ok(preview) = e.preview(1000) {
    //         println!("\"{}\"\n{}\n", e.label, preview);
    //     }
    // });

    Ok(())
}
