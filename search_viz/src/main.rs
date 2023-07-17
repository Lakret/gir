use egui::{vec2, Color32, Frame, Margin, RichText};
use instant::Duration;

mod bfs;
mod bfs_ui;
mod minimax;
mod minimax_ui;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tabs {
  BFS,
  Minimax,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateApp {
  tab: Tabs,
  bfs_state: bfs_ui::State,
  minimax_state: minimax_ui::State,
}

impl Default for TemplateApp {
  fn default() -> Self {
    TemplateApp {
      // TODO: return it to be the BFS default
      tab: Tabs::Minimax,
      bfs_state: bfs_ui::State::default(),
      minimax_state: minimax_ui::State::default(),
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
    let tab_bg_fill = if self.tab == Tabs::BFS {
      Color32::from_rgb(20, 0, 50)
    } else {
      Color32::from_rgb(0, 30, 30)
    };

    egui::CentralPanel::default()
      .frame(Frame::default().fill(tab_bg_fill).inner_margin(Margin::same(20.0)))
      .show(ctx, |ui| {
        ui.horizontal_top(|ui| {
          ui.selectable_value(
            &mut self.tab,
            Tabs::BFS,
            RichText::new("Breadth-First Search Example").size(26.0),
          );
          ui.selectable_value(
            &mut self.tab,
            Tabs::Minimax,
            RichText::new("Minimax Example").size(26.0),
          );
        });

        if self.tab == Tabs::BFS {
          self.bfs_state.ui(ui);
        } else {
          self.minimax_state.ui(ui);
        }
      });

    // critical to make sure the animation is running
    ctx.request_repaint_after(Duration::from_millis(50));
  }
}
