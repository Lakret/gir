use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
  backend::{Backend, CrosstermBackend},
  widgets::{Block, Borders},
  Terminal,
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

  let res = run_app(&mut terminal);

  // restore terminal
  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
  terminal.show_cursor()?;

  res
}

fn run_app<B>(terminal: &mut Terminal<B>) -> io::Result<()>
where
  B: Backend,
{
  terminal.draw(|f| {
    let size = f.size();
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, size);
  })?;

  // Start a thread to discard any input events. Without handling events, the
  // stdin buffer will fill up, and be read into the shell when the program exits.
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
