mod components;
mod data;

use components::{Preview, Stories, StoryListing};
use data::hacker_news::StoryItem;
use dioxus::prelude::*;

fn main() {
    launch(App)
}

pub fn App() -> Element {
    rsx! {
        div { class: "flex w-full",
            div { class: "w-1/2", Stories {} }
            div { class: "w-1/2", Preview {} }
        }
    }
}
