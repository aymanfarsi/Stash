use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crossbeam::channel::{Receiver, Sender};
use egui::{
    CentralPanel, CursorIcon, FontDefinitions, Pos2, Rect, RichText, TopBottomPanel,
    ViewportBuilder, ViewportClass, ViewportId, WindowLevel,
};
use egui_phosphor::regular;

use crate::{
    ui::{
        about::AboutViewport,
        components::{calc_btn_size_from_text, custom_button, custom_header},
    },
    utils::enums::AppMessage,
};

#[derive(Debug)]
pub struct StashApp {
    is_first_run: bool,
    initial_viewport_center: Pos2,

    pub is_about_open: Arc<AtomicBool>,

    tx: Sender<AppMessage>,
    rx: Receiver<AppMessage>,
}

impl Default for StashApp {
    fn default() -> Self {
        let (tx, rx) = crossbeam::channel::unbounded::<AppMessage>();

        Self {
            is_first_run: true,
            initial_viewport_center: Pos2::ZERO,

            is_about_open: Arc::new(AtomicBool::new(false)),

            tx,
            rx,
        }
    }
}

impl eframe::App for StashApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // * First run
        if self.is_first_run {
            self.is_first_run = false;

            let center = ctx
                .input(|i| i.viewport().outer_rect)
                .unwrap_or(Rect::NOTHING)
                .center();
            self.initial_viewport_center = Pos2::new(center.x - 170., center.y - 120.);

            egui_extras::install_image_loaders(ctx);

            let mut fonts = FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            ctx.set_fonts(fonts);
        }

        // * Handle app messages
        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                AppMessage::AddTopic => {
                    println!("Add Topic");
                }
                AppMessage::AddLink => {
                    println!("Add Link");
                }
            }
        }

        // * Top panel
        TopBottomPanel::top("top_panel")
            .resizable(false)
            .show_separator_line(false)
            .default_height(35.0)
            .show(ctx, |ui| {
                ui.add_space(5.);
                ui.horizontal(|ui| {
                    let info = ui.label(RichText::new(regular::INFO.to_string()).size(20.));
                    if info.clicked() {
                        let is_about_open = self.is_about_open.load(Ordering::Relaxed);
                        self.is_about_open.store(!is_about_open, Ordering::Relaxed);
                    }
                    if info.hovered() {
                        ui.output_mut(|o| o.cursor_icon = CursorIcon::Default);
                    }

                    let available_width = ui.available_width();
                    let label = "Add Topic";

                    ui.add_space(available_width - calc_btn_size_from_text(label));

                    custom_button(ui, label, || {
                        self.tx
                            .send(AppMessage::AddTopic)
                            .expect("Failed to send AddTopic message");
                    });
                });
            });

        // * Main UI
        CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new("Stash").size(32.0));

            ui.add_space(10.0);

            custom_header(ui, "Topic 1", || {
                println!("Expand Topic 1");
            });

            ui.add_space(10.0);

            custom_header(ui, "Link 1", || {
                println!("View Link 1");
            });
        });

        // * About viewport
        if self.is_about_open.load(Ordering::Relaxed) {
            let is_about_open = self.is_about_open.clone();

            let min_size = [320.0, 240.0];
            let about_pos2 = self.initial_viewport_center;

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
