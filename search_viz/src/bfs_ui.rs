use egui::Rect;
use egui::{vec2, Color32, RichText, Sense, Slider, Stroke, TextEdit, Vec2};
use instant::Instant;

use crate::bfs::*;
use crate::TemplateApp;

const CELL_SIZE: f32 = 16.0;
const WALL_COLOR: Color32 = Color32::from_rgb(125, 0, 255);
const START_COLOR: Color32 = Color32::from_rgb(0, 0, 255);
const GOAL_COLOR: Color32 = Color32::from_rgb(0, 255, 0);
const PATH_COLOR: Color32 = Color32::from_rgb(255, 255, 0);
const EXPLORED_COLOR: Color32 = Color32::from_rgb(0, 50, 100);

pub fn ui(state: &mut TemplateApp, ui: &mut egui::Ui) {
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
    ui.add(TextEdit::singleline(&mut state.user_input.fav_number).margin(vec2(10.0, 6.0)));

    ui.add(
      Slider::new(&mut state.user_input.levels, 1..=200)
        .text("Levels to Draw")
        .integer(),
    );
  });
  ui.add_space(15.0);

  match state.user_input.validate() {
    Ok(new_validated) => {
      let (path, explored) = path_and_explored_by_generation_for_animation(
        new_validated.fav_number,
        new_validated.start,
        new_validated.goal,
      );
      state.path = path;
      state.explored = explored;
      state.validated = new_validated;
    }
    Err(msg) => {
      ui.label(RichText::new(msg.to_string()).color(Color32::DARK_RED));
    }
  }

  ui.horizontal(|ui| {
    let animate_bfs_checkbox = ui.checkbox(&mut state.animate_bfs, "Play Animation");
    if animate_bfs_checkbox.changed() && state.animate_bfs {
      state.time = Instant::now();
    }

    ui.add(Slider::new(&mut state.time_tick, 0..=100).text("Tick").integer());

    ui.checkbox(&mut state.show_path, "Show Path");
  });
  ui.add_space(15.0);

  egui::ScrollArea::both().show(ui, |ui| draw_animated_grid(state, ui));
}

fn draw_animated_grid(state: &mut TemplateApp, ui: &mut egui::Ui) {
  if state.animate_bfs {
    // each animation tick is 1/20 of a second = 50 milliseconds
    state.time_tick = (state.time.elapsed().as_secs_f32() * 20.0).ceil() as u32 % 100;
  }

  let (response, painter) = ui.allocate_painter(Vec2::splat(state.validated.levels as f32 * CELL_SIZE), Sense::hover());

  let rect = response.rect;
  painter.rect_stroke(rect, 0.0, Stroke::new(1.0, WALL_COLOR));

  let min_x = *rect.x_range().start();
  let min_y = *rect.y_range().start();
  for y in 0..state.validated.levels {
    for x in 0..state.validated.levels {
      let pos = Pos { x, y };
      let cell = logical_pos_to_screen_rect(pos, min_x, min_y);

      if !pos.is_open(state.validated.fav_number) {
        painter.rect_filled(cell, 0.0, WALL_COLOR);
      }

      if state.validated.start == pos {
        painter.circle_filled(cell.center(), CELL_SIZE / 2.0, START_COLOR);
      }

      if state.validated.goal == pos {
        painter.rect_filled(cell, 4.0, GOAL_COLOR);
      }
    }
  }

  if state.show_path {
    // can be used for both path length and generation to show
    let elements_to_show = (state.path.len() as f32 / 100.0 * state.time_tick as f32).ceil() as usize;
    for &pos in &state.path[..elements_to_show] {
      if pos != state.validated.start && pos != state.validated.goal {
        let cell = logical_pos_to_screen_rect(pos, min_x, min_y);
        painter.rect_filled(cell, 6.0, PATH_COLOR);
      }
    }

    for generation in 0..(elements_to_show as u32) {
      if let Some(positions) = state.explored.get(&generation) {
        for &pos in positions {
          let cell = logical_pos_to_screen_rect(pos, min_x, min_y);
          painter.rect_filled(cell, 0.0, EXPLORED_COLOR);
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
