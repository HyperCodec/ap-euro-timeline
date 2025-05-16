use dioxus::prelude::*;

use crate::timeline_data::{TimePeriod, TimelineData, TimelineEvent};

const PIXELS_PER_YEAR: u32 = 5;
const HEIGHT: u32 = 250;
const SIDE_MARGIN: u32 = 25;
const YEAR_TOP_MARGIN: u32 = 5;

const END_RADIUS: u32 = 30;
const END_YEAR_Y: u32 = CENTER + END_RADIUS + YEAR_TOP_MARGIN;

const NORMAL_RADIUS: u32 = 20;
const CENTURY_Y: u32 = CENTER + NORMAL_RADIUS + YEAR_TOP_MARGIN;

const SMALL_RADIUS: u32 = 10;

const CENTER: u32 = HEIGHT / 2;


#[component]
pub fn Timeline(data: TimelineData) -> Element {
    let (start, end, years) = if let TimePeriod::YearRange { start, end } = data.time_period {
        (start, end, end - start)
    } else {
        unreachable!()
    };

    let w = years * PIXELS_PER_YEAR + SIDE_MARGIN * 2;

    rsx! {
        svg {
            id: "timeline",
            width: w,
            height: HEIGHT,

            TimelineBar { start, end, w }
        }
    }
}

#[component]
fn TimelineBar(start: u32, end: u32, w: u32) -> Element {
    let right_bound = w - SIDE_MARGIN;
    
    rsx! {
        line {
                class: "timeline_base",
                x1: SIDE_MARGIN,
                y1: CENTER,
                x2: right_bound,
                y2: CENTER,
            }

            Tick { start, year: start, radius: END_RADIUS }
            Tick { start, year: end, radius: END_RADIUS }

            for year in (ceil_neareset_ten(start+1)..end).step_by(10) {
                if year % 100 == 0 {
                    Tick { start, year, radius: NORMAL_RADIUS }
                    text {
                        class: "timeline_century_text",
                        x: x_from_year(start, year),
                        y: CENTURY_Y,

                        dominant_baseline: "hanging",
                        text_anchor: "middle",

                        "{year.to_string()}"
                    }
                } else {
                    Tick { start, year, radius: SMALL_RADIUS }
                }
            }

            text {
                class: "timeline_end_text",
                x: SIDE_MARGIN,
                y: END_YEAR_Y,

                dominant_baseline: "hanging",
                text_anchor: "middle",

                "{start.to_string()}"
            }

            text {
                class: "timeline_end_text",
                x: right_bound,
                y: END_YEAR_Y,

                dominant_baseline: "hanging",
                text_anchor: "middle",

                "{end.to_string()}"
            }
    }
}

#[component]
fn Tick(start: u32, year: u32, radius: u32) -> Element {
    let x = x_from_year(start, year);

    rsx! {
        line {
            class: "timeline_tick",

            x1: x,
            y1: CENTER - radius,
            x2: x,
            y2: CENTER + radius,
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
fn ceil_neareset_ten(number: u32) -> u32 {
    ((number as f32 / 10.0).ceil() * 10.0) as u32
}

fn x_from_year(start: u32, year: u32) -> u32 {
    let delta = year - start;
    SIDE_MARGIN + delta * PIXELS_PER_YEAR
}