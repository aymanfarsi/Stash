use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::backend::models::{LinkModel, TopicModel};

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
