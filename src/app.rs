use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use crossbeam::channel::{unbounded, Receiver, Sender};
use egui::{
    collapsing_header, epaint::Shadow, scroll_area::ScrollBarVisibility, vec2, CentralPanel,
    CursorIcon, FontDefinitions, Frame, Margin, Pos2, Rect, RichText, Rounding, ScrollArea,
    TopBottomPanel, ViewportBuilder, ViewportClass, ViewportCommand, ViewportId, WindowLevel,
};
use egui_phosphor::regular;
use lazy_static::lazy_static;

use crate::{
    backend::bookmark_manager::BookmarkManager,
    ui::{
        about::AboutViewport, add_link::AddLinkViewport, add_topic::AddTopicViewport,
        components::custom_button,
    },
    utils::{
        backup_bookmarks, calc_btn_size_from_text,
        enums::{AppMessage, BookmarkItem, OpenLocationType},
        open_file_location, open_urls,
    },
};

lazy_static! {
    static ref ABOUT_VIEWPORT: AboutViewport = AboutViewport::default();
    static ref ADD_TOPIC_VIEWPORT: Mutex<AddTopicViewport> =
        Mutex::new(AddTopicViewport::default());
    static ref ADD_LINK_VIEWPORT: Mutex<AddLinkViewport> = Mutex::new(AddLinkViewport::default());
    static ref MIN_SIZE: [f32; 2] = [320.0, 240.0];
}

#[derive(Debug)]
pub struct StashApp {
    is_first_run: bool,
    initial_viewport_center: Pos2,
    window_level: WindowLevel,

    is_about_open: Arc<AtomicBool>,
    is_add_topic_open: Arc<AtomicBool>,
    is_add_link_open: Arc<AtomicBool>,

    bookmark_manager: BookmarkManager,
    expanded_topics: Vec<bool>,

    tx: Sender<AppMessage>,
    rx: Receiver<AppMessage>,
}

impl StashApp {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (tx, rx) = unbounded::<AppMessage>();

        let bookmark_manager = BookmarkManager::default();
        let expanded_topics = bookmark_manager
            .get_topics()
            .iter()
            .map(|_| false)
            .collect();

