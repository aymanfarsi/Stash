use crate::backend::models::{LinkModel, TopicModel};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMessage {
    AddTopic(TopicModel),
    AddLink(TopicModel, LinkModel),
}

impl AppMessage {}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BookmarkItem {
    Topic(TopicModel),
    Link(LinkModel),
}

impl BookmarkItem {}
