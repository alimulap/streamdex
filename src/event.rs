use std::{fmt::Display, time::Duration};

use color_eyre::eyre::{OptionExt, Result};
use crossterm::event::KeyModifiers;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

pub enum Event {
    Tick,
    Crossterm(crossterm::event::Event),
    Tui(TuiEvent),
}

pub enum TuiEvent {
    Move(Direction),
    MovePanel(Direction),
    DoAction(KeyModifiers),
    Exit,
}

pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Right => write!(f, "Right"),
            Direction::Left => write!(f, "Left"),
            Direction::Down => write!(f, "Down"),
        }
    }
}

pub struct EventHandler {
    sender: UnboundedSender<Event>,
    receiver: UnboundedReceiver<Event>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = EventTask::new(sender.clone());
        tokio::spawn(async { actor.run().await });
        Self { sender, receiver }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or_eyre("Failed to receive event")
    }

    pub fn send(&self, event: TuiEvent) {
        let _ = self.sender.send(Event::Tui(event));
    }
}

pub struct EventTask {
    sender: UnboundedSender<Event>,
}

impl EventTask {
    pub fn new(sender: UnboundedSender<Event>) -> Self {
        Self { sender }
    }

    pub async fn run(self) -> Result<()> {
        let tick_rate = Duration::from_secs_f64(1. / 30.);
        let mut reader = crossterm::event::EventStream::new();
        let mut tick = tokio::time::interval(tick_rate);
        loop {
            let tick_delay = tick.tick();
            let crossterm_event = reader.next().fuse();
            tokio::select! {
                _ = self.sender.closed() => {
                    break;
                }
                _ = tick_delay => {
                    self.send(Event::Tick);
                }
                Some(Ok(event)) = crossterm_event => {
                    self.send(Event::Crossterm(event));
                }
            }
        }
        Ok(())
    }

    pub fn send(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}
