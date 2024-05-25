use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TopicModel {
    pub name: String,
    // pub color: Color32,
}

impl TopicModel {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn empty() -> Self {
        Self::new(String::new())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LinkModel {
    pub title: String,
    pub url: String,
    pub preview: Option<String>,
}

impl LinkModel {
    pub fn new(title: String, url: String, preview: Option<String>) -> Self {
        Self {
            title,
            url,
            preview,
        }
    }

    // pub fn fetch_link(url: &str, ctx: &Context) -> Self {
    //     let request = Request::get(url);
    //     fetch(request, move |result: Result<Response>| {
    //         if let Ok(response) = result {
    //             let title = response.t
    //             let preview = response.text().unwrap_or_default();
    //             Self::new(title, url.to_string(), Some(preview))
    //         } else {
    //             Self::new(url.to_string(), url.to_string(), None)
    //         }
    //         ctx.request_repaint();
    //     });
    // }
}
