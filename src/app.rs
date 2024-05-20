use egui::{CentralPanel, RichText};

#[derive(Debug)]
pub struct StashApp {
    pub is_first_run: bool,
}

impl Default for StashApp {
    fn default() -> Self {
        Self { is_first_run: true }
    }
}

impl eframe::App for StashApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_first_run {
            self.is_first_run = false;
        }

        // * Main UI
        CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new("Stash").size(32.0));
        });
    }
}
