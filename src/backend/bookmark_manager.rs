use indexmap::IndexMap;

use super::models::{LinkModel, TopicModel};
use crate::utils::enums::BookmarkItem;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BookmarkManager {
    bookmarks: IndexMap<BookmarkItem, Vec<BookmarkItem>>,
}

impl BookmarkManager {
    pub fn add_topic(&mut self, topic: BookmarkItem) {
        if self.bookmarks.contains_key(&topic) {
            return;
        }

        self.bookmarks.insert(topic, vec![]);
    }

    pub fn remove_topic(&mut self, topic: BookmarkItem) {
        self.bookmarks.shift_remove(&topic);
    }

    pub fn get_topics(&self) -> Vec<TopicModel> {
        self.bookmarks
            .keys()
            .filter_map(|item| match item {
                BookmarkItem::Topic(topic) => Some(topic.clone()),
                _ => None,
            })
            .collect()
    }

    pub fn add_link(&mut self, topic: BookmarkItem, link: BookmarkItem) {
        if !self.bookmarks.contains_key(&topic) {
            self.bookmarks.insert(topic.clone(), vec![]);
        } else {
            self.bookmarks.get_mut(&topic).unwrap().push(link);
        }
    }

    pub fn remove_link(&mut self, topic: BookmarkItem, link: BookmarkItem) {
        if self.bookmarks.contains_key(&topic) {
            let links = self.bookmarks.get_mut(&topic).unwrap();
            if let Some(index) = links.iter().position(|l| l == &link) {
                links.remove(index);
            }
        }
    }

    pub fn get_links_for_topic(&self, topic: &BookmarkItem) -> Vec<LinkModel> {
        self.bookmarks
            .get(topic)
            .map(|links| {
                links
                    .iter()
                    .filter_map(|item| match item {
                        BookmarkItem::Link(link) => Some(link.clone()),
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}
