use crate::data::hacker_news::Comment;
use dioxus::prelude::*;

#[component]
pub fn StoryComment(comment: Comment) -> Element {
    rsx! {
        div { padding: "0.5rem",
            div { color: "gray", "by {comment.by}" }
            div { dangerous_inner_html: "{comment.text}" }
            for kid in &comment.sub_comments {
                StoryComment { comment: kid.clone() }
            }
        }
    }
}
