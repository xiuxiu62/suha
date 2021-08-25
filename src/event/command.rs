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
