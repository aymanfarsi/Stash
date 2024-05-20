use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use egui::{CentralPanel, Pos2, RichText, ViewportBuilder, ViewportClass, ViewportId, WindowLevel};

use crate::ui::about::AboutViewport;

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

            egui_extras::install_image_loaders(ctx);
        }

        // * Main UI
        CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new("Stash").size(32.0));

            let mut is_about_open = self.is_about_open.load(Ordering::Relaxed);
            ui.checkbox(&mut is_about_open, "Show about");
            self.is_about_open.store(is_about_open, Ordering::Relaxed);
        });

        // * About viewport
        if self.is_about_open.load(Ordering::Relaxed) {
            let is_about_open = self.is_about_open.clone();

            let min_size = [320.0, 240.0];
            // let outer_size = ctx.input(|i| i.viewport().outer_rect);
            // let about_pos2 = match outer_size {
            //     Some(outer_size) => {
            //         let outer_center = outer_size.center();
            //         let x = outer_center.x - min_size[0] / 2.0 + 50.;
            //         let y = outer_center.y - min_size[1] / 2.0 + 50.;
            //         Pos2::new(x, y)
            //     }
            //     None => Pos2::new(250.0, 250.0),
            // };
            let about_pos2 = Pos2::new(250.0, 250.0);

            // * Show about viewport
            ctx.show_viewport_deferred(
                ViewportId::from_hash_of("about_viewport"),
                ViewportBuilder::default()
                    .with_title("About")
                    .with_position(about_pos2)
                    .with_inner_size(min_size)
                    .with_resizable(false)
                    .with_maximize_button(false)
                    .with_minimize_button(false)
                    .with_window_level(WindowLevel::Normal)
                    .with_min_inner_size(min_size),
                move |ctx, class| {
                    assert!(
                        class == ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );

                    // * About UI
                    AboutViewport::default().ui(ctx, &is_about_open);
                },
            );
        }
    }
}
