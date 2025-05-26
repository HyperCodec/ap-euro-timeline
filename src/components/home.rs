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
                    "Very significant (has a lasting effect on history)"
                }
            }
            tr {
                td {
                    color: "yellow",
                    "Yellow"
                }
                td {
                    "Somewhat significant (balance of power, politics, etc)"
                }
            }
            tr {
                td {
                    color: "green",
                    "Green"
                }
                td {
                    "Not very significant (notable, but not too many long-term effects)"
                }
            }
        }

        Timeline { data: TIMELINE.clone() }
    }
}