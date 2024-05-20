use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use egui::{
    include_image, vec2, CentralPanel, Hyperlink, Image, Pos2, RichText, Rounding, WindowLevel,
};

#[derive(Debug)]
pub struct StashApp {
    version: String,
    description: String,

    pub is_first_run: bool,

    pub is_about_open: Arc<AtomicBool>,
}

impl Default for StashApp {
    fn default() -> Self {
        let cargo_text = include_str!("../Cargo.toml");
        let version = cargo_text
            .lines()
            .find(|line| line.starts_with("version = "))
            .map(|line| line.split('=').last().unwrap().trim())
            .unwrap_or("unknown")
            .replace('"', "");
        let description = cargo_text
            .lines()
            .find(|line| line.starts_with("description = "))
            .map(|line| line.split('=').last().unwrap().trim())
            .unwrap_or("unknown")
            .replace('"', "");

        Self {
            version,
            description,
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

        // * About
        if self.is_about_open.load(Ordering::Relaxed) {
            let is_about_open = self.is_about_open.clone();
            let version = self.version.clone();
            let description = self.description.clone();

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

            ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("about_viewport"),
                egui::ViewportBuilder::default()
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
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );

                    let version = version.clone();
                    let description = description.clone();
                    CentralPanel::default().show(ctx, |ui| {
                        ui.add_space(7.0);

                        ui.vertical_centered(|ui| {
                            ui.add(
                                Image::new(include_image!("../assets/app-icon.png"))
                                    .rounding(Rounding::same(4.0))
                                    .fit_to_exact_size(vec2(100., 100.))
                                    .show_loading_spinner(true),
                            );
                            ui.add_space(7.0);

                            ui.vertical_centered(|ui| {
                                ui.label(RichText::new("Stash").size(24.0).strong());
                                ui.label(RichText::new(format!("v{}", version)).size(14.0));
                            });

                            ui.add_space(7.0);

                            ui.label(RichText::new(description).size(16.0));
                        });

                        ui.add_space(7.0);

                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.label("Developed by ");

                            ui.add(
                                Hyperlink::from_label_and_url(
                                    "Ayman Farsi",
                                    "https://aymanfarsi.github.io/",
                                )
                                .open_in_new_tab(true),
                            )
                            .on_hover_ui(|ui| {
                                ui.label(RichText::new("Personal portfolio"));
                            });

                            ui.label(" with ♥ using ");
                            ui.add(
                                Hyperlink::from_label_and_url("Rust", "https://www.rust-lang.org/")
                                    .open_in_new_tab(true),
                            )
                            .on_hover_ui(|ui| {
                                ui.label(RichText::new("Programming language"));
                            });

                            ui.label(" and ");
                            ui.add(
                                Hyperlink::from_label_and_url(
                                    "egui",
                                    "https://github.com/emilk/egui",
                                )
                                .open_in_new_tab(true),
                            )
                            .on_hover_ui(|ui| {
                                ui.label(RichText::new("Immediate mode GUI library by emilk"));
                            });
                            ui.label(".");
                        });
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
