use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Text,
    widgets::{
        block::Title,
        Block, Borders, List, ListState, Padding, StatefulWidget, Widget,
    },
};

use super::colors;

pub struct MainList<'a, T, F> {
    content: &'a Vec<T>,
    fn_display: F,
}

impl<'a, T, F> MainList<'a, T, F> {
    pub fn new(content: &'a Vec<T>, fn_display: F) -> Self {
        Self {
            content,
            fn_display,
        }
    }
}

impl<'a, T, F> Widget for MainList<'a, T, F>
where
    F: FnMut((usize, &'a T)) -> Text<'_>,
{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let top_title = Title::from("List");
        // let bottom_title = Title::from(" Press ctrl + 'q' to quit ")
        //     .alignment(Alignment::Center)
        //     .position(block::Position::Bottom);
        let block = Block::new()
            .title(top_title)
            // .title(bottom_title)
            .borders(Borders::all())
            .padding(Padding::horizontal(1))
            .fg(colors::PRIMARY);
        let list = self
            .content
            .iter()
            .enumerate()
            .map(self.fn_display)
            .collect::<Vec<_>>();
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        StatefulWidget::render(List::new(list).block(block), area, buf, &mut list_state);
    }
}

// pub struct MainListState<'a> {
//     pub links: Vec<&'a Link>,
// }
