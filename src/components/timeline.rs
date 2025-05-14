use dioxus::prelude::*;

use crate::timeline_data::{TimePeriod, TimelineData, TimelineEvent};

// https://www.geeksforgeeks.org/create-a-timeline-using-javascript/
#[component]
pub fn Timeline(events: TimelineData) -> Element {
    rsx! {
        ul {
            id: "timeline",
            class: "timeline",
        
            // TODO
        }
    }
}

#[component]
fn Event(data: TimelineEvent) -> Element {
    todo!()
}

#[component]
fn SingleEvent(data: TimelineEvent) -> Element {
    todo!()
}

#[component]
fn RangeEvent(data: TimelineEvent) -> Element {
    todo!()
}

#[component]
fn EventInfo(data: TimelineEvent) -> Element {
    rsx! {
        div {
            class: "card",
            
            a {
                href: "{data.link}",
                target: "_blank",
                h3 { "{data.title}" }
            }
            p { "{data.summary}" }

            // TODO figure out element for this
            p { "{format_time_period(&data.time_period)}" }
        }
    }
}

fn format_time_period(period: &TimePeriod) -> String {
    match period {
        TimePeriod::OneDate(date) => date.to_string(),
        TimePeriod::OneYear(year) => year.to_string(),
        TimePeriod::DateRange { start, end } => format!("{}-{}", start.to_string(), end.to_string()),
        TimePeriod::YearRange { start, end } => format!("{start}-{end}"),
    }
}