use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct DateTimeRange {
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl DateTimeRange {
    pub fn of(start: NaiveDateTime, end: NaiveDateTime) -> Self {
        Self { start, end }
    }

    pub fn from_time_range_on_date(start_time: NaiveTime, end_time: NaiveTime, date: NaiveDate) -> Self {
        if end_time < start_time {
            Self {
                start: date.and_time(start_time),
                end: date.succ_opt().unwrap().and_time(end_time), // plusDays(1)
            }
        } else {
            Self {
                start: date.and_time(start_time),
                end: date.and_time(end_time),
            }
        }
    }

    pub fn all_day(date: NaiveDate) -> Self {
        Self {
            start: date.and_hms_opt(0, 0, 0).unwrap(),
            end: date.succ_opt().unwrap().and_hms_opt(0, 0, 0).unwrap(),
        }
    }

    pub fn start(&self) -> NaiveDateTime {
        self.start
    }

    pub fn end(&self) -> NaiveDateTime {
        self.end
    }

    pub fn duration(&self) -> Duration {
        self.end - self.start
    }

    pub fn overlaps(&self, other: &DateTimeRange) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    pub fn overlaps_exclusive(&self, other: &DateTimeRange) -> bool {
        self.start < other.end && self.end > other.start
    }

    pub fn overlaps_completely(&self, other: &DateTimeRange) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    pub fn overlap_duration(&self, other: &DateTimeRange) -> Duration {
        self.overlap_range(other)
            .map(|r| r.duration())
            .unwrap_or_else(|| Duration::milliseconds(0))
    }

    pub fn overlap_range(&self, other: &DateTimeRange) -> Option<DateTimeRange> {
        if !self.overlaps(other) {
            return None;
        }

        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        Some(Self::of(start, end))
    }

    pub fn contains(&self, dt: NaiveDateTime) -> bool {
        dt >= self.start && dt <= self.end
    }

    pub fn contains_exclusive(&self, dt: NaiveDateTime) -> bool {
        dt > self.start && dt < self.end
    }
}

impl PartialEq for DateTimeRange {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for DateTimeRange {}

impl Hash for DateTimeRange {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.end.hash(state);
    }
}

impl PartialOrd for DateTimeRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTimeRange {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.start.cmp(&other.start) {
            Ordering::Equal => self.end.cmp(&other.end),
            ord => ord,
        }
    }
}
