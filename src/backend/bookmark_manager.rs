use std::{collections::HashMap, fs};

use directories::UserDirs;
use indexmap::IndexMap;

use super::models::{LinkModel, TopicModel};
use crate::utils::enums::BookmarkItem;

#[derive(Debug, Clone, PartialEq)]
pub struct BookmarkManager {
    path: String,
    pub filename: String,
    bookmarks: IndexMap<BookmarkItem, Vec<BookmarkItem>>,
}

impl BookmarkManager {
    pub fn new(is_debug: bool) -> Self {
        let dirs = UserDirs::new().expect("Failed to get user directories");
        let documents = dirs
            .document_dir()
            .expect("Failed to get document directory");
        let path = documents.join("stash");
        if !path.exists() {
            fs::create_dir_all(&path).expect("Failed to create stash directory");
        }

        let filename = if is_debug {
            "bookmarks_debug.json"
        } else {
            "bookmarks.json"
        };

        let mut bookmarks = IndexMap::new();
        if let Ok(data) = fs::read_to_string(format!("{}/{}", path.to_str().unwrap(), filename)) {
            let json: HashMap<String, Vec<LinkModel>> =
                serde_json::from_str(&data).expect("Failed to deserialize bookmarks");
            let mut json = json.into_iter().collect::<Vec<(String, Vec<LinkModel>)>>();
            json.sort_by(|a, b| a.0.cmp(&b.0));

            for (topic, links) in json {
                let mut split = topic.split('_').collect::<Vec<&str>>();
                split.remove(0);
                let name = if split.len() > 1 {
                    split.join("_")
                } else {
                    split[0].to_string()
                };

                bookmarks.insert(
                    BookmarkItem::Topic(TopicModel { name }),
                    links.into_iter().map(BookmarkItem::Link).collect(),
                );
            }
        }

        Self {
            path: path
                .to_str()
                .expect("Failed to convert path to string")
                .to_string(),
            filename: filename.to_string(),
            bookmarks,
        }
    }

    pub fn add_topic(&mut self, topic: BookmarkItem) {
        if self.bookmarks.contains_key(&topic) {
            return;
        }

        self.bookmarks.insert(topic, vec![]);
        self.save_bookmarks();
    }

    pub fn edit_topic(&mut self, old_topic: BookmarkItem, new_topic: BookmarkItem) {
        if !self.bookmarks.contains_key(&old_topic) {
            return;
        }

        let bookmarks = self.bookmarks.clone();
        let (idx, _, value) = bookmarks.get_full(&old_topic).expect("Failed to get topic");
        self.remove_topic(old_topic);
        self.bookmarks.shift_insert(idx, new_topic, value.to_vec());
    }

    pub fn remove_topic(&mut self, topic: BookmarkItem) {
        self.bookmarks.shift_remove(&topic);
        self.save_bookmarks();
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

        self.save_bookmarks();
    }

    pub fn edit_link(
        &mut self,
        topic: BookmarkItem,
        old_link: BookmarkItem,
        new_link: BookmarkItem,
    ) {
        if self.bookmarks.contains_key(&topic) {
            let links = self.bookmarks.get_mut(&topic).unwrap();
            if let Some(index) = links.iter().position(|l| l == &old_link) {
                links[index] = new_link;
            }
        }

        self.save_bookmarks();
    }

    pub fn remove_link(&mut self, topic: BookmarkItem, link: BookmarkItem) {
        if self.bookmarks.contains_key(&topic) {
            let links = self.bookmarks.get_mut(&topic).unwrap();
            if let Some(index) = links.iter().position(|l| l == &link) {
                links.remove(index);
            }
        }

        self.save_bookmarks();
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

    pub fn save_bookmarks(&self) {
        let mut data: HashMap<String, Vec<LinkModel>> = HashMap::new();
        let mut count = 0;
        for (topic, links) in &self.bookmarks {
            let topic = match topic {
                BookmarkItem::Topic(topic) => format!("{}_{}", count, topic.name),
                _ => continue,
            };

            let links = links
                .iter()
                .filter_map(|item| match item {
                    BookmarkItem::Link(link) => Some(link.clone()),
                    _ => None,
                })
                .collect();

            data.insert(topic, links);
            count += 1;
        }

        let data = serde_json::to_string(&data).expect("Failed to serialize bookmarks");
        let path = format!("{}/{}", self.path, self.filename);
        fs::write(path, data).expect("Failed to write bookmarks to file");
    }
}
