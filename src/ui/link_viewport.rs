use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crossbeam::channel::Sender;
use egui::{CentralPanel, Frame, Grid, Key, Margin, RichText, Rounding, ViewportCommand};

use crate::{backend::models::LinkModel, utils::enums::AppMessage};

use super::components::custom_button;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LinkViewport {
    topic_name: String,

    old_title: String,
    old_url: String,

    new_title: String,
    new_url: String,
    preview: Option<String>,

    is_editing: bool,
}

impl LinkViewport {
    pub fn set_topic_name(&mut self, header: String) {
        self.topic_name = header;
    }

    pub fn set_old_title(&mut self, title: String) {
        self.old_title = title;
    }

    pub fn set_old_url(&mut self, url: String) {
        self.old_url = url;
    }

    pub fn set_new_title(&mut self, title: String) {
        self.new_title = title;
    }

    pub fn set_new_url(&mut self, url: String) {
        self.new_url = url;
    }

    pub fn set_is_editing(&mut self, is_editing: bool) {
        self.is_editing = is_editing;
    }

    pub fn ui(&mut self, ctx: &egui::Context, is_open: &Arc<AtomicBool>, tx: &Sender<AppMessage>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add_space(9.);

            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(self.topic_name.clone())
                        .strong()
                        .size(24.)
                        .heading()
                        .extra_letter_spacing(1.),
                );
            });

            ui.add_space(9.);

            Grid::new("add_link_grid")
                .num_columns(2)
                .striped(false)
                .spacing([35., 9.])
                .show(ui, |ui| {

                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.new_title);
                    if ui.input(|i| i.key_pressed(Key::Enter)) {
                        self.send_link(ctx, tx);
                    }

                    ui.end_row();

                    ui.label("URL:");
                    ui.text_edit_singleline(&mut self.new_url);
                    if ui.input(|i| i.key_pressed(Key::Enter)) {
                        self.send_link(ctx, tx);
                    }

                    ui.end_row();

                    ui.label("Preview:");
                    ui.label("Not implemented yet.");

                    ui.end_row();
                });

            ui.add_space(9.);

            ui.horizontal(|ui| {
                let available_width = ui.available_width();
                let button_width = 75.;
                let button_spacing = 9.;
                let spacing_around = (available_width - button_width * 2. - button_spacing) / 2.;

                ui.add_space(spacing_around);

                let text = if self.is_editing { "Edit" } else { "Add" };
                custom_button(ui, text, Some(button_width), || {
                    self.send_link(ctx, tx);
                });

                ui.add_space(button_spacing);

                custom_button(ui, "Cancel", Some(button_width), || {
                    self.exit_viewport(ctx, true);
                });

                ui.add_space(spacing_around);
            });

            Frame::group(ui.style())
                .rounding(Rounding::same(9.))
                .outer_margin(Margin::same(9.))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.;
                        ui.spacing_mut().item_spacing.y = 5.;
                        ui.set_width(ui.available_width() - 18.);

                        ui.label(RichText::new(
                            "Press 'Enter' or click on button after filling the fields to add link.\n\n",
                        ));
                        ui.label(RichText::new(
                            "If the new link already exists with is the topic (both title and url), it will be ignored.",
                        ));
                    });
                });
        });

        // * Close viewport on close button
        if ctx.input(|i| i.viewport().close_requested()) {
            // Tell parent to close us.
            is_open.store(false, Ordering::Relaxed);
        }
    }

    fn send_link(&mut self, ctx: &egui::Context, tx: &Sender<AppMessage>) {
        if !self.new_title.is_empty() && !self.new_url.is_empty() {
            let link = LinkModel::new(
                self.new_title.clone(),
                self.new_url.clone(),
                self.preview.clone(),
            );
            let msg = if self.is_editing {
                AppMessage::EditLink(
                    self.topic_name.clone(),
                    LinkModel::new(self.old_title.clone(), self.old_url.clone(), None),
                    link.clone(),
                )
            } else {
                AppMessage::AddLink(self.topic_name.clone(), link)
            };
            let res = tx.send(msg);
            match res {
                Ok(_) => {
                    self.exit_viewport(ctx, false);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

    fn exit_viewport(&mut self, ctx: &egui::Context, should_exit: bool) {
        self.new_title.clear();
        self.new_url.clear();
        self.preview = None;

        if self.is_editing || should_exit {
            self.topic_name.clear();
            self.old_title.clear();
            self.old_url.clear();
            self.is_editing = false;
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
    }
}