        Self {
            is_first_run: true,
            initial_viewport_center: Pos2::ZERO,
            window_level: WindowLevel::Normal,

            is_about_open: Arc::new(AtomicBool::new(false)),
            is_add_topic_open: Arc::new(AtomicBool::new(false)),
            is_add_link_open: Arc::new(AtomicBool::new(false)),

            bookmark_manager,
            expanded_topics,

            tx,
            rx,
        }
    }

    fn open_add_topic_viewport(&self) {
        let is_add_topic_open = self.is_add_topic_open.load(Ordering::Relaxed);
        self.is_add_topic_open
            .store(!is_add_topic_open, Ordering::Relaxed);
    }

    fn open_add_link_viewport(&self, topic_name: String) {
        ADD_LINK_VIEWPORT
            .lock()
            .expect("Unable to lock AddLinkViewport")
            .set_topic_name(topic_name);

        self.is_add_link_open.store(true, Ordering::Relaxed);
    }

    fn open_edit_topic_viewport(&self, topic_name: String, title: String, url: String) {
        let mut viewport = ADD_LINK_VIEWPORT
            .lock()
            .expect("Unable to lock AddTopicViewport");
        viewport.set_topic_name(topic_name);
        viewport.set_title(title);
        viewport.set_url(url);

        self.is_add_topic_open.store(true, Ordering::Relaxed);
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
                // * Topics
                AppMessage::AddTopic(topic) => {
                    println!("Adding topic: {:?}", topic);

                    self.bookmark_manager.add_topic(BookmarkItem::Topic(topic));
                    self.expanded_topics.push(false);
                    ctx.request_repaint();
                }
                AppMessage::EditTopic(topic) => {
                    println!("Editing topic: {:?}", topic);
                }
                AppMessage::RemoveTopic(topic) => {
                    println!("Removing topic: {:?}", topic);

                    self.bookmark_manager
                        .remove_topic(BookmarkItem::Topic(topic.clone()));

                    let idx = self
                        .bookmark_manager
                        .get_topics()
                        .iter()
                        .position(|t| t == &topic)
                        .unwrap_or_default();
                    self.expanded_topics.remove(idx);

                    ctx.request_repaint();
                }

                // * Links
                AppMessage::AddLink(name, link) => {
                    println!("Adding link: {:?} to topic: {:?}", link, name);

                    let topic = self
                        .bookmark_manager
                        .get_topics()
                        .iter()
                        .find(|t| t.name == name)
                        .expect("Topic not found")
                        .clone();

                    self.bookmark_manager
                        .add_link(BookmarkItem::Topic(topic.clone()), BookmarkItem::Link(link));
                    ctx.request_repaint();
                }
                AppMessage::EditLink(name, link) => {
                    println!("Editing link: {:?} in topic: {:?}", link, name);
                }
                AppMessage::RemoveLink(name, link) => {
                    println!("Removing link: {:?} from topic: {:?}", link, name);

                    let topic = self
                        .bookmark_manager
                        .get_topics()
                        .iter()
                        .find(|t| t.name == name)
                        .expect("Topic not found")
                        .clone();

                    self.bookmark_manager
                        .remove_link(BookmarkItem::Topic(topic.clone()), BookmarkItem::Link(link));
                    ctx.request_repaint();
                }

                // * UI
                AppMessage::ToggleCollapsed(index) => {
                    let is_expanded = self.expanded_topics.get(index).copied().unwrap_or(false);
                    self.expanded_topics[index] = !is_expanded;
                    ctx.request_repaint();
                }

                // * Misc
                AppMessage::ToggleAlwaysOnTop => {
                    self.window_level = match self.window_level {
                        WindowLevel::Normal => {
                            ctx.send_viewport_cmd(ViewportCommand::Title(
                                "Stash: AlwaysOnTop".to_owned(),
                            ));
                            WindowLevel::AlwaysOnTop
                        }
                        WindowLevel::AlwaysOnTop => {
                            ctx.send_viewport_cmd(ViewportCommand::Title("Stash".to_owned()));
                            WindowLevel::Normal
                        }
                        _ => WindowLevel::Normal,
                    };
                    ctx.send_viewport_cmd(ViewportCommand::WindowLevel(self.window_level));
                    ctx.request_repaint();
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
                    if info.hovered() {
                        ui.output_mut(|o| o.cursor_icon = CursorIcon::Default);
                    }
                    info.context_menu(|ui| {
                        if ui.button("Open location").clicked() {
                            open_file_location(OpenLocationType::Documents);
                            ui.close_menu();
                        }
                        if ui.button("Backup bookmarks").clicked() {
                            backup_bookmarks();
                            ui.close_menu();
                        }
                        if ui.button("Toggle AlwaysOnTop").clicked() {
                            self.tx
                                .send(AppMessage::ToggleAlwaysOnTop)
                                .expect("Unable to send");
                            ui.close_menu();
                        }
                        if ui.button("About").clicked() {
                            let is_about_open = self.is_about_open.load(Ordering::Relaxed);
                            self.is_about_open.store(!is_about_open, Ordering::Relaxed);
                            ui.close_menu();
                        }
                    });

                    let available_width = ui.available_width();
                    let label = "Add Topic";

                    ui.add_space(available_width - calc_btn_size_from_text(label));

                    custom_button(ui, label, None, || {
                        self.open_add_topic_viewport();
                    });
                });
            });

        // * Main UI
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                .show(ui, |ui| {
                    let topics_list = self.bookmark_manager.get_topics();
                    for (idx, topic) in topics_list.into_iter().enumerate() {
                        let id_str = format!("topic_{}_{}", topic.name.replace(' ', "_"), idx);
                        let id = ui.make_persistent_id(id_str.clone());
                        let is_expanded = self.expanded_topics.get(idx).copied().unwrap_or(false);

                        ui.push_id(id_str.clone(), |ui| {
                            let mut state =
                                collapsing_header::CollapsingState::load_with_default_open(
                                    ui.ctx(),
                                    id,
                                    is_expanded,
                                );

                            state.set_open(is_expanded);
                            let mut clicked_on_button = false;

                            // ? Topic header
                            let header_res = ui.horizontal(|ui| {
                                ui.allocate_ui(vec2(50., 50.), |ui| {
                                    ui.horizontal_centered(|ui| {
                                        let resp = ui.label(
                                            RichText::new(if !is_expanded {
                                                regular::CARET_RIGHT
                                            } else {
                                                regular::CARET_DOUBLE_DOWN
                                            })
                                            .size(12.),
                                        );
                                        if resp.hovered() {
                                            ui.output_mut(|o| o.cursor_icon = CursorIcon::Default);
                                        }
                                    });
                                });

                                Frame::group(ui.style())
                                    .inner_margin(Margin::same(9.))
                                    .rounding(Rounding::same(9.))
                                    .show(ui, |ui| {
                                        let topic_label =
                                            ui.label(RichText::new(topic.name.clone()).size(20.));
                                        if topic_label.hovered() {
                                            ui.output_mut(|o| o.cursor_icon = CursorIcon::Default);
                                        }

                                        let available_width = ui.available_width();
                                        let label = "Add Link";

                                        ui.add_space(
                                            available_width - calc_btn_size_from_text(label),
                                        );

                                        custom_button(ui, label, None, || {
                                            self.open_add_link_viewport(topic.name.clone());
                                            clicked_on_button = true;
                                        });
                                    });
                            });

                            let header_response = header_res.response.clone();
                            if header_response
                                .rect
                                .contains(ui.input(|i| i.pointer.hover_pos().unwrap_or(Pos2::ZERO)))
                                && ui.input(|i| i.pointer.primary_clicked() && !clicked_on_button)
                            {
                                self.tx
                                    .send(AppMessage::ToggleCollapsed(idx))
                                    .expect("Unable to send");
                            }
                            header_response.context_menu(|ui| {
                                if ui.button("Add Link").clicked() {
                                    self.open_add_link_viewport(topic.name.clone());
                                    ui.close_menu();
                                }
                                if ui.button("Edit topic name").clicked() {
                                    self.tx
                                        .send(AppMessage::EditTopic(topic.clone()))
                                        .expect("Unable to send");
                                    ui.close_menu();
                                }
                                if ui.button("Remove Topic").clicked() {
                                    self.tx
                                        .send(AppMessage::RemoveTopic(topic.clone()))
                                        .expect("Unable to send");
                                    ui.close_menu();
                                }
                            });

                            state.show_body_unindented(ui, |ui| {
                                ui.add_space(5.);

                                // ? Links UI
                                let links = self
                                    .bookmark_manager
                                    .get_links_for_topic(&BookmarkItem::Topic(topic.clone()));
                                if links.is_empty() {
                                    ui.vertical_centered(|ui| {
                                        ui.label("No links!");
                                    });
                                } else {
                                    for (idx, link) in links.iter().enumerate() {
                                        Frame::group(ui.style())
                                            .shadow(Shadow::default())
                                            .rounding(Rounding::same(9.))
                                            .inner_margin(Margin::same(9.))
                                            .show(ui, |ui| {
                                                let id_str = format!(
                                                    "{}_{}_{}",
                                                    topic.name.replace(' ', "_"),
                                                    link.title.replace(' ', "_"),
                                                    idx
                                                );
                                                ui.push_id(id_str, |ui| {
                                                    let link_ui = ui.horizontal(|ui| {
                                                        ui.label(link.title.clone());

                                                        let available_width = ui.available_width();
                                                        let label = "Open";

                                                        ui.add_space(
                                                            available_width
                                                                - calc_btn_size_from_text(label),
                                                        );

                                                        custom_button(ui, label, None, || {
                                                            open_urls(&[link.url.clone()]);
                                                        });
                                                    });
                                                    link_ui.response.context_menu(|ui| {
                                                        if ui.button("Edit link").clicked() {
                                                            self.open_edit_topic_viewport(
                                                                topic.name.clone(),
                                                                link.title.clone(),
                                                                link.url.clone(),
                                                            );
                                                            ui.close_menu();
                                                        }
                                                        if ui.button("Remove link").clicked() {
                                                            self.tx
                                                                .send(AppMessage::RemoveLink(
                                                                    topic.name.clone(),
                                                                    link.clone(),
                                                                ))
                                                                .expect("Unable to send");
                                                            ui.close_menu();
                                                        }
                                                    });
                                                });
                                            });

                                        if idx < links.len() - 1 {
                                            ui.add_space(5.);
                                        }
                                    }
                                }
                            });
                        });

                        ui.add_space(5.);
                    }
                });
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

        // * Add link viewport
        if self.is_add_link_open.load(Ordering::Relaxed) {
            let is_add_link_open = self.is_add_link_open.clone();
            let tx = self.tx.clone();

            let add_link_pos2 = self.initial_viewport_center;
            let min_size = *MIN_SIZE;
            let width = min_size[0];
            let height = min_size[1] + 50.;
            let min_size = [width, height];

            // * Show add link viewport
            ctx.show_viewport_deferred(
                ViewportId::from_hash_of("add_link_viewport"),
                ViewportBuilder::default()
                    .with_title("Add Link")
                    .with_position(add_link_pos2)
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

                    // * Add link UI
                    ADD_LINK_VIEWPORT
                        .lock()
                        .expect("Unable to lock AddLinkViewport")
                        .ui(ctx, &is_add_link_open, &tx);
                },
            );
        }
    }
}
