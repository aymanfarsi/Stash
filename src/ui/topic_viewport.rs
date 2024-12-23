use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crossbeam::channel::Sender;
use egui::{CentralPanel, Frame, Grid, Key, Margin, RichText, Rounding, ViewportCommand};

use crate::{backend::models::TopicModel, utils::enums::AppMessage};

use super::components::custom_button;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TopicViewport {
    old_name: String,
    new_name: String,

    is_editing: bool,
}

impl TopicViewport {
    pub fn set_old_name(&mut self, name: String) {
        self.old_name = name;
    }

    pub fn set_new_name(&mut self, name: String) {
        self.new_name = name;
    }

    pub fn set_editing(&mut self, is_editing: bool) {
        self.is_editing = is_editing;
    }

    pub fn ui(&mut self, ctx: &egui::Context, is_open: &Arc<AtomicBool>, tx: &Sender<AppMessage>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add_space(9.);

            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("Add Topic")
                        .strong()
                        .size(24.)
                        .heading()
                        .extra_letter_spacing(1.),
                );
            });

            ui.add_space(9.);

            Grid::new("add_topic_grid")
                .num_columns(2)
                .striped(false)
                .spacing([35., 9.])
                .show(ui, |ui| {
                    ui.label("Name:");

                    ui.text_edit_singleline(&mut self.new_name);
                    if ui.input(|i| i.key_pressed(Key::Enter)) {
                        self.send_topic(ctx, tx);
                    }

                    ui.end_row();
                });

            ui.add_space(9.);

            ui.horizontal(|ui| {
                let available_width = ui.available_width();
                let button_width = 75.;
                let button_spacing = 9.;
                let spacing_around = (available_width - button_width * 2. - button_spacing) / 2.;

                ui.add_space(spacing_around);

                let text = if self.is_editing { "Update" } else { "Add" };
                custom_button(ui, text, Some(button_width), || {
                    self.send_topic(ctx, tx);
                });

                ui.add_space(button_spacing);

                custom_button(ui, "Cancel", Some(button_width), || {
                    self.clear_exit_viewport(ctx, true);
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

                        ui.label(RichText::new(
                            "Press 'Enter' or click on button to add topic.\n",
                        ));
                        ui.label(RichText::new(
                            "If the new topic already exists, it will be ignored. ",
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

    fn send_topic(&mut self, ctx: &egui::Context, tx: &Sender<AppMessage>) {
        if !self.new_name.is_empty() {
            let new_topic = TopicModel::new(self.new_name.clone());
            let msg = if self.is_editing {
                let old_topic = TopicModel::new(self.old_name.clone());
                AppMessage::EditTopic(old_topic, new_topic)
            } else {
                AppMessage::AddTopic(new_topic)
            };
            let res = tx.send(msg);
            match res {
                Ok(_) => {
                    self.clear_exit_viewport(ctx, false);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

    fn clear_exit_viewport(&mut self, ctx: &egui::Context, should_exit: bool) {
        self.new_name.clear();

        if self.is_editing || should_exit {
            self.is_editing = false;
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
    }
}
