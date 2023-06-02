use crossterm::event;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use rand::Rng;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::style::Color;
use ratatui::widgets::canvas::{Canvas, Line, Points, Rectangle};
use ratatui::widgets::{Block, Borders};
use ratatui::{Frame, Terminal};
use std::{array, io};

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

    let title = format!("{} ({}x{})", self.title, size.width, size.height);

    let block = Block::default().title(title.as_str()).borders(Borders::ALL);
    let canvas = Canvas::default()
      .block(block)
      .background_color(Color::Gray)
      .x_bounds([0.0, 10.0])
      .y_bounds([0.0, 10.0])
      .paint(|ctx| {
        // ctx.draw(&Rectangle {
        //   x: 1.0,
        //   y: 1.0,
        //   width: rand::thread_rng().gen_range(1.0..8.0),
        //   height: 1.0,
        //   color: Color::Red,
        // });

        let mut filled_rect = [(0.0, 0.0); 6 * 5 * 1000];
        let mut idx = 0;
        let step = 1.0 / 1000.0;
        for x in 2..8 {
          for y in 2..7 {
            for part in 0..1000 {
              filled_rect[idx] = (x as f64 + step * part as f64, y as f64 + step * part as f64);
              idx += 1;
            }
          }
        }

        ctx.draw(&Points {
          // coords: &[(2.0, 4.0), (4.0, 2.0), (6.0, 6.0)],
          coords: &filled_rect,
          color: Color::Yellow,
        });

        ctx.draw(&Line {
          x1: 1.0,
          y1: 1.5,
          x2: 5.0,
          y2: 8.0,
          color: Color::Blue,
        });
      });
    f.render_widget(canvas, size);
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
            // quit
            KeyCode::Char('q') => return Ok::<(), io::Error>(()),
            // refresh
            KeyCode::Char('r') => {
              terminal.draw(|f| self.view(f))?;
            }
            _ => {
              // dbg!(key_event.code);
              continue;
            }
          },

          Event::Resize(_cols, _rows) => {
            terminal.draw(|f| self.view(f))?;
          }

          _ => {
            // dbg!(ev);
            continue;
          }
        }
      }
    }
  }
}
