use dioxus::prelude::*;

use crate::{components::timeline::Timeline, timeline_data::TIMELINE};

const _HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { "AP European History Timeline" }

        table {
            tr {
                th { "Color" }
                th { "Significance" }
            }
            tr {
                td {
                    color: "purple",
                    "Purple"
                }
                td {
                    "Major time period"
                }
            }
            tr {
                td {
                    color: "red",
                    "Red"
                }
                td {
                    "Turning point event"
                }
            }
            tr {
                td {
                    color: "orange",
                    "Orange"
                }
                td {
                    "Extremely significant (has a major lasting effect on history)"
                }
            }
            tr {
                td {
                    color: "yellow",
                    "Yellow"
                }
                td {
                    "Very significant (affects balance of power, politics, economic conditions, etc)"
                }
            }
            tr {
                td {
                    color: "green",
                    "Green"
                }
                td {
                    "Somewhat significant (notable, but not too many long-term effects)"
                }
            }
        }

        Timeline { data: TIMELINE.clone() }
    }
}