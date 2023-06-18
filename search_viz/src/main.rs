use std::collections::HashMap;
use std::error::Error;

use egui::style::{Selection, WidgetVisuals, Widgets};
use egui::{vec2, Color32, Frame, Margin, Rect, RichText, Sense, Slider, Stroke, TextEdit, Vec2};
use instant::{Duration, Instant};

mod bfs;
use bfs::*;
mod dfs;
use dfs::*;

const CELL_SIZE: f32 = 16.0;
const WALL_COLOR: Color32 = Color32::from_rgb(125, 0, 255);
const START_COLOR: Color32 = Color32::from_rgb(0, 0, 255);
const GOAL_COLOR: Color32 = Color32::from_rgb(0, 255, 0);
const PATH_COLOR: Color32 = Color32::from_rgb(255, 255, 0);
const EXPLORED_COLOR: Color32 = Color32::from_rgb(0, 50, 100);

// when compiling natively
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
  // Log to stdout (if you run with `RUST_LOG=debug`).
  tracing_subscriber::fmt::init();

  let mut native_options = eframe::NativeOptions::default();
  native_options.initial_window_size = Some(vec2(1100.0, 1050.0));
  native_options.resizable = true;
  eframe::run_native(
    "GIR: Graph Search Examples",
    native_options,
    Box::new(|cc| Box::new(TemplateApp::new(cc))),
  )
}

// when compiling to wasm using trunk
#[cfg(target_arch = "wasm32")]
fn main() {
  // make sure panics are logged using `console.error`
  console_error_panic_hook::set_once();
  // redirect tracing to console.log and friends
  tracing_wasm::set_as_global_default();

  let web_options = eframe::WebOptions::default();
  wasm_bindgen_futures::spawn_local(async {
    // attaches canvas to the hardcoded id in the html
    eframe::start_web(
      "the_canvas_id",
      web_options,
      Box::new(|cc| Box::new(TemplateApp::new(cc))),
    )
    .await
    .expect("failed to start eframe");
  });
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

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateApp {
  user_input: UserInput,
  validated: Validated,
  path: Vec<Pos>,
  explored: HashMap<u32, Vec<Pos>>,
  time: Instant,
  time_tick: u32,
  animate_bfs: bool,
  show_path: bool,
}

impl Default for TemplateApp {
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

impl TemplateApp {
  /// Called once before the first frame.
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(Color32::WHITE);
    visuals.extreme_bg_color = Color32::from_rgb(50, 0, 125);
    visuals.slider_trailing_fill = true;

    visuals.widgets.inactive.bg_fill = Color32::from_rgb(60, 0, 120);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(255, 100, 175);
    visuals.widgets.active.bg_fill = Color32::from_rgb(255, 100, 175);

    // slider "progress" color Color32::from_rgb(0, 92, 128) is default
    visuals.selection.bg_fill = Color32::from_rgb(255, 100, 175);
    cc.egui_ctx.set_visuals(visuals);

    let mut style = (*cc.egui_ctx.style()).clone();
    for (_text_style, font_id) in style.text_styles.iter_mut() {
      font_id.size *= 1.5;
    }
    style.spacing.slider_width = 400.0;
    cc.egui_ctx.set_style(style.clone());

    Default::default()
  }
}

impl eframe::App for TemplateApp {
  /// Called each time the UI needs repainting, which may be many times per second.
  /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default()
      .frame(
        Frame::default()
          .fill(Color32::from_rgb(20, 0, 50))
          .inner_margin(Margin::same(20.0)),
      )
      .show(ctx, |ui| {
        ui.collapsing(
          RichText::new("Breadth-First and Depth-First Graph Search Algorithms Demo").heading(),
          |ui| {
            ui.label("This demo allows you to try out 2 example Advent of Code problems for bfs and dfs respectively.");
            ui.horizontal(|ui| {
              ui.label("You can find the source code for this example ");
              ui.hyperlink_to("here", "https://github.com/lakret/gir");
              ui.label(".");
            });
          },
        );

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
      });

    // critical to make sure the animation is running
    ctx.request_repaint_after(Duration::from_millis(50));
  }
}

impl TemplateApp {
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
        let pos = bfs::Pos { x, y };
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
