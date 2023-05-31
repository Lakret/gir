use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
  backend::{Backend, CrosstermBackend},
  widgets::{Block, Borders},
  Frame, Terminal,
};
use std::{
  io,
  process::exit,
  thread,
  time::{Duration, Instant},
};

use graphs::Graph;

fn main() -> Result<(), io::Error> {
  // setup terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut app = App::default();
  let res = app.run(&mut terminal);

  // restore terminal
  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
  terminal.show_cursor()?;

  res
}

struct App {
  title: String,
}

impl Default for App {
  fn default() -> App {
    App {
      title: "AoC 2016, Day 13".to_string(),
    }
  }
}

impl App {
  fn view<B>(&self, f: &mut Frame<B>)
  where
    B: Backend,
  {
    let size = f.size();
    let block = Block::default().title(self.title.as_str()).borders(Borders::ALL);
    f.render_widget(block, size);
  }

  pub fn run<B>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()>
  where
    B: Backend,
  {
    terminal.draw(|f| self.view(f))?;

    loop {
      if let Ok(event) = event::read() {
        match event {
          Event::Key(key_event) => match key_event.code {
            KeyCode::Char('q') => return Ok::<(), io::Error>(()),
            _ => {
              // dbg!(key_event.code);
              continue;
            }
          },

          _ev => {
            // dbg!(ev);
            continue;
          }
        }
      }
    }
  }
}
