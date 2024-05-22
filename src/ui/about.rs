use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use egui::{include_image, vec2, CentralPanel, Hyperlink, Image, RichText, Rounding};
use lazy_static::lazy_static;

lazy_static! {
    static ref VERSION: String = {
        let cargo_text = include_str!("../../Cargo.toml");
        cargo_text
            .lines()
            .find(|line| line.starts_with("version = "))
            .map(|line| line.split('=').last().unwrap().trim())
            .unwrap_or("unknown")
            .replace('"', "")
    };
    static ref DESCRIPTION: String = {
        let cargo_text = include_str!("../../Cargo.toml");
        cargo_text
            .lines()
            .find(|line| line.starts_with("description = "))
            .map(|line| line.split('=').last().unwrap().trim())
            .unwrap_or("unknown")
            .replace('"', "")
    };
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AboutViewport {}

impl AboutViewport {
    pub fn ui(&self, ctx: &egui::Context, is_open: &Arc<AtomicBool>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add_space(7.0);

            ui.vertical_centered(|ui| {
                ui.add(
                    Image::new(include_image!("../../assets/app-icon.png"))
                        .rounding(Rounding::same(4.0))
                        .fit_to_exact_size(vec2(100., 100.))
                        .show_loading_spinner(true),
                );
                ui.add_space(7.0);

                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Stash").size(24.0).strong());
                    ui.label(
                        RichText::new(format!("v{}", *VERSION))
                            .size(14.0)
                            .small()
                            .underline(),
                    );
                });

                ui.add_space(7.0);

                ui.label(RichText::new((*DESCRIPTION).to_string()).size(16.0));
            });

            ui.add_space(7.0);

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("Developed by ");

                ui.add(
                    Hyperlink::from_label_and_url("Ayman Farsi", "https://aymanfarsi.github.io")
                        .open_in_new_tab(true),
                )
                .on_hover_ui(|ui| {
                    ui.label(RichText::new("Personal portfolio"));
                });

                ui.label(" with ♥ using ");
                ui.add(
                    Hyperlink::from_label_and_url("Rust", "https://www.rust-lang.org")
                        .open_in_new_tab(true),
                )
                .on_hover_ui(|ui| {
                    ui.label(RichText::new("Programming language"));
                });

                ui.label(" and ");
                ui.add(
                    Hyperlink::from_label_and_url("egui", "https://github.com/emilk/egui")
                        .open_in_new_tab(true),
                )
                .on_hover_ui(|ui| {
                    ui.label(RichText::new("Immediate GUI library by emilk"));
                });
                ui.label(".");
            });
        });

        // * Close viewport on close button
        if ctx.input(|i| i.viewport().close_requested()) {
            // Tell parent to close us.
            is_open.store(false, Ordering::Relaxed);
        }
    }
}
