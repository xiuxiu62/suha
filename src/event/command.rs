use std::fmt::Display;

#[derive(Debug)]
pub enum Command {
    Exit,           // Unimplemented
    Mark,           // Unimplemented
    Copy,           // Unimplemented
    Cut,            // Unimplemented
    Paste,          // Unimplemented
    Undo,           // Unimplemented
    Move(Movement), // Unimplemented
    Debug(String),  // Logs debug info
    Error(String),  // Logs error info
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Command::Exit => "Command(Exit)".to_string(),
            Command::Mark => "Command(Mark)".to_string(),
            Command::Copy => "Command(Copy)".to_string(),
            Command::Cut => "Command(Cut)".to_string(),
            Command::Paste => "Command(Paste)".to_string(),
            Command::Undo => "Command(Undo)".to_string(),
            Command::Move(direction) => format!("Command(Move({}))", direction),
            Command::Debug(message) => format!("Debug: {}", message),
            Command::Error(message) => format!("Error: {}", message),
        };
        write!(f, "\r{}\r", message)
    }
}

#[derive(Debug)]
pub enum Movement {
    Up,    // ↑
    Down,  // ↓
    Left,  // ←
    Right, // →
    In,    // Into file inspection mode
    Out,   // Out of file inspection mode
}

impl Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match *self {
            Movement::Up => "Up",
            Movement::Down => "Down",
            Movement::Left => "Left",
            Movement::Right => "Right",
            Movement::In => "In",
            Movement::Out => "Out",
        };
        write!(f, "{}", message)
    }
}

#[derive(Debug)]
pub enum DispatchError {
    Io(std::io::Error),
}

impl From<std::io::Error> for DispatchError {
    fn from(err: std::io::Error) -> Self {
        DispatchError::Io(err)
    }
}

pub type DispatchResult<T> = Result<T, DispatchError>;

pub trait Dispatcher<T: Copy + Sized> {
    fn dispatch<K: Sized>(key: K) -> fn() -> DispatchResult<T>;
}
