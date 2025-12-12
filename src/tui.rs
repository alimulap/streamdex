use std::hash::Hash;

use color_eyre::eyre::{OptionExt, Result};
use crossterm::event::{
    Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind,
};
use indexmap::{IndexMap, IndexSet};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize, palette::material::WHITE},
    text::Line,
    widgets::{Block, ListState, Widget},
};
use url::Url;

use crate::{
    context::Context,
    event::{self, Event, EventHandler, TuiEvent},
    target::TuiTarget,
    tui::{
        panel::{Panel, PanelId, PanelList},
        utils::inside_area,
    },
};

mod panel;
mod utils;

pub struct Tui {
    target_list: PanelList<IndexMap<String, TuiTarget>>,
    wait_list: PanelList<IndexSet<WaitItem>>,
    wait_list_priority: PanelList<IndexSet<WaitItem>>,
    events: EventHandler,
    focused_panel: PanelId,
    exit: bool,
    _debug_log: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WaitItem {
    name: String,
    url: Url,
    display_link: String,
}

impl PartialEq for WaitItem {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.url == other.url
    }
}

impl Eq for WaitItem {}

impl Hash for WaitItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.url.hash(state);
    }
}

impl Tui {
    pub fn new(ctx: &Context) -> Result<Self> {
        let focused_panel = PanelId::TargetList;
        let targets = TuiTarget::get_all(ctx)?;
        let mut target_list_state = ListState::default();
        if targets.len() > 0 {
            target_list_state.select_first();
        }
        let target_list = PanelList {
            id: PanelId::TargetList,
            title: String::from("Target List"),
            items: targets,
            key: '1',
            state: target_list_state,
            area: Rect::default(),
            focused: focused_panel == PanelId::TargetList,
        };
        let wait_list = PanelList {
            id: PanelId::WaitList,
            title: String::from("Wait List"),
            items: IndexSet::new(),
            key: '2',
            state: ListState::default(),
            area: Rect::default(),
            focused: focused_panel == PanelId::WaitList,
        };
        let wait_list_priority = PanelList {
            id: PanelId::WaitListPriority,
            title: String::from("Wait List Priority"),
            items: IndexSet::new(),
            key: '3',
            state: ListState::default(),
            area: Rect::default(),
            focused: focused_panel == PanelId::WaitListPriority,
        };
        let events = EventHandler::new();
        Ok(Self {
            target_list,
            wait_list,
            wait_list_priority,
            events,
            focused_panel,
            exit: false,
            _debug_log: Vec::new(),
        })
    }

    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            match self.events.next().await? {
                Event::Tick => (),
                Event::Crossterm(event) => match event {
                    CrosstermEvent::Key(event) => self.handle_key_event(event)?,
                    CrosstermEvent::Mouse(event) => self.handle_mouse_event(event)?,
                    _ => (),
                },
                Event::Tui(event) => match event {
                    TuiEvent::Move(direction) => self.move_item(direction),
                    TuiEvent::MovePanel(direction) => self.move_panel(direction),
                    TuiEvent::DoAction(modifiers) => self.do_action(modifiers)?,
                    TuiEvent::Exit => self.exit = true,
                },
            }
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) -> Result<()> {
        if event.is_press() {
            match (event.modifiers, event.code) {
                (KeyModifiers::NONE, KeyCode::Char(char)) => match char {
                    'q' => self.events.send(TuiEvent::Exit),
                    'k' => self.events.send(TuiEvent::Move(event::Direction::Up)),
                    'l' => self.events.send(TuiEvent::Move(event::Direction::Right)),
                    'h' => self.events.send(TuiEvent::Move(event::Direction::Left)),
                    'j' => self.events.send(TuiEvent::Move(event::Direction::Down)),
                    ' ' => self.events.send(TuiEvent::DoAction(KeyModifiers::NONE)),
                    _ => (),
                },
                #[rustfmt::skip]
                (KeyModifiers::CONTROL, KeyCode::Char(char)) => match char {
                    'k' => self.events.send(TuiEvent::MovePanel(event::Direction::Up)),
                    'l' => self.events.send(TuiEvent::MovePanel(event::Direction::Right)),
                    'h' => self.events.send(TuiEvent::MovePanel(event::Direction::Left)),
                    'j' => self.events.send(TuiEvent::MovePanel(event::Direction::Down)),
                    ' ' => self.events.send(TuiEvent::DoAction(KeyModifiers::CONTROL)),
                    _ => (),
                },
                (_, KeyCode::Char(char)) => match char {
                    ' ' => self.events.send(TuiEvent::DoAction(event.modifiers)),
                    _ => (),
                },
                _ => (),
            }
        }

        let result = Self::PANEL_IDS
            .map(|id| self.get_panel(id).handle_key_event(event))
            .into_iter()
            .collect();
        self.handle_panel_change(result);

        // if self.target_list.handle_key_event(event) {
        //     self.focused_panel = PanelId::TargetList;
        // }
        // if self.wait_list.handle_key_event(event) {
        //     self.focused_panel = PanelId::WaitList;
        // }
        // if self.wait_list_priority.handle_key_event(event) {
        //     self.focused_panel = PanelId::WaitListPriority;
        // }

        Ok(())
    }

    fn handle_mouse_event(&mut self, event: MouseEvent) -> Result<()> {
        match event.kind {
            MouseEventKind::Down(button) if button == MouseButton::Left => {
                let x = event.column;
                let y = event.row;
                self.handle_panel_click(x, y);
            }
            _ => (),
        }
        Ok(())
    }

    fn handle_panel_click(&mut self, x: u16, y: u16) {
        let panel_click_result = Self::PANEL_IDS
            .map(|id| self.get_panel(id).handle_click(x, y))
            .into_iter()
            .collect();
        self.handle_panel_change(panel_click_result);
    }

    const PANEL_IDS: [PanelId; 3] = [
        PanelId::TargetList,
        PanelId::WaitList,
        PanelId::WaitListPriority,
    ];

    fn handle_panel_change(&mut self, panel_click_result: Vec<Option<PanelId>>) {
        if let Some(Some(next_focused_panel)) =
            panel_click_result.iter().find(|item| item.is_some())
        {
            self.focused_panel = *next_focused_panel;
            for panel_id in Self::PANEL_IDS {
                let panel = self.get_panel(panel_id);
                if &panel.get_panel_id() == next_focused_panel {
                    panel.set_focus();
                } else {
                    panel.set_unfocus();
                }
            }
        }
    }

    fn get_panel(&mut self, panel_id: PanelId) -> &mut dyn Panel {
        match panel_id {
            PanelId::TargetList => &mut self.target_list,
            PanelId::WaitList => &mut self.wait_list,
            PanelId::WaitListPriority => &mut self.wait_list_priority,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn do_action(&mut self, modifiers: KeyModifiers) -> Result<()> {
        match self.focused_panel {
            PanelId::TargetList => {
                if let Some(index) = self.target_list.state.selected() {
                    let (name, target) = self
                        .target_list
                        .items
                        .get_index(index)
                        .ok_or_eyre(format!("Target with index {index} not found"))?;

                    match target {
                        TuiTarget::Links(links) => {
                            if let Some(yt) = &links.youtube {
                                self.wait_list.items.insert(WaitItem {
                                    name: name.clone(),
                                    url: yt.url.clone(),
                                    display_link: String::from("Youtube"),
                                });
                            }
                            if let Some(twitch) = &links.twitch {
                                self.wait_list.items.insert(WaitItem {
                                    name: name.clone(),
                                    url: twitch.url.clone(),
                                    display_link: String::from("TWitch"),
                                });
                            }
                        }
                        TuiTarget::Url(url) => {
                            self.wait_list.items.insert(WaitItem {
                                name: name.clone(),
                                url: url.clone(),
                                display_link: url.to_string(),
                            });
                        }
                    }

                    if self.wait_list.items.len() > 0 {
                        self.wait_list.state.select_first();
                    }
                }
            }
            PanelId::WaitList => {
                if let Some(selected) = self.wait_list.state.selected() {
                    match modifiers {
                        KeyModifiers::CONTROL => {
                            self.wait_list_priority.items.insert(
                                self.wait_list
                                    .items
                                    .get_index(selected)
                                    .cloned()
                                    .ok_or_eyre("Failed to get selected wait_item")?,
                            );
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }

        Ok(())
    }

    pub fn move_item(&mut self, direction: event::Direction) {
        match self.focused_panel {
            PanelId::TargetList => match direction {
                event::Direction::Up => self.target_list.state.select_previous(),
                event::Direction::Down => self.target_list.state.select_next(),
                _ => (),
            },
            PanelId::WaitList => match direction {
                event::Direction::Up => self.wait_list.state.select_previous(),
                event::Direction::Down => self.wait_list.state.select_next(),
                _ => (),
            },
            PanelId::WaitListPriority => match direction {
                event::Direction::Up => self.wait_list_priority.state.select_previous(),
                event::Direction::Down => self.wait_list_priority.state.select_next(),
                _ => (),
            },
        }
    }

    pub fn move_panel(&mut self, direction: event::Direction) {
        let next_panel = match self.focused_panel {
            PanelId::TargetList => match direction {
                event::Direction::Down => PanelId::WaitListPriority,
                _ => self.focused_panel,
            },
            PanelId::WaitListPriority => match direction {
                event::Direction::Up => PanelId::TargetList,
                event::Direction::Left => PanelId::WaitList,
                _ => self.focused_panel,
            },
            PanelId::WaitList => match direction {
                event::Direction::Up => PanelId::TargetList,
                event::Direction::Right => PanelId::WaitListPriority,
                _ => self.focused_panel,
            },
        };
        // let next_panel = match direction {
        //     event::Direction::Up => match self.focused_panel {
        //         PanelId::WaitListPriority | PanelId::WaitList => PanelId::TargetList,
        //         _ => self.focused_panel,
        //     },
        //     event::Direction::Right => match self.focused_panel {
        //         PanelId::WaitList => PanelId::WaitListPriority,
        //         _ => self.focused_panel,
        //     },
        //     event::Direction::Left => match self.focused_panel {
        //         PanelId::WaitListPriority => PanelId::WaitList,
        //         _ => self.focused_panel,
        //     },
        //     event::Direction::Down => match self.focused_panel {
        //         PanelId::TargetList => PanelId::WaitListPriority,
        //         _ => self.focused_panel,
        //     },
        // };
        self.handle_panel_change(vec![Some(next_panel)]);
    }
}

#[derive(Debug, Clone)]
pub enum ColorType {
    Primary,
    Secondary,
}

impl Widget for &mut Tui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(3),
                Constraint::Fill(5),
                Constraint::Fill(3),
            ],
        )
        .split(area);

        let inside_area = inside_area(&layout[0]);

        Line::from(vec![
            "Targets(".into(),
            self.target_list.items.len().to_string().into(),
            ") | ".into(),
            "WaitList(".into(),
            self.wait_list.items.len().to_string().into(),
            ") | ".into(),
            "Priority(".into(),
            self.wait_list_priority.items.len().to_string().into(),
            ")".into(),
        ])
        .render(inside_area, buf);

        Block::bordered()
            .title(" [0] Info ")
            .fg(Color::White)
            .render(layout[0], buf);

        self.target_list.draw(
            |(name, target)| {
                Vec::from([
                    (name.clone(), ColorType::Primary),
                    (target.to_string(), ColorType::Secondary),
                ])
            },
            layout[1],
            buf,
        );

        let bottom_layout = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .split(layout[2]);

        self.wait_list_priority.draw(
            |item| {
                Vec::from([
                    (item.name.clone(), ColorType::Primary),
                    (format!(" {}", item.display_link), ColorType::Secondary),
                ])
            },
            bottom_layout[1],
            buf,
        );

        self.wait_list.draw(
            |item| {
                Vec::from([
                    (item.name.clone(), ColorType::Primary),
                    (format!(" {}", item.display_link), ColorType::Secondary),
                ])
            },
            bottom_layout[0],
            buf,
        );

        Block::bordered()
            .title(" [4] Running Streams ")
            .fg(WHITE)
            .render(bottom_layout[2], buf);
    }
}
