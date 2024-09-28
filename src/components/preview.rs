use crate::components::StoryComment;
use crate::data::hacker_news::{Comment, StoryPageData};
use dioxus::prelude::*;

#[derive(Clone, Debug)]
pub enum PreviewState {
    Unset,
    Loading,
    Loaded(StoryPageData),
}

pub fn Preview() -> Element {
    let preview_state = PreviewState::Unset;
    match preview_state {
        PreviewState::Unset => rsx! {"Hover over a story to preview it here"},
        PreviewState::Loading => rsx! {"Loading..."},
        PreviewState::Loaded(story) => {
            rsx! {
                div { padding: "0.5rem",
                    div { font_size: "1.5rem", a { href: story.item.url, "{story.item.title}" } }
                    div { dangerous_inner_html: story.item.text }
                    for comment in &story.comments {
                        StoryComment { comment: comment.clone() }
                    }
                }
            }
        }
    }
}
