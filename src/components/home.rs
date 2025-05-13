use dioxus::prelude::*;

const _HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { "hello world" }
    }
}