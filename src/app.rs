use crossbeam::channel::{unbounded, Receiver, Sender};
use egui::{CentralPanel, RichText, ViewportCommand};
use tray_icon::{menu::MenuEvent, MouseButton, MouseButtonState, TrayIconEvent};

use crate::enums::TrayMessage;

#[derive(Debug)]
pub struct StashApp {
    tx_tray: Sender<TrayMessage>,
    rx_tray: Receiver<TrayMessage>,

    pub is_first_run: bool,
}

impl Default for StashApp {
    fn default() -> Self {
        let (tx_tray, rx_tray) = unbounded::<TrayMessage>();

        Self {
            tx_tray,
            rx_tray,
            is_first_run: true,
        }
    }
}

impl eframe::App for StashApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_first_run {
            self.is_first_run = false;

            // * Tray Icon event loop
            let tx_tray = self.tx_tray.clone();
            let ctx_tray = ctx.clone();
            tokio::spawn(async move {
                loop {
                    // * Tray Icon events
                    if let Ok(event) = TrayIconEvent::receiver().try_recv() {
                        match event {
                            TrayIconEvent::Click {
                                id: _,
                                position: _,
                                rect: _,
                                button,
                                button_state,
                            } => {
                                if let MouseButton::Left = button {
                                    if let MouseButtonState::Up = button_state {
                                        ctx_tray.send_viewport_cmd(ViewportCommand::Focus);
                                    }
                                }
                            }
                            TrayIconEvent::Enter { .. } => {}
                            TrayIconEvent::Leave { .. } => {}
                            TrayIconEvent::Move { .. } => {}
                            _ => {
                                println!("Unhandled tray event");
                                println!("{:?}", event);
                            }
                        }
                        ctx_tray.request_repaint();
                    }

                    // * Tray Icon menu events
                    if let Ok(event) = MenuEvent::receiver().try_recv() {
                        let msg = TrayMessage::from_string(event.id.0);
                        match msg {
                            TrayMessage::ShowHide => {
                                // tx_tray
                                //     .send(TrayMessage::ShowHide)
                                //     .expect("Failed to send tray message");
                                ctx_tray.send_viewport_cmd(ViewportCommand::Focus);
                            }

                            TrayMessage::Quit => {
                                tx_tray
                                    .send(TrayMessage::Quit)
                                    .expect("Failed to send tray message");
                            }
                        }
                        ctx_tray.request_repaint();
                    }
                }
            });
        }

        // * Handle tray messages
        if let Ok(msg) = self.rx_tray.try_recv() {
            match msg {
                TrayMessage::ShowHide => {
                    ctx.send_viewport_cmd(ViewportCommand::Visible(true));
                }

                TrayMessage::Quit => {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            }
        }

        // * Main UI
        CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new("Stash").size(32.0));
        });
    }
}
