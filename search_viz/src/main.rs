// use rand::Rng;
use std::{error::Error, f32::consts::TAU};

use egui::{
  vec2, Color32, FontId, Frame, Margin, Pos2, Rect, Rgba, Sense, Separator, Stroke, TextEdit, TextStyle, Vec2,
};
use graphs::Graph;

mod bfs;

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
  levels: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Validated {
  fav_number: u32,
  levels: u32,
}

impl UserInput {
  fn validate(&self) -> Result<Validated, Box<dyn Error>> {
    let fav_number = self.fav_number.parse::<u32>().map_err(|err| err.to_string())?;
    let levels = self.levels.parse::<u32>().map_err(|err| err.to_string())?;
    Ok(Validated { fav_number, levels })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateApp {
  user_input: UserInput,
  validated: Validated,
}

impl Default for TemplateApp {
  fn default() -> Self {
    let user_input = UserInput {
      fav_number: "1350".to_string(),
      levels: "20".to_string(),
    };
    let validated = user_input.validate().unwrap();

    Self { user_input, validated }
  }
}

impl TemplateApp {
  /// Called once before the first frame.
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    cc.egui_ctx.set_visuals(egui::Visuals {
      override_text_color: Some(Color32::from_rgb(255, 255, 255)),
      extreme_bg_color: Color32::from_rgb(50, 0, 125),
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
    let TemplateApp {
      user_input: ref mut user_input,
      //  @ UserInput {
      //   fav_number: user_input_fav_number,
      // },
      validated: ref mut validated, // @ Validated {
                                    //   fav_number: validated_fav_number,
                                    // },
    } = self;

    egui::CentralPanel::default()
      .frame(
        Frame::default()
          .fill(Color32::from_rgb(20, 0, 50))
          .inner_margin(Margin::same(20.0)),
      )
      .show(ctx, |ui| {
        ui.heading("Breadth-First and Depth-First Graph Search Algorithms Demo");
        ui.label("This demo allows you to try out 2 example Advent of Code problems for bfs and dfs respectively.");
        ui.horizontal(|ui| {
          ui.label("You can find the source code for this example ");
          ui.hyperlink_to("here", "https://github.com/lakret/gir");
          ui.label(".");
        });
        ui.add(Separator::default().spacing(20.0));

        ui.heading("Breadth-First Search Example: Advent of Code 2016, Day 13");
        ui.hyperlink_to("The Task", "https://adventofcode.com/2016/day/13");
        ui.add_space(15.0);

        let mut fav_number_input = None;
        ui.horizontal(|ui| {
          ui.label("Office Designer's Favourite Number: ");
          fav_number_input = Some(ui.add(TextEdit::singleline(&mut user_input.fav_number).margin(vec2(10.0, 6.0))));
        });

        ui.horizontal(|ui| {
          ui.label("Levels to Draw: ");
          ui.add(TextEdit::singleline(&mut user_input.levels));
        });

        match user_input.validate() {
          Ok(new_validated) => {
            self.validated = new_validated;
            // TODO: redraw only on change
          }
          Err(msg) => {
            // TODO: style and better messages (with field names)
            ui.label(msg.to_string());
          }
        }

        ui.label(format!(
          "The office for the given number {} with {} levels:",
          self.validated.fav_number, self.validated.levels,
        ));

        let size = 20.0;
        let (response, painter) = ui.allocate_painter(Vec2::splat(self.validated.levels as f32 * size), Sense::hover());

        let rect = response.rect;
        let wall_color = Color32::from_rgb(125, 0, 255);
        let path_color = Color32::from_rgb(255, 255, 0);
        painter.rect_stroke(rect, 0.0, Stroke::new(1.0, wall_color));

        let min_x = *rect.x_range().start();
        let min_y = *rect.y_range().start();
        for y in 0..self.validated.levels {
          for x in 0..self.validated.levels {
            let pos = bfs::Pos { x, y };
            let cell = Rect::from_x_y_ranges(
              (x as f32 * size + min_x)..=((x + 1) as f32 * size + min_x),
              (y as f32 * size + min_y)..=((y + 1) as f32 * size + min_y),
            );
            if !pos.is_open(self.validated.fav_number) {
              painter.rect_filled(cell, 0.0, wall_color);
            }

            if x == 1 && y == 1 {
              painter.circle_filled(cell.center(), size / 2.0, path_color);
            }
          }
        }

        // let stroke = Stroke::new(3.0, color);
        // painter.circle_stroke(c, r, stroke);
        // painter.line_segment(
        //   [c - vec2(0.0, r), c + vec2(0.0, r)],
        //   Stroke {
        //     color: Color32::from_rgb(0, 255, 255),
        //     ..stroke
        //   },
        // );
        // painter.line_segment(
        //   [c, c + r * Vec2::angled(TAU * 1.0 / 8.0)],
        //   Stroke {
        //     color: Color32::from_rgb(0, 128, 128),
        //     ..stroke
        //   },
        // );
        // painter.line_segment(
        //   [c, c + r * Vec2::angled(TAU * 3.0 / 8.0)],
        //   Stroke {
        //     color: Color32::from_rgb(0, 128, 128),
        //     ..stroke
        //   },
        // );

        // ui.label(format!("rect = {rect:?}"));

        // ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
        // if ui.button("Increment").clicked() {
        //   *value += 1.0;
        // }

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
      });
  }
}
