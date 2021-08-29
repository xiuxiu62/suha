#[derive(Debug)]
pub enum Command {
    Exit,          // Unimplemented
    Move,          // Unimplemented
    Copy,          // Unimplemented
    Cut,           // Unimplemented
    Paste,         // Unimplemented
    Undo,          // Unimplemented
    Debug(String), // Logs debug info
    Error(String), // Logs error info
}

#[derive(Debug)]
enum Movement {
    Up,    // ↑
    Down,  // ↓
    Left,  // ←
    Right, // →
    In,    // Into file inspection mode
    Out,   // Out of file inspection mode
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
