use serde::{Deserialize, Serialize};

use crate::backend::models::{LinkModel, TopicModel};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMessage {
    // Topic
    AddTopic(TopicModel),
    EditTopic(TopicModel),
    RemoveTopic(TopicModel),

    // Link
    AddLink(TopicModel, LinkModel),
    EditLink(TopicModel, LinkModel),
    RemoveLink(TopicModel, LinkModel),

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

impl BookmarkItem {}
