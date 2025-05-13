use serde::{Serialize, Deserialize};

pub type TimelineEvents = Vec<TimelineEvent>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimelineData {
    pub time_period: TimePeriod,
    pub events: TimelineEvents,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimelineEvent {
    // TODO header image (string link or dioxus object)
    pub title: String,
    pub summary: String,
    pub link: String,
    pub time_period: TimePeriod,
    pub color: Option<String>, // TODO idk if i want to keep
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TimePeriod {
    OneYear(u32),
    YearRange {
        start: u32,
        end: u32,
    },
    OneDate(SimpleDate),
    DateRange {
        start: SimpleDate,
        end: SimpleDate,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SimpleDate {
    year: u32,
    month: u32,
    day: u32,
}

const TIMELINE_RON: &str = include_str!("timeline.ron");

pub fn parse_timeline() -> TimelineData {
    ron::from_str(TIMELINE_RON).unwrap()
}