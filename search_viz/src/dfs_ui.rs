use egui::RichText;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct State {}

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
  }
}
