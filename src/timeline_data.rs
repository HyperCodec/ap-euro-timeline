use std::cmp::Ordering;

use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

pub type TimelineEvents = Vec<TimelineEvent>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimelineData {
    pub time_period: TimePeriod,
    pub events: TimelineEvents,
}

impl Default for TimelineData {
    fn default() -> Self {
        TIMELINE.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimelineEvent {
    // TODO header image (string link or dioxus object)
    pub title: String,
    pub summary: String,
    pub link: String,
    pub time: EventTime,

    #[serde(default = "default_color")]
    pub color: String,
}

fn default_color() -> String {
    "green".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Timestamp {
    Year(u32),
    Date(SimpleDate),
}

impl Timestamp {
    pub fn date(&self) -> SimpleDate {
        match self {
            Self::Year(year) => SimpleDate { year: *year, month: 0, day: 0 },
            Self::Date(date) => date.clone(),
        }
    }

    pub fn year(&self) -> u32 {
        match self {
            Self::Year(year) => *year,
            Self::Date(date) => date.year,
        }
    }
}

impl ToString for Timestamp {
    fn to_string(&self) -> String {
        match self {
            Self::Year(year) => year.to_string(),
            Self::Date(date) => date.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimePeriod {
    pub start: Timestamp,
    pub end: Timestamp,
}

impl TimePeriod {
    pub fn contains(&self, date: &SimpleDate) -> bool {
        // the extra 5 boundary probably doesn't belong here 
        // since this is a time type, but it's only being used in one place.
        self.start.year() - 5 <= date.year && self.end.year() + 5 >= date.year
    }
    
    pub fn overlaps(&self, other: &Self) -> bool {
        let self_start = self.start.date();
        let self_end = self.end.date();

        let other_start = other.start.date();
        let other_end = other.end.date();

        self_start <= other_end && other_start <= self_end
    }

    pub fn years(&self) -> u32 {
        self.end.year() - self.start.year()
    }
}

impl ToString for TimePeriod {
    fn to_string(&self) -> String {
        format!("{} - {}", self.start.to_string(), self.end.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EventTime {
    Single(Timestamp),
    Period(TimePeriod),
}

impl EventTime {
    pub fn overlaps(&self, other: &Self) -> bool {
        match self {
            Self::Single(time) => match other {
                Self::Single(time2) => {
                    // TODO check text length or something instead
                    time.year().abs_diff(time2.year()) <= 10
                },
                Self::Period(period) => period.contains(&time.date()),
            },
            Self::Period(period) => match other {
                Self::Single(time) => period.contains(&time.date()),
                Self::Period(period2) => period.overlaps(period2),
            }
        }
    }

    pub fn start_date(&self) -> SimpleDate {
        match self {
            Self::Single(time) => time.date(),
            Self::Period(period) => period.start.date(),
        }
    }
    
    pub fn duration_years(&self) -> u32 {
        match self {
            Self::Single(_) => 0,
            Self::Period(period) => period.years(),
        }
    }
}

impl ToString for EventTime {
    fn to_string(&self) -> String {
        match self {
            Self::Single(single) => single.to_string(),
            Self::Period(period) => period.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord)]
pub struct SimpleDate {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl PartialOrd for SimpleDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let year_cmp = self.year.cmp(&other.year);
        if year_cmp != Ordering::Equal {
            return Some(year_cmp);
        }
        let month_cmp = self.month.cmp(&other.month);
        if month_cmp != Ordering::Equal {
            return Some(month_cmp)
        }
        let day_cmp = self.day.cmp(&other.day);
        Some(day_cmp)
    }
}

impl ToString for SimpleDate {
    fn to_string(&self) -> String {
        format!("{}/{}/{}", self.month, self.day, self.year)
    }
}

// probably should make an asset instead
// of including. however its a pretty small
// file so i dont think it matters much.
const TIMELINE_RON: &str = include_str!("timeline.ron");

lazy_static! {
    pub static ref TIMELINE: TimelineData = ron::from_str(TIMELINE_RON).unwrap();
}