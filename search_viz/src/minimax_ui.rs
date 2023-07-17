use egui::{vec2, Button, Color32, Grid, RichText};

use crate::minimax::{Game, Mark};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct State {
  game: Game,
  manual_mode: bool,
}

impl State {
  pub fn ui(&mut self, ui: &mut egui::Ui) {
    ui.collapsing(RichText::new("Minimax Algorithm Demo").heading(), |ui| {
      ui.label("This demo allows you to play tic-tac-toe against the computer.");
      ui.label("Computer will use minimax to select its moves.");
      ui.horizontal(|ui| {
        ui.label("You can find the source code for this example ");
        ui.hyperlink_to("here", "https://github.com/lakret/gir");
        ui.label(".");
      });
    });

    ui.horizontal(|ui| {
      ui.label("Play against:");
      ui.radio_value(&mut self.manual_mode, false, "Computer");
      ui.radio_value(&mut self.manual_mode, true, "Person");
    });
    ui.add_space(8.0);

    Grid::new("game_grid")
      .num_columns(3)
      .spacing([8.0, 8.0])
      .show(ui, |ui| {
        for row in 0..3 {
          for col in 0..3 {
            let mark = self.game.mark_at(row, col);
            let label = mark.map_or("", mark_image);

            let cell = Button::new(RichText::new(label).size(46.0))
              .fill(match mark {
                Some(Mark::Cross) => egui::Color32::from_rgb(150, 0, 150),
                Some(Mark::Circle) => egui::Color32::from_rgb(0, 100, 200),
                None => egui::Color32::from_gray(200),
              })
              .min_size(vec2(64.0, 64.0));
            let cell = ui.add(cell);

            if cell.clicked() && !self.game.is_occupied(row, col) && !self.game.is_won() {
              self.game.do_move_row_col(row, col);

              // do computer turn if computer mode is enabled
              if !self.manual_mode {
                if let Some(pos) = self.game.select_next_move() {
                  self.game.do_move(pos);
                }
              }
            }
          }

          ui.end_row();
        }
      });

    match self.game.winning_mark() {
      None => {
        ui.label(format!(
          "{:?} turn.",
          // TODO: maybe we should store Mark as the "next_turn" field in Game struct directly?
          if self.game.is_cross_turn() {
            Mark::Cross
          } else {
            Mark::Circle
          }
        ));
      }
      Some(winning_mark) => {
        ui.label(format!("{:?} won.", winning_mark));
      }
    }

    if ui
      .add(Button::new(RichText::new("New Game").size(26.0)).fill(Color32::DARK_GREEN))
      .clicked()
    {
      self.game = Game::default();
    }
  }
}

fn mark_image(mark: Mark) -> &'static str {
  match mark {
    Mark::Cross => "X",
    Mark::Circle => "O",
  }
}
