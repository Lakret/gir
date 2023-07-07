use std::collections::HashMap;
use std::error::Error;

use egui::Rect;
use egui::{vec2, Color32, RichText, Sense, Slider, Stroke, TextEdit, Vec2};
use instant::Instant;

use crate::bfs::*;

const CELL_SIZE: f32 = 16.0;
const WALL_COLOR: Color32 = Color32::from_rgb(125, 0, 255);
const START_COLOR: Color32 = Color32::from_rgb(0, 0, 255);
const GOAL_COLOR: Color32 = Color32::from_rgb(0, 255, 0);
const PATH_COLOR: Color32 = Color32::from_rgb(255, 255, 0);
const EXPLORED_COLOR: Color32 = Color32::from_rgb(0, 50, 100);

#[derive(Debug, Clone, PartialEq)]
pub struct State {
  user_input: UserInput,
  validated: Validated,
  path: Vec<Pos>,
  explored: HashMap<u32, Vec<Pos>>,
  time: Instant,
  time_tick: u32,
  animate_bfs: bool,
  show_path: bool,
}

impl Default for State {
  fn default() -> Self {
    let user_input = UserInput {
      fav_number: "1350".to_string(),
      levels: 50,
      start_x: "1".to_string(),
      start_y: "1".to_string(),
      goal_x: "31".to_string(),
      goal_y: "39".to_string(),
    };
    let validated = user_input.validate().unwrap();
    let (path, explored) =
      path_and_explored_by_generation_for_animation(validated.fav_number, validated.start, validated.goal);

    Self {
      user_input,
      validated,
      path,
      explored,
      time: Instant::now(),
      time_tick: 0,
      animate_bfs: true,
      show_path: true,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserInput {
  fav_number: String,
  levels: u32,
  start_x: String,
  start_y: String,
  goal_x: String,
  goal_y: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Validated {
  fav_number: u32,
  levels: u32,
  start: Pos,
  goal: Pos,
}

impl UserInput {
  fn validate(&self) -> Result<Validated, Box<dyn Error>> {
    let fav_number = self
      .fav_number
      .parse::<u32>()
      .map_err(|err| format!("Incorrect value for Favorite Number: {}", err.to_string()))?;
    let start = Pos {
      x: self.start_x.parse::<u32>().map_err(|err| err.to_string())?,
      y: self.start_y.parse::<u32>().map_err(|err| err.to_string())?,
    };
    let goal = Pos {
      x: self.goal_x.parse::<u32>().map_err(|err| err.to_string())?,
      y: self.goal_y.parse::<u32>().map_err(|err| err.to_string())?,
    };

    Ok(Validated {
      fav_number,
      levels: self.levels,
      start,
      goal,
    })
  }
}

impl State {
  pub fn ui(&mut self, ui: &mut egui::Ui) {
    ui.collapsing(
      RichText::new("Breadth-First Search Graph Algorithm Demo").heading(),
      |ui| {
        ui.label("This demo allows you to try out an example Advent of Code problem for BFS.");
        ui.horizontal(|ui| {
          ui.label("You can find the source code for this example ");
          ui.hyperlink_to("here", "https://github.com/lakret/gir");
          ui.label(".");
        });
      },
    );

    ui.hyperlink_to("Tutorial Video", "https://youtu.be/ZDy3tqn-DKA");
    ui.hyperlink_to("Advent of Code 2016, Day 13", "https://adventofcode.com/2016/day/13");
    ui.add_space(15.0);

    ui.horizontal(|ui| {
      ui.label("Favorite Number: ");
      ui.add(TextEdit::singleline(&mut self.user_input.fav_number).margin(vec2(10.0, 6.0)));

      ui.add(
        Slider::new(&mut self.user_input.levels, 1..=200)
          .text("Levels to Draw")
          .integer(),
      );
    });
    ui.add_space(15.0);

    match self.user_input.validate() {
      Ok(new_validated) => {
        let (path, explored) = path_and_explored_by_generation_for_animation(
          new_validated.fav_number,
          new_validated.start,
          new_validated.goal,
        );
        self.path = path;
        self.explored = explored;
        self.validated = new_validated;
      }
      Err(msg) => {
        ui.label(RichText::new(msg.to_string()).color(Color32::DARK_RED));
      }
    }

    ui.horizontal(|ui| {
      let animate_bfs_checkbox = ui.checkbox(&mut self.animate_bfs, "Play Animation");
      if animate_bfs_checkbox.changed() && self.animate_bfs {
        self.time = Instant::now();
      }

      ui.add(Slider::new(&mut self.time_tick, 0..=100).text("Tick").integer());

      ui.checkbox(&mut self.show_path, "Show Path");
    });
    ui.add_space(15.0);

    egui::ScrollArea::both().show(ui, |ui| self.draw_animated_grid(ui));
  }

  fn draw_animated_grid(&mut self, ui: &mut egui::Ui) {
    if self.animate_bfs {
      // each animation tick is 1/20 of a second = 50 milliseconds
      self.time_tick = (self.time.elapsed().as_secs_f32() * 20.0).ceil() as u32 % 100;
    }

    let (response, painter) =
      ui.allocate_painter(Vec2::splat(self.validated.levels as f32 * CELL_SIZE), Sense::hover());

    let rect = response.rect;
    painter.rect_stroke(rect, 0.0, Stroke::new(1.0, WALL_COLOR));

    let min_x = *rect.x_range().start();
    let min_y = *rect.y_range().start();
    for y in 0..self.validated.levels {
      for x in 0..self.validated.levels {
        let pos = Pos { x, y };
        let cell = logical_pos_to_screen_rect(pos, min_x, min_y);

        if !pos.is_open(self.validated.fav_number) {
          painter.rect_filled(cell, 0.0, WALL_COLOR);
        }

        if self.validated.start == pos {
          painter.circle_filled(cell.center(), CELL_SIZE / 2.0, START_COLOR);
        }

        if self.validated.goal == pos {
          painter.rect_filled(cell, 4.0, GOAL_COLOR);
        }
      }
    }

    if self.show_path {
      // can be used for both path length and generation to show
      let elements_to_show = (self.path.len() as f32 / 100.0 * self.time_tick as f32).ceil() as usize;
      for &pos in &self.path[..elements_to_show] {
        if pos != self.validated.start && pos != self.validated.goal {
          let cell = logical_pos_to_screen_rect(pos, min_x, min_y);
          painter.rect_filled(cell, 6.0, PATH_COLOR);
        }
      }

      for generation in 0..(elements_to_show as u32) {
        if let Some(positions) = self.explored.get(&generation) {
          for &pos in positions {
            let cell = logical_pos_to_screen_rect(pos, min_x, min_y);
            painter.rect_filled(cell, 0.0, EXPLORED_COLOR);
          }
        }
      }
    }
  }
}

fn logical_pos_to_screen_rect(pos: Pos, min_x: f32, min_y: f32) -> Rect {
  let Pos { x, y } = pos;
  let screen_min_x = x as f32 * CELL_SIZE + min_x;
  let screen_max_x = (x + 1) as f32 * CELL_SIZE + min_x;
  let screen_min_y = (y as f32 * CELL_SIZE + min_y).ceil();
  let screen_max_y = ((y + 1) as f32 * CELL_SIZE + min_y).ceil();
  Rect::from_x_y_ranges(screen_min_x..=screen_max_x, screen_min_y..=screen_max_y)
}
