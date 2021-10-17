use tui::widgets::Widget;

pub trait Component<W: Widget> {
    fn draw(&self) -> W;
}
