use egui::{vec2, Button, Grid, RichText};

use crate::dfs::{Game, Mark};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct State {
  game: Game,
}

impl State {
  pub fn ui(&mut self, ui: &mut egui::Ui) {
    ui.collapsing(
      RichText::new("Depth-First Search Graph Algorithm Demo").heading(),
      |ui| {
        ui.label("This demo allows you to play tic-tac-toe against the computer.");
        ui.label("Computer will use depth-first search to select its moves.");
        ui.horizontal(|ui| {
          ui.label("You can find the source code for this example ");
          ui.hyperlink_to("here", "https://github.com/lakret/gir");
          ui.label(".");
        });
      },
    );

    Grid::new("game_grid")
      .num_columns(3)
      .spacing([8.0, 8.0])
      .show(ui, |ui| {
        for row in 0..3 {
          for col in 0..3 {
            let mark = self.game.mark_at(row, col);
            let label = mark.map_or("", mark_image);

            let button = Button::new(label)
              .fill(match mark {
                Some(Mark::Cross) => egui::Color32::from_rgb(150, 0, 150),
                Some(Mark::Circle) => egui::Color32::from_rgb(0, 100, 200),
                None => egui::Color32::from_gray(200),
              })
              .min_size(vec2(64.0, 64.0));

            let button = ui.add(button);
            if button.clicked() && !self.game.is_occupied(row, col) && !self.game.is_won() {
              self.game.do_move_row_col(row, col);
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
  }
}

fn mark_image(mark: Mark) -> &'static str {
  match mark {
    Mark::Cross => "X",
    Mark::Circle => "O",
  }
}
