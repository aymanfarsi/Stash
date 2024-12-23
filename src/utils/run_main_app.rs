use eframe::{icon_data::from_png_bytes, Theme};
use egui::{ViewportBuilder, X11WindowType};

use crate::app::StashApp;

pub fn run_main_app() -> Result<(), eframe::Error> {
    let min_size = [350.0, 500.0];
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(min_size)
            .with_min_inner_size(min_size)
            .with_decorations(true)
            .with_transparent(false)
            .with_close_button(true)
            .with_maximize_button(true)
            .with_minimize_button(true)
            .with_drag_and_drop(false)
            .with_active(true)
            .with_resizable(true)
            .with_taskbar(true)
            .with_visible(true)
            .with_icon(
                from_png_bytes(include_bytes!("../../assets/stash.png"))
                    .expect("Failed to load icon"),
            )
            .with_app_id("io.github.aymanfarsi.stash")
            .with_window_type(X11WindowType::Normal),
        default_theme: Theme::Dark,
        centered: true,
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "Stash",
        options,
        Box::new(move |_cc| {
            let app = StashApp::new();

            Box::new(app)
        }),
    )
}
