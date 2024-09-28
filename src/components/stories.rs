use super::StoryListing;
use crate::data::hacker_news::StoryItem;
use dioxus::prelude::*;

#[component]
pub fn Stories() -> Element {
    rsx! {
        StoryListing {
            story: StoryItem {
                id: 0,
                title: "hello hackernews".to_string(),
                url: None,
                text: None,
                by: "Author".to_string(),
                score: 0,
                descendants: 0,
                time: chrono::Utc::now(),
                kids: vec![],
                r#type: "".to_string(),
            }
        }
    }
}
