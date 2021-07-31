#[derive(Debug, Clone)]
pub struct DisplayOptions {
    pub show_hidden: bool,
    pub show_icons: bool,
}

impl DisplayOptions {
    pub fn new(show_hidden: bool, show_icons: bool) -> Self {
        Self {
            show_hidden,
            show_icons,
        }
    }
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_icons: false,
        }
    }
}
