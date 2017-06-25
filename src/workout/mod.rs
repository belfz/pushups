use std::cmp::Ordering;
use chrono::Duration;
use chrono::prelude::*;

static INVALID_DATE: &'static str = "invalid date";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Workout {
    date: String,
    repeats: u32,
}

impl Workout {
    pub fn new(date: String, repeats: u32) -> Workout {
        Workout { date, repeats }
    }

    pub fn get_repeats(&self) -> u32 {
        self.repeats
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }

    pub fn difference(&self, other: &Workout) -> Duration {
        DateTime::parse_from_rfc2822(&self.date).expect(&format!("{}: {}", INVALID_DATE, self.date))
            .signed_duration_since(
                DateTime::parse_from_rfc2822(&other.date).expect(&format!("{}: {}", INVALID_DATE, other.date))
            )
    }
}

impl Ord for Workout {
    fn cmp(&self, other: &Workout) -> Ordering {
        DateTime::parse_from_rfc2822(&self.date).expect(&format!("{}: {}", INVALID_DATE, self.date))
            .cmp(
                &DateTime::parse_from_rfc2822(&other.date).expect(&format!("{}: {}", INVALID_DATE, other.date))
            )
    }
}

impl PartialOrd for Workout {
    fn partial_cmp(&self, other: &Workout) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
