#![allow(dead_code)]

use std::{io, sync::Arc, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use nucleo::{
    pattern::{CaseMatching, Normalization},
    Nucleo,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Stylize},
    text::Text,
    widgets::Paragraph,
    Frame,
};

use crate::{
    context::Context2,
    links::{self, Link},
};

use super::{
    matcher::Matcher,
    widgets::{colors, main_list::MainList, search_bar::SearchBar},
    Tui,
};

pub struct App {
    links: LinksContainer,
    matcher: Matcher,
    // matcher: Nucleo<Link>,
    // matcher_config: nucleo::Config,
    // injector: Injector<Link>,
    search_query: String,
    exit: bool,
    debug_string: String,
}

#[derive(Clone)]
pub struct LinksContainer {
    primary: Vec<Link>,
    secondary: Vec<Link>,
    no_alias: Vec<Link>,
}

impl App {
    pub fn new(ctx: &Context2) -> Self {
        let matcher_config = nucleo::Config::DEFAULT;
        let matcher = Nucleo::new(matcher_config.clone(), Arc::new(|| {}), None, 1);
        let injector = matcher.injector();

        let links = LinksContainer {
            primary: links::get_primary(ctx),
            secondary: links::get_secondary(ctx),
            no_alias: links::get_no_alias(ctx),
        };

        for link in &links.primary {
            injector.push(link.clone(), |item, cols| {
                cols[0] = if let Some(alias) = &item.alias {
                    format!("{} {}", alias, item.url).into()
                } else {
                    item.url.clone().into()
                };
                // cols[0] = (item.alias.clone().unwrap_or("".to_owned())).into();
                // cols[1] = item.url.clone().into()
            });
        }

        for link in &links.secondary {
            injector.push(link.clone(), |item, cols| {
                cols[0] = if let Some(alias) = &item.alias {
                    format!("{} {}", alias, item.url).into()
                } else {
                    item.url.clone().into()
                };
                // cols[0] = (item.alias.clone().unwrap_or("".to_owned())).into();
                // cols[1] = item.url.clone().into()
            });
        }

        for link in &links.no_alias {
            injector.push(link.clone(), |item, cols| {
                cols[0] = if let Some(alias) = &item.alias {
                    format!("{} {}", alias, item.url).into()
                } else {
                    item.url.clone().into()
                };
                // cols[0] = (item.alias.clone().unwrap_or("".to_owned())).into();
                // cols[1] = item.url.clone().into()
            });
        }

        let matcher = Matcher {
            matcher,
            matcher_config,
            injector,
        };

        Self {
            links,
            matcher,
            // matcher_config,
            // injector,
            search_query: String::new(),
            exit: false,
            debug_string: String::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.exit {
            self.update();
            terminal.draw(|frame| self.draw(frame)).unwrap();
        }
        Ok(())
    }

    fn update(&mut self) {
        if event::poll(Duration::from_millis(100)).unwrap() {
            self.event_handler();
        }

        // self.matcher.tick(10);

        let _status = self.matcher.tick(50);
    }

    fn event_handler(&mut self) {
        match event::read().unwrap() {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.exit = true
                    }
                    KeyCode::Char(char) => {
                        self.search_query.push(char);
                        // self.matcher.pattern.reparse(0, &self.search_query, CaseMatching::Ignore, Normalization::Smart, false);
                        self.matcher.pattern.reparse(
                            0,
                            &self.search_query,
                            CaseMatching::Ignore,
                            Normalization::Smart,
                            false,
                        );
                    }
                    KeyCode::Backspace => {
                        self.search_query.pop();
                        // self.matcher.pattern.reparse(0, &self.search_query, CaseMatching::Ignore, Normalization::Smart, false);
                        self.matcher.pattern.reparse(
                            0,
                            &self.search_query,
                            CaseMatching::Ignore,
                            Normalization::Smart,
                            false,
                        );
                    }
                    // KeyCode::Right => self.counter += 1,
                    // KeyCode::Left => self.counter -= 1,
                    _ => (),
                }
            }
            _ => (),
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        // frame.render_stateful_widget(MainWidget::default(), frame.area(), &mut MainWidgetState {});

        let links_to_list = self
            .matcher
            .snapshot()
            .matched_items(..)
            .map(|item| item.data)
            .collect::<Vec<&Link>>();

        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ],
        )
        .split(frame.area());
        frame.render_widget(SearchBar::new(&self.search_query), main_layout[0]);
        frame.render_widget(
            MainList::new(&links_to_list, |(i, link): (usize, &&Link)| -> Text<'_> {
                if let Some(alias) = &link.alias {
                    Text::raw(format!("[{}] {}", alias, link.url)).bg(if i == 0 {
                        Color::Blue
                    } else {
                        Color::Reset
                    })
                } else {
                    Text::raw(link.url.clone()).bg(if i == 0 { Color::Blue } else { Color::Reset })
                }
            }),
            main_layout[1],
        );
        frame.render_widget(
            Paragraph::new(" Press ctrl + 'q' to quit ")
                .fg(colors::PRIMARY)
                .alignment(Alignment::Center),
            main_layout[2],
        );
    }
}
