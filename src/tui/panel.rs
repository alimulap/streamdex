use std::fmt::Display;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, List, ListState, Padding, StatefulWidget},
};

use crate::tui::{
    ColorType,
    utils::{border_color, line_style, text_color},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelId {
    TargetList,
    WaitListPriority,
    WaitList,
}

pub struct PanelList<Items> {
    pub id: PanelId,
    pub title: String,
    pub items: Items,
    pub key: char,
    pub state: ListState,
    pub area: Rect,
    pub focused: bool,
}

pub trait Panel {
    fn get_panel_id(&self) -> PanelId;
    fn set_focus(&mut self);
    fn set_unfocus(&mut self);
    fn handle_key_event(&mut self, event: KeyEvent) -> Option<PanelId>;
    fn handle_click(&mut self, x: u16, y: u16) -> Option<PanelId>;
}

impl<Items> Panel for PanelList<Items>
where
    for<'t> &'t Items: IntoIterator,
{
    fn get_panel_id(&self) -> PanelId {
        self.id
    }

    fn set_focus(&mut self) {
        self.focused = true;
    }

    fn set_unfocus(&mut self) {
        self.focused = false;
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> Option<PanelId> {
        if event.is_press() {
            if event.code == KeyCode::Char(self.key) {
                self.focused = true;
                return Some(self.id);
            }
        }
        return None;
    }

    fn handle_click(&mut self, x: u16, y: u16) -> Option<PanelId> {
        if self.area.contains(Position::new(x, y)) {
            let area_inside = Rect::new(
                self.area.x + 2,
                self.area.y + 1,
                self.area.width.saturating_sub(4),
                self.area.height.saturating_sub(2),
            );

            if area_inside.contains(Position::new(x, y)) {
                let item_index = (y - area_inside.y) as usize;
                self.state
                    .select(Some(item_index.min(self.items.into_iter().count())));
            }

            self.focused = true;
            return Some(self.id);
        } else {
            return None;
        }
    }
}

impl<Items> PanelList<Items>
where
    for<'t> &'t Items: IntoIterator,
{
    pub fn draw<'a, F>(&mut self, list_fn: F, area: Rect, buf: &mut Buffer)
    where
        F: FnMut(<&Items as IntoIterator>::Item) -> Vec<(String, ColorType)>,
    {
        let selected = self.state.selected();
        let items = self
            .items
            .into_iter()
            .map(list_fn)
            .enumerate()
            .map(|(i, texts)| {
                let selected = selected == Some(i);
                Line::from(
                    texts
                        .iter()
                        .map(|(text, ct)| text.clone().fg(text_color(self.focused, ct, selected)))
                        .collect::<Vec<Span>>(),
                )
                .style(line_style(self.focused, selected))
            })
            .collect::<Vec<Line<'a>>>();

        let list = List::new(items).block(
            Block::bordered()
                .title(format!(" [{}] {} ", self.key, self.title))
                .padding(Padding::horizontal(1))
                .fg(border_color(self.focused)),
        );

        StatefulWidget::render(list, area, buf, &mut self.state);
        self.area = area;
    }
}

impl Display for PanelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PanelId::TargetList => write!(f, "TargetList"),
            PanelId::WaitList => write!(f, "WaitList"),
            PanelId::WaitListPriority => write!(f, "WaitListPriority"),
        }
    }
}
