use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use egui::{CentralPanel, Pos2, RichText};

#[derive(Debug)]
pub struct StashApp {
    pub is_first_run: bool,

    pub is_about_open: Arc<AtomicBool>,
}

impl Default for StashApp {
    fn default() -> Self {
        Self {
            is_first_run: true,
            is_about_open: Arc::new(AtomicBool::new(false)),
        }
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

            let mut is_about_open = self.is_about_open.load(Ordering::Relaxed);
            ui.checkbox(&mut is_about_open, "Show about");
            self.is_about_open.store(is_about_open, Ordering::Relaxed);
        });

        // * About
        if self.is_about_open.load(Ordering::Relaxed) {
            let is_about_open = self.is_about_open.clone();

            let min_size = [320.0, 240.0];
            let centered_pos = Pos2::new(250., 250.);

            ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("about_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("About")
                    .with_position(Pos2::new(centered_pos.x, centered_pos.y))
                    .with_inner_size(min_size)
                    .with_min_inner_size(min_size),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Stash");
                        ui.label("Version: 0.1.0");
                    });
                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent to close us.
                        is_about_open.store(false, Ordering::Relaxed);
                    }
                },
            );
        }
    }
}
