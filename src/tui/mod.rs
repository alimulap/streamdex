use std::io::{self, stdout, Stdout};

mod app;
mod matcher;

use app::App;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::context::Context2;

mod widgets;

pub fn run(ctx: &Context2) {
    let mut terminal = init().unwrap();
    App::new(ctx).run(&mut terminal).unwrap();
    restore().unwrap();
}

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
