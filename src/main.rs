#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{icon_data::from_png_bytes, Theme};
use egui::ViewportBuilder;
use tokio::runtime::Runtime;

use stash::app::StashApp;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let rt = Runtime::new().expect("Unable to create Runtime");
    let _enter = rt.enter();

    let min_size = [320.0, 240.0];
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(min_size)
            .with_min_inner_size(min_size)
            .with_decorations(true)
            .with_transparent(false)
            .with_close_button(true)
            .with_maximize_button(false)
            .with_minimize_button(true)
            .with_titlebar_buttons_shown(true)
            .with_drag_and_drop(true)
            .with_active(true)
            .with_resizable(true)
            .with_taskbar(true)
            .with_visible(true)
            .with_icon(
                from_png_bytes(include_bytes!("../assets/app-icon.png"))
                    .expect("Failed to load icon"),
            )
            .with_app_id("io.github.aymanfarsi.stash"),
        default_theme: Theme::Dark,
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Stash",
        options,
        Box::new(move |_cc| Box::<StashApp>::default()),
    )
}
