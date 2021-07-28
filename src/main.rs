use std::io;
use std::process;

use event::Event;
use event::Events;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use unicode_width::UnicodeWidthStr;

mod event;

fn main() -> Result<(), io::Error> {
    if let Err(e) = build() {
        eprintln!("{}", e);
        process::exit(1);
    }
    Ok(())
}

enum InputMode {
    Normal,
    Editing,
}

struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

fn build() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = AlternateScreen::from(MouseTerminal::from(io::stdout().into_raw_mode()?));
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new(None);
    let mut app = App::default();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let (msg, style) = match app.input_mode {
                InputMode::Normal => (
                    vec![
                        Span::raw("Press "),
                        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to exit, "),
                        Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to start editing."),
                    ],
                    Style::default().add_modifier(Modifier::RAPID_BLINK),
                ),
                InputMode::Editing => (
                    vec![
                        Span::raw("Press "),
                        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to stop editing, "),
                        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to record the message"),
                    ],
                    Style::default(),
                ),
            };

            let mut text = Text::from(Spans::from(msg));
            text.patch_style(style);
            let help_message = Paragraph::new(text);
            f.render_widget(help_message, chunks[0]);

            let input = Paragraph::new(app.input.as_ref())
                .style(match app.input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => Style::default().fg(Color::Yellow),
                })
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input, chunks[1]);
            match app.input_mode {
                InputMode::Normal =>
                    // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                    {}

                InputMode::Editing => {
                    // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                    f.set_cursor(
                        // Put cursor past the end of the input text
                        chunks[1].x + app.input.width() as u16 + 1,
                        // Move one line down, from the border to the input line
                        chunks[1].y + 1,
                    )
                }
            }

            let messages: Vec<ListItem> = app
                .messages
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
                    ListItem::new(content)
                })
                .collect();

            let messages =
                List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
            f.render_widget(messages, chunks[2]);
        })?;

        // Handle input
        if let Event::Input(input) = events.next()? {
            match app.input_mode {
                InputMode::Normal => match input {
                    Key::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    Key::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                InputMode::Editing => match input {
                    Key::Char('\n') => {
                        app.messages.push(app.input.drain(..).collect());
                    }
                    Key::Char(c) => {
                        app.input.push(c);
                    }
                    Key::Backspace => {
                        app.input.pop();
                    }
                    Key::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(())
}

// fn build() -> Result<(), Box<dyn std::error::Error>> {
//     let stdout = AlternateScreen::from(MouseTerminal::from(io::stdout().into_raw_mode()?));
//     let backend = TermionBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;
//     // let worker = EventHandler::new();

//     terminal.clear()?;

//     loop {
//         // the trouble is in this event reciever blocking
//         if let Ok(event) = worker.next() {
//             match event {
//                 RuntimeEvent::Termion(event) => {
//                     if let termion::event::Event::Key(termion::event::Key::Char(key)) = event {
//                         if key == 'q' {
//                             break;
//                         }
//                         println!("{:#?}", key);
//                     }
//                 }
//                 _ => {}
//             }
//         }

//         thread::sleep(Duration::from_millis(100));

//         terminal.draw(|f| {
//             let size = f.size();
//             let chunks = Layout::default().split(size);
//             let inner_text = Paragraph::new("working");
//             f.render_widget(inner_text, chunks[0]);
//         })?;
//     }

//     Ok(())
// }

// fn render_block(frame: &mut TermFrame, title: &str, chunk: Rect) {
//     let block = Block::default()
//         .title(title)
//         .borders(Borders::ALL)
//         .border_style(Style::default().fg(Color::Cyan));
//     frame.render_widget(block, chunk);
// }
