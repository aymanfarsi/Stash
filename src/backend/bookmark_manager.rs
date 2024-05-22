use std::collections::{hash_map::Entry, HashMap};

use crate::utils::enums::BookmarkItem;

use super::models::{LinkModel, TopicModel};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BookmarkManager {
    bookmarks: HashMap<BookmarkItem, Vec<BookmarkItem>>,
}

impl BookmarkManager {
    pub fn add_topic(&mut self, topic: BookmarkItem) {
        if self.bookmarks.contains_key(&topic) {
            return;
        }

        self.bookmarks.insert(topic, vec![]);
    }

    pub fn remove_topic(&mut self, topic: BookmarkItem) {
        self.bookmarks.remove(&topic);
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
        if let Entry::Vacant(e) = self.bookmarks.entry(topic.clone()) {
            e.insert(vec![link]);
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
