use std::sync::{Arc, RwLock};

use dioxus::prelude::*;

use crate::timeline_data::{EventTime, SimpleDate, TimePeriod, TimelineData, TimelineEvent, Timestamp};

// TODO maybe turn these into args of `Timeline`.
const PIXELS_PER_YEAR: f32 = 5.0;
const PIXELS_PER_MONTH: f32 = PIXELS_PER_YEAR as f32 / 12.0;
const PIXELS_PER_DAY: f32 = PIXELS_PER_MONTH / 30.0;

const HEIGHT: u32 = 250;
const SIDE_MARGIN: f32 = 25.0;
const YEAR_TOP_MARGIN: u32 = 5;
const EVENTS_BOTTOM_MARGIN: u32 = 5;
const EVENTS_TEXT_MARGIN: u32 = 3;
const EVENTS_SEPARATION_MARGIN: u32 = 20;

const END_RADIUS: u32 = 30;
const END_YEAR_Y: u32 = CENTER_Y + END_RADIUS + YEAR_TOP_MARGIN;

const NORMAL_RADIUS: u32 = 20;
const CENTURY_Y: u32 = CENTER_Y + NORMAL_RADIUS + YEAR_TOP_MARGIN;

const SMALL_RADIUS: u32 = 10;

const CENTER_Y: u32 = HEIGHT / 2;
const EVENTS_Y: u32 = CENTER_Y - END_RADIUS - EVENTS_BOTTOM_MARGIN;
const POINT_EVENT_RADIUS: u32 = 5;

const EVENT_INFO_WIDTH: u32 = 50;
const EVENT_INFO_HEIGHT: u32 = 100;


#[component]
pub fn Timeline(data: TimelineData) -> Element {
    let (start, end) = (data.time_period.start.year(), data.time_period.end.year());
    let years = end - start;

    let w = years as f32 * PIXELS_PER_YEAR + SIDE_MARGIN * 2.0;

    let mut stacking = EventStackingManager::new();
    let mut events: Vec<(TimelineEvent, u32, Signal<bool>)> = Vec::with_capacity(data.events.len());

    // O(n^2) bad but idrc since it's only running
    // once per page load. not worth optimizing.
    for event in data.events {
        let overlaps = stacking.register(event.time.clone());
        let y = EVENTS_Y - overlaps * EVENTS_SEPARATION_MARGIN;
        let is_hovered = use_signal(|| false);
        events.push((event, y, is_hovered));
    }

    rsx! {
        div {
            svg {
                id: "timeline",
                width: w,
                height: HEIGHT,

                TimelineBar { start, end, w }

                Events {
                    start,
                    events: events.clone(),
                }
            }

            EventInfos {
                start,
                events,
            }
        }
    }
}

