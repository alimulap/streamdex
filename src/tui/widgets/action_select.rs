use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

pub struct ActionSelect {}

impl Widget for ActionSelect {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
    }
}
