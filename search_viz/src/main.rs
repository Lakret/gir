use std::collections::HashMap;
use std::error::Error;

use egui::style::{Selection, WidgetVisuals, Widgets};
use egui::{vec2, Color32, Frame, Margin, Rect, RichText, Sense, Slider, Stroke, TextEdit, Vec2};
use instant::{Duration, Instant};

mod bfs;
use bfs::*;

const CELL_SIZE: f32 = 20.0;
const WALL_COLOR: Color32 = Color32::from_rgb(125, 0, 255);
const START_COLOR: Color32 = Color32::from_rgb(0, 0, 255);
const GOAL_COLOR: Color32 = Color32::from_rgb(0, 255, 0);
const PATH_COLOR: Color32 = Color32::from_rgb(255, 255, 0);
const EXPLORED_COLOR: Color32 = Color32::from_rgb(0, 50, 100);

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
  // Log to stdout (if you run with `RUST_LOG=debug`).
  tracing_subscriber::fmt::init();

  let native_options = eframe::NativeOptions::default();
  eframe::run_native(
    "GIR: Graph Search Examples",
    native_options,
    Box::new(|cc| Box::new(TemplateApp::new(cc))),
  )
}

// when compiling to web using trunk.
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
    let fav_number = self.fav_number.parse::<u32>().map_err(|err| err.to_string())?;
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
  animate_bfs: bool,
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
      animate_bfs: true,
    }
  }
}

impl TemplateApp {
  /// Called once before the first frame.
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    let default_dark_visuals = egui::Visuals::dark();
    cc.egui_ctx.set_visuals(egui::Visuals {
      override_text_color: Some(Color32::from_rgb(255, 255, 255)),
      extreme_bg_color: Color32::from_rgb(50, 0, 125),
      widgets: Widgets {
        inactive: WidgetVisuals {
          bg_fill: Color32::from_rgb(60, 0, 120),
          ..default_dark_visuals.widgets.inactive
        },
        hovered: WidgetVisuals {
          bg_fill: Color32::from_rgb(255, 100, 175),
          ..default_dark_visuals.widgets.hovered
        },
        active: WidgetVisuals {
          bg_fill: Color32::from_rgb(255, 100, 175),
          ..default_dark_visuals.widgets.hovered
        },
        ..default_dark_visuals.widgets
      },
      selection: Selection {
        // slider "progress" color Color32::from_rgb(0, 92, 128) is default
        bg_fill: Color32::from_rgb(255, 100, 175),
        ..default_dark_visuals.selection
      },
      slider_trailing_fill: true,
      ..default_dark_visuals
    });

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
    // each animation tick is 1/20 of a second = 50 milliseconds
    let time_tick = (self.time.elapsed().as_secs_f32() * 20.0).ceil() as u32 % 100;

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

        ui.heading("Breadth-First Search Example");
        ui.hyperlink_to("Advent of Code 2016, Day 13", "https://adventofcode.com/2016/day/13");
        ui.add_space(15.0);

        let mut fav_number_input = None;
        ui.horizontal(|ui| {
          ui.label("Office Designer's Favourite Number: ");
          fav_number_input =
            Some(ui.add(TextEdit::singleline(&mut self.user_input.fav_number).margin(vec2(10.0, 6.0))));
        });

        let levels_to_draw_slider = ui.add(
          Slider::new(&mut self.user_input.levels, 1..=200)
            .text("Levels to Draw")
            .integer(),
        );

        if let Some(fav_number_input) = fav_number_input {
          if fav_number_input.changed() || levels_to_draw_slider.changed() {
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
                // TODO: style and better messages (with field names)
                ui.label(msg.to_string());
              }
            }
          }
        }

        ui.label(format!(
          "The office for the given number {} with {} levels:",
          self.validated.fav_number, self.validated.levels,
        ));

        let animate_bfs_checkbox = ui.checkbox(&mut self.animate_bfs, "Show Animation");
        if animate_bfs_checkbox.changed() {
          self.time = Instant::now();
        }

        egui::ScrollArea::both().show(ui, |ui| {
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

          if self.animate_bfs {
            // can be used for both path length and generation to show
            let elements_to_show = (self.path.len() as f32 / 100.0 * time_tick as f32).ceil() as usize;
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
        });
      });

    ctx.request_repaint_after(Duration::from_millis(50));
  }
}

fn logical_pos_to_screen_rect(pos: Pos, min_x: f32, min_y: f32) -> Rect {
  let Pos { x, y } = pos;
  let screen_min_x = x as f32 * CELL_SIZE + min_x;
  let screen_max_x = (x + 1) as f32 * CELL_SIZE + min_x;
  let screen_min_y = y as f32 * CELL_SIZE + min_y;
  let screen_max_y = (y + 1) as f32 * CELL_SIZE + min_y;
  Rect::from_x_y_ranges(screen_min_x..=screen_max_x, screen_min_y..=screen_max_y)
}