#[component]
fn TimelineBar(start: u32, end: u32, w: f32) -> Element {
    let right_bound = w - SIDE_MARGIN;
    
    rsx! {
        g {
            class: "timeline_bar",

            line {
                class: "timeline_base",
                x1: SIDE_MARGIN,
                y1: CENTER_Y,
                x2: right_bound,
                y2: CENTER_Y,
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
}

#[component]
fn EventInfos(start: u32, events: Vec<(TimelineEvent, u32, Signal<bool>)>) -> Element {
    rsx! {
        for (data, y, is_hovered) in events {
            EventInfo {
                cx: x_from_date(start, &data.time.start_date()),
                bottom: y,
                data,
                is_hovered,
            }
        }
    }
}

struct EventStackingManager {
    already_placed: Vec<EventTime>,
}

impl EventStackingManager {
    fn new() -> Self {
        Self {
            already_placed: Vec::new()
        }
    }

    // registers and counts overlaps
    fn register(&mut self, time: EventTime) -> u32 {
        let mut overlaps = 0;
        for time2 in &self.already_placed {
            if time2.overlaps(&time) {
                overlaps += 1;
            }
        }

        self.already_placed.push(time);

        overlaps
    }
}

#[component]
fn Events(start: u32, events: Vec<(TimelineEvent, u32, Signal<bool>)>) -> Element {
    rsx! {
        for (data, y, is_hovered) in events {
            Event { start, data, y, is_hovered }
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
            y1: CENTER_Y - radius,
            x2: x,
            y2: CENTER_Y + radius,
        }
    }
}

#[component]
fn Event(start: u32, y: u32, data: TimelineEvent, is_hovered: Signal<bool>) -> Element {
    match data.time.clone() {
        EventTime::Single(time) => rsx! { SingleEvent { start, y, data, time, is_hovered } },
        EventTime::Period(period) => rsx! { RangeEvent { start, y, data, period, is_hovered } },
    }
}

#[component]
fn SingleEvent(start: u32, y: u32, data: TimelineEvent, time: Timestamp, is_hovered: Signal<bool>) -> Element {
    let x = x_from_date(start, &time.date());

    rsx! {
        PointEvent { x, y, data: data.clone(), is_hovered }
    }
}

#[component]
fn PointEvent(x: u32, y: u32, data: TimelineEvent, mut is_hovered: Signal<bool>) -> Element {
    rsx! {
        g {
            class: "timeline_event",
            onclick: move |_| {
                // idk another way of doing this. the a tag doesn't seem to work.
                document::eval(&format!("window.open(\"{}\", \"_blank\");", data.link));
            },
            onmouseenter: move |_| {
                is_hovered.set(true);
            },
            onmouseleave: move |_| {
                is_hovered.set(false);
            },
            
            circle {
                cx: x,
                cy: y,

                r: POINT_EVENT_RADIUS,

                fill: data.color,
            }

            text {
                x,
                y: y - POINT_EVENT_RADIUS - EVENTS_TEXT_MARGIN,

                text_anchor: "middle",

                "{data.title}"
            }
        }
    }
}

#[component]
fn RangeEvent(start: u32, y: u32, data: TimelineEvent, period: TimePeriod, is_hovered: Signal<bool>) -> Element {
    let (x1, x2) = (x_from_date(start, &period.start.date()), x_from_date(start, &period.end.date()));

    rsx! {
        LineEvent {
            start,
            x1,
            x2,
            y,
            data,
            is_hovered,
        }
    }
}

#[component]
fn LineEvent(start: u32, x1: u32, x2: u32, y: u32, data: TimelineEvent, mut is_hovered: Signal<bool>) -> Element {
    rsx! {
        g {
            class: "timeline_event",
            onclick: move |_| {
                // idk another way of doing this. the a tag doesn't seem to work.
                document::eval(&format!("window.open(\"{}\", \"_blank\");", data.link));
            },
            onmouseenter: move |_| {
                is_hovered.set(true);
            },
            onmouseleave: move |_| {
                is_hovered.set(false);
            },

            line {
                x1,
                y1: y,
                x2,
                y2: y,

                stroke_width: 4,

                stroke: data.color,
            }

            text {
                x: x1,
                y: EVENTS_Y - 2 - EVENTS_TEXT_MARGIN, 

                text_anchor: "start",

                "{data.title}"
            }
        }
    }
}

#[component]
fn EventInfo(cx: u32, bottom: u32, data: TimelineEvent, is_hovered: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "event_info",
            class: if *is_hovered.read() { "visible" },
            position: "absolute",

            right: cx + EVENT_INFO_WIDTH / 2,
            bottom,

            width: EVENT_INFO_WIDTH,
            height: EVENT_INFO_HEIGHT,
            
            a {
                href: "{data.link}",
                target: "_blank",
                h3 { "{data.title}" }
            }
            p { "{data.summary}" }

            // TODO figure out the element for this
            p { "{data.time.to_string()}" }
        }
    }
}

fn ceil_neareset_ten(number: u32) -> u32 {
    ((number as f32 / 10.0).ceil() * 10.0) as u32
}

fn x_from_date(start: u32, date: &SimpleDate) -> u32 {
    let delta = date.year - start;
    (SIDE_MARGIN + 
        delta as f32 * PIXELS_PER_YEAR +
        date.month as f32 * PIXELS_PER_MONTH +
        date.day as f32 * PIXELS_PER_DAY
    ).round() as u32
}

fn x_from_year(start: u32, year: u32) -> u32 {
    let delta = year - start;
    (SIDE_MARGIN + delta as f32 * PIXELS_PER_YEAR).round() as u32
}