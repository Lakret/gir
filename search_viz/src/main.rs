use std::collections::HashMap;
use std::error::Error;

use egui::{vec2, Color32, Frame, Margin, RichText};
use instant::{Duration, Instant};

mod bfs;
use bfs::*;
mod bfs_ui;
mod dfs_ui;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tabs {
  BFS,
  DFS,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateApp {
  tab: Tabs,
  // BFS
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
      // TODO: return it to be the BFS default
      tab: Tabs::DFS,
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
        ui.horizontal_top(|ui| {
          ui.selectable_value(
            &mut self.tab,
            Tabs::BFS,
            RichText::new("Breadth-First Search Example").size(26.0),
          );
          ui.selectable_value(
            &mut self.tab,
            Tabs::DFS,
            RichText::new("Depth-First Search Example").size(26.0),
          );
        });

        if self.tab == Tabs::BFS {
          bfs_ui::ui(self, ui);
        } else {
          dfs_ui::ui(self, ui);
        }
      });

    // critical to make sure the animation is running
    ctx.request_repaint_after(Duration::from_millis(50));
  }
}
