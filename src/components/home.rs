use dioxus::prelude::*;

use crate::{components::timeline::Timeline, timeline_data::TIMELINE};

const _HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { "AP European History Timeline" }

        Timeline { data: TIMELINE.clone() }
    }
}