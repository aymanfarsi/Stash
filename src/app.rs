use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use crossbeam::channel::{unbounded, Receiver, Sender};
use egui::{
    CentralPanel, CursorIcon, FontDefinitions, Pos2, Rect, RichText, TopBottomPanel,
    ViewportBuilder, ViewportClass, ViewportId, WindowLevel,
};
use egui_phosphor::regular;
use lazy_static::lazy_static;

use crate::{
    backend::bookmark_manager::BookmarkManager,
    ui::{about::AboutViewport, add_topic::AddTopicViewport, components::custom_button},
    utils::{
        calc_btn_size_from_text,
        enums::{AppMessage, BookmarkItem},
    },
};

lazy_static! {
    static ref ABOUT_VIEWPORT: AboutViewport = AboutViewport::default();
    static ref ADD_TOPIC_VIEWPORT: Mutex<AddTopicViewport> =
        Mutex::new(AddTopicViewport::default());
    static ref MIN_SIZE: [f32; 2] = [320.0, 240.0];
}

#[derive(Debug)]
pub struct StashApp {
    is_first_run: bool,
    initial_viewport_center: Pos2,

    is_about_open: Arc<AtomicBool>,
    is_add_topic_open: Arc<AtomicBool>,

    bookmark_manager: BookmarkManager,

    tx: Sender<AppMessage>,
    rx: Receiver<AppMessage>,
}

impl StashApp {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (tx, rx) = unbounded::<AppMessage>();

        Self {
            is_first_run: true,
            initial_viewport_center: Pos2::ZERO,

            is_about_open: Arc::new(AtomicBool::new(false)),

            is_add_topic_open: Arc::new(AtomicBool::new(false)),

            bookmark_manager: BookmarkManager::default(),

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
                AppMessage::AddTopic(topic) => {
                    println!("Adding topic: {:?}", topic);

                    self.bookmark_manager.add_topic(BookmarkItem::Topic(topic));
                }
                AppMessage::AddLink(topic, link) => {
                    println!("Adding link: {:?} to topic: {:?}", link, topic.name);

                    self.bookmark_manager
                        .add_link(BookmarkItem::Topic(topic), BookmarkItem::Link(link));
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

                    custom_button(ui, label, None, || {
                        let is_add_topic_open = self.is_add_topic_open.load(Ordering::Relaxed);
                        self.is_add_topic_open
                            .store(!is_add_topic_open, Ordering::Relaxed);
                    });
                });
            });

        // * Main UI
        CentralPanel::default().show(ctx, |ui| {
            let topics = self.bookmark_manager.get_topics();
            for topic in topics {
                ui.label(RichText::new(topic.name.clone()).size(20.));

                let links = self
                    .bookmark_manager
                    .get_links_for_topic(&BookmarkItem::Topic(topic));
                if links.is_empty() {
                    ui.label("| No links found");
                } else {
                    for link in links {
                        ui.label(format!("| {}", link.title));
                    }
                }
            }
        });

        // * About viewport
        if self.is_about_open.load(Ordering::Relaxed) {
            let is_about_open = self.is_about_open.clone();

            let about_pos2 = self.initial_viewport_center;
            let min_size = *MIN_SIZE;

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
                    ABOUT_VIEWPORT.ui(ctx, &is_about_open);
                },
            );
        }

        // * Add topic viewport
        if self.is_add_topic_open.load(Ordering::Relaxed) {
            let is_add_topic_open = self.is_add_topic_open.clone();
            let tx = self.tx.clone();

            let add_topic_pos2 = self.initial_viewport_center;
            let min_size = *MIN_SIZE;

            // * Show add topic viewport
            ctx.show_viewport_deferred(
                ViewportId::from_hash_of("add_topic_viewport"),
                ViewportBuilder::default()
                    .with_title("Add Topic")
                    .with_position(add_topic_pos2)
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

                    // * Add topic UI
                    ADD_TOPIC_VIEWPORT
                        .lock()
                        .expect("Unable to lock AddTopicViewport")
                        .ui(ctx, &is_add_topic_open, &tx);
                },
            );
        }
    }
}
