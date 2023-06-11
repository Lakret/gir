// use rand::Rng;
use std::f32::consts::TAU;

use egui::{vec2, Color32, FontId, Frame, Pos2, Rgba, Sense, Stroke, TextStyle, Vec2};
use graphs::Graph;

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

pub struct TemplateApp {
  label: String,
  value: f32,
}

impl Default for TemplateApp {
  fn default() -> Self {
    Self {
      label: "Hello World!".to_owned(),
      value: 2.7,
    }
  }
}

impl TemplateApp {
  /// Called once before the first frame.
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    cc.egui_ctx.set_visuals(egui::Visuals {
      override_text_color: Some(Color32::from_rgb(255, 255, 255)),
      ..egui::Visuals::dark()
    });

    let mut style = (*cc.egui_ctx.style()).clone();
    for (_text_style, font_id) in style.text_styles.iter_mut() {
      font_id.size *= 1.5;
    }
    cc.egui_ctx.set_style(style.clone());

    Default::default()
  }
}

impl eframe::App for TemplateApp {
  /// Called each time the UI needs repainting, which may be many times per second.
  /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let Self { label, value } = self;

    egui::CentralPanel::default()
      .frame(Frame::default().fill(Color32::from_rgb(20, 0, 50)))
      .show(ctx, |ui| {
        // let mut visuals = ui.visuals_mut();
        // visuals.override_text_color = Some(Color32::from_rgb(255, 255, 255));
        // visuals.window_fill = Color32::from_rgb(100, 0, 0);

        ui.heading("Breadth-First and Depth-First Graph Search Algorithms Demo");
        // ui.hyperlink("https://github.com/emilk/eframe_template");

        ui.horizontal(|ui| {
          ui.label("Write something: ");
          ui.text_edit_singleline(label);
        });

        ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
        if ui.button("Increment").clicked() {
          *value += 1.0;
        }

        // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //   ui.horizontal(|ui| {
        //     ui.spacing_mut().item_spacing.x = 0.0;
        //     ui.label("powered by ");
        //     ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        //     ui.label(" and ");
        //     ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/crates/eframe");
        //     ui.label(".");
        //   });
        // });
        //
        // ui.add(egui::github_link_file!(
        //   "https://github.com/emilk/eframe_template/blob/master/",
        //   "Source code yo."
        // ));

        let (response, painter) = ui.allocate_painter(Vec2::splat(128.0), Sense::hover());

        let rect = response.rect;
        let c = rect.center();
        let r = rect.width() / 2.0 - 5.0;
        let color = Color32::from_rgb(255, 255, 0);
        let stroke = Stroke::new(3.0, color);
        painter.circle_stroke(c, r, stroke);
        painter.line_segment(
          [c - vec2(0.0, r), c + vec2(0.0, r)],
          Stroke {
            color: Color32::from_rgb(0, 255, 255),
            ..stroke
          },
        );
        painter.line_segment(
          [c, c + r * Vec2::angled(TAU * 1.0 / 8.0)],
          Stroke {
            color: Color32::from_rgb(0, 128, 128),
            ..stroke
          },
        );
        painter.line_segment(
          [c, c + r * Vec2::angled(TAU * 3.0 / 8.0)],
          Stroke {
            color: Color32::from_rgb(0, 128, 128),
            ..stroke
          },
        );

        ui.heading(format!("c = {c:?}, r = {r:?}"));
      });
  }
}
