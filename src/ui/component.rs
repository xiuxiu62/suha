use tui::widgets::Widget;

pub trait Component<T> {
    fn draw(&self) -> T;
}
