use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    widgets::{block::Title, Block, Borders, Padding, Paragraph, Widget},
};

use super::colors;

#[derive(Default)]
pub struct SearchBar<'a> {
    query: &'a str,
}

impl<'a> SearchBar<'a> {
    pub fn new(query: &'a str) -> Self {
        Self { query }
    }
}

impl<'a> Widget for SearchBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(self.query)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .fg(colors::PRIMARY)
                    .title(Title::from("Search".fg(colors::PRIMARY)))
                    .padding(Padding::horizontal(1)),
            )
            .render(area, buf);
        // .render(area, buf);
    }
}
