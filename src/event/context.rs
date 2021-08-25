pub struct Context {
    pub mode: Mode,
    leader_key: &'static str,
}

pub enum Mode {
    Normal,  // standard keybindings
    Macro,   // redirect key-events for macro (C-x) sub-commands
    Command, // redirect key-events to a string buf, for command interpretation
}
