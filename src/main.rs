mod tournament;
mod ui; // Import the UI module // Import tournament logic

use eframe::egui;
// use egui::ViewportBuilder;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Tournament Bracket Generator",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    tournament_ui: ui::TournamentUI,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            tournament_ui: ui::TournamentUI::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.tournament_ui.render(ctx);
    }
}
