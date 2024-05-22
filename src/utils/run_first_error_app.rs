use eframe::{icon_data::from_png_bytes, Theme};
use egui::{CentralPanel, RichText, ViewportBuilder, WindowLevel};

pub fn run_first_error_app(error: String) -> Result<(), eframe::Error> {
    let min_size = [320.0, 240.0];
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(min_size)
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_window_level(WindowLevel::Normal)
            .with_min_inner_size(min_size)
            .with_icon(
                from_png_bytes(include_bytes!("../../assets/app-icon.png"))
                    .expect("Failed to load icon"),
            )
            .with_app_id("io.github.aymanfarsi.stash"),
        default_theme: Theme::Dark,
        centered: true,
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "Stash: Error",
        options,
        Box::new(move |_cc| Box::new(ErrorApp { error })),
    )
}

#[derive(Default, Debug, Clone, PartialEq)]
struct ErrorApp {
    error: String,
}

impl eframe::App for ErrorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // * Main UI
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(&self.error));
            });
        });
    }
}
