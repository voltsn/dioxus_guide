use crate::data::hacker_news::StoryItem;
use dioxus::prelude::*;

#[component]
pub fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
    let StoryItem {
        title,
        url,
        by,
        score,
        time,
        kids,
        ..
    } = &*story.read();

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!("{score} {}", if *score == 1 { " point" } else { " points" });

    let comments = format!(
        "{} {}",
        kids.len(),
        if kids.len() == 1 {
            " comment"
        } else {
            " comments"
        }
    );
    let time = time.format("%D %l:%M %p");

    rsx! {
        div { class: "p-4 relative",
            div { class: "text-2xl",
                a { class:"text-blue-600", href: url, "{title}" },
                a { class: "text-gray-400",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    "({hostname})"
                }
            }
            div { class: "flex text-gray-400",
                div { "{score}" }
                div { class: "pl-2", "by {by}" }
                div { class: "pl-2", "{time}" }
                div { class: "pl-2", "{comments}" }
            }
        }
    }
}
