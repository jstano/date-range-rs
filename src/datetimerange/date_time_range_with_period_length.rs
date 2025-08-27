use crate::datetimerange::date_time_range::DateTimeRange;
use chrono::{Duration, NaiveDateTime, Timelike};
use std::hash::{Hash, Hasher};
use std::iter::Iterator;

const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = 1440;

#[derive(Debug, Clone)]
pub struct DateTimeRangeWithPeriodLength {
    date_time_range: DateTimeRange,
    period_length_minutes: i32,
}

impl DateTimeRangeWithPeriodLength {
    pub fn of(date_time_range: DateTimeRange, period_length_minutes: i32) -> Self {
        Self {
            date_time_range,
            period_length_minutes,
        }
    }

    pub fn of_datetimes(start: NaiveDateTime, end: NaiveDateTime, period_length_minutes: i32) -> Self {
        Self::of(DateTimeRange::of(start, end), period_length_minutes)
    }

    pub fn start_index(&self) -> i32 {
        let start = self.date_time_range.start();
        (start.hour() as i32 * MINUTES_PER_HOUR + start.minute() as i32) / self.period_length_minutes
    }

    pub fn end_index(&self) -> i32 {
        let start = self.date_time_range.start();
        let end = self.date_time_range.end();

        let mut end_index = end.hour() as i32 * MINUTES_PER_HOUR + end.minute() as i32;

        if end.date() > start.date() {
            end_index += MINUTES_PER_DAY;
        }

        end_index / self.period_length_minutes
    }

    pub fn period_length_in_minutes(&self) -> i32 {
        self.period_length_minutes
    }

    pub fn date_time_range(&self) -> &DateTimeRange {
        &self.date_time_range
    }

    pub fn index_range(&self) -> (i32, i32) {
        (self.start_index(), self.end_index() - 1)
    }

    pub fn number_of_periods_in_shift(&self) -> i32 {
        (self.date_time_range.duration().num_minutes() as i32) / self.period_length_minutes
    }
}

impl PartialEq for DateTimeRangeWithPeriodLength {
    fn eq(&self, other: &Self) -> bool {
        self.date_time_range == other.date_time_range
    }
}
impl Eq for DateTimeRangeWithPeriodLength {}

impl Hash for DateTimeRangeWithPeriodLength {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.date_time_range.hash(state);
    }
}

/// Iterator over the `NaiveDateTime`s in the range
pub struct DateTimeRangeIterator {
    current: NaiveDateTime,
    end: NaiveDateTime,
    step: Duration,
}

impl DateTimeRangeIterator {
    fn new(range: &DateTimeRange, period_minutes: i32) -> Self {
        Self {
            current: range.start(),
            end: range.end(),
            step: Duration::minutes(period_minutes as i64),
        }
    }
}

impl Iterator for DateTimeRangeIterator {
    type Item = NaiveDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            None
        } else {
            let result = self.current;
            self.current += self.step;
            Some(result)
        }
    }
}

/// Hook into Rust’s for-loops
impl IntoIterator for DateTimeRangeWithPeriodLength {
    type Item = NaiveDateTime;
    type IntoIter = DateTimeRangeIterator;

    fn into_iter(self) -> Self::IntoIter {
        DateTimeRangeIterator::new(&self.date_time_range, self.period_length_minutes)
    }
}
