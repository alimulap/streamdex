#![allow(dead_code, unused)]

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{StatefulWidget, Widget},
};

use super::search_bar::SearchBar;

#[derive(Default)]
pub struct MainWidget {
    _counter: i32,
}

impl StatefulWidget for MainWidget {
    type State = MainWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State)
    where
        Self: Sized,
    {
        let main_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Length(3), Constraint::Min(1)],
        )
        .split(area);
        // SearchBar::default().render(main_layout[0], buf);
        // MainList::default().render(main_layout[1], buf)
    }
}

pub struct MainWidgetState {}
