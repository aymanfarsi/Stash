use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::backend::models::{LinkModel, TopicModel};

#[derive(Debug, Clone, PartialEq)]
pub enum AppPage {
    Main,
    Settings,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppMessage {
    // Topic
    AddTopic(TopicModel),
    EditTopic(TopicModel, TopicModel),
    RemoveTopic(TopicModel),

    // Link
    AddLink(String, LinkModel),
    EditLink(String, LinkModel, LinkModel),
    RemoveLink(String, LinkModel),

    // UI
    ToggleCollapsed(usize),

    // Misc
    ToggleAlwaysOnTop,
}

impl AppMessage {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BookmarkItem {
    Topic(TopicModel),
    Link(LinkModel),
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpenLocationType {
    Documents,
    Custom(PathBuf),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AppTheme {
    #[serde(rename = "light")]
    Light,

    #[serde(rename = "dark")]
    Dark,

    #[serde(rename = "latte")]
    LATTE,

    #[serde(rename = "frappe")]
    FRAPPE,

    #[serde(rename = "macchiato")]
    MACCHIATO,

    #[serde(rename = "mocha")]
    MOCHA,
}

impl AppTheme {
    pub fn name(&self) -> &str {
        match self {
            AppTheme::Light => "Light",
            AppTheme::Dark => "Dark",
            AppTheme::LATTE => "Latte",
            AppTheme::FRAPPE => "Frappe",
            AppTheme::MACCHIATO => "Macchiato",
            AppTheme::MOCHA => "Mocha",
        }
    }

    pub fn values() -> [AppTheme; 6] {
        [
            AppTheme::Light,
            AppTheme::Dark,
            AppTheme::LATTE,
            AppTheme::FRAPPE,
            AppTheme::MACCHIATO,
            AppTheme::MOCHA,
        ]
    }

    pub fn set_theme(&self, ctx: &egui::Context) {
        match self {
            AppTheme::Light => {
                ctx.set_visuals(egui::Visuals::light());
            }
            AppTheme::Dark => {
                ctx.set_visuals(egui::Visuals::dark());
            }
            AppTheme::LATTE => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE);
            }
            AppTheme::FRAPPE => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE);
            }
            AppTheme::MACCHIATO => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);
            }
            AppTheme::MOCHA => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
            }
        }
    }

    pub fn get_highlight_color(&self) -> egui::Color32 {
        match self {
            AppTheme::Light => egui::Color32::from_rgb(255, 255, 255),
            AppTheme::Dark => egui::Color32::from_rgb(0, 0, 0),
            AppTheme::LATTE => catppuccin_egui::LATTE.blue,
            AppTheme::FRAPPE => catppuccin_egui::FRAPPE.blue,
            AppTheme::MACCHIATO => catppuccin_egui::MACCHIATO.blue,
            AppTheme::MOCHA => catppuccin_egui::MOCHA.blue,
        }
    }
}
