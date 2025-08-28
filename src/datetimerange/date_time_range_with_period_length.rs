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

#[cfg(test)]
mod tests {
    use super::DateTimeRangeWithPeriodLength as R;
    use crate::datetimerange::date_time_range::DateTimeRange;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).expect("invalid date")
    }
    fn t(h: u32, m: u32, s: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(h, m, s).expect("invalid time")
    }
    fn dt(y: i32, m: u32, day: u32, h: u32, mi: u32, s: u32) -> NaiveDateTime {
        d(y, m, day).and_time(t(h, mi, s))
    }

    #[test]
    fn constructors_and_accessors() {
        let start = dt(2023, 3, 10, 8, 15, 0);
        let end = dt(2023, 3, 10, 16, 45, 0);
        let r1 = R::of(DateTimeRange::of(start, end), 60);
        let r2 = R::of_datetimes(start, end, 60);
        assert_eq!(r1.period_length_in_minutes(), 60);
        assert_eq!(r1.date_time_range().start(), start);
        assert_eq!(r1.date_time_range().end(), end);
        // of_datetimes should build the same
        assert_eq!(r1, r2);
    }

    #[test]
    fn indices_same_day_and_index_range() {
        // 08:15..16:45 with 60-min periods
        let r = R::of_datetimes(dt(2023, 3, 1, 8, 15, 0), dt(2023, 3, 1, 16, 45, 0), 60);
        assert_eq!(r.start_index(), 8); // floor(495/60)
        assert_eq!(r.end_index(), 16);  // floor(1005/60)
        assert_eq!(r.index_range(), (8, 15)); // end_index - 1
        // number_of_periods uses duration minutes / 60 = 510/60 = 8
        assert_eq!(r.number_of_periods_in_shift(), 8);
    }

    #[test]
    fn indices_cross_midnight_and_counts() {
        // 22:00..06:00 next day, 30-min periods
        let r = R::of_datetimes(dt(2023, 3, 1, 22, 0, 0), dt(2023, 3, 2, 6, 0, 0), 30);
        assert_eq!(r.start_index(), 44); // 22*60/30
        assert_eq!(r.end_index(), 60);   // (24h+6h)*60/30 = 60
        assert_eq!(r.index_range(), (44, 59));
        assert_eq!(r.number_of_periods_in_shift(), 16); // 480/30
    }

    #[test]
    fn iterator_inclusive_when_aligned() {
        // 09:00..11:00, hourly -> points at 09,10,11
        let r = R::of_datetimes(dt(2023, 5, 1, 9, 0, 0), dt(2023, 5, 1, 11, 0, 0), 60);
        let v: Vec<_> = r.clone().into_iter().collect();
        assert_eq!(v, vec![
            dt(2023, 5, 1, 9, 0, 0),
            dt(2023, 5, 1, 10, 0, 0),
            dt(2023, 5, 1, 11, 0, 0),
        ]);
        // points = periods + 1
        assert_eq!(v.len() as i32, r.number_of_periods_in_shift() + 1);
    }

    #[test]
    fn iterator_stops_before_passing_end_when_not_aligned() {
        // 09:00..10:45, step 30 -> last point 10:30 (11:00 would exceed end)
        let r = R::of_datetimes(dt(2023, 5, 1, 9, 0, 0), dt(2023, 5, 1, 10, 45, 0), 30);
        let v: Vec<_> = r.clone().into_iter().collect();
        assert_eq!(v, vec![
            dt(2023, 5, 1, 9, 0, 0),
            dt(2023, 5, 1, 9, 30, 0),
            dt(2023, 5, 1, 10, 0, 0),
            dt(2023, 5, 1, 10, 30, 0),
        ]);
        // periods = floor(105/30) = 3; points = periods + 1 = 4
        assert_eq!(r.number_of_periods_in_shift(), 3);
        assert_eq!(v.len(), 4);
    }

    #[test]
    fn iterator_cross_midnight_sequence() {
        // 22:00..01:00 next day, 60-min -> 22,23,00,01
        let r = R::of_datetimes(dt(2023, 6, 1, 22, 0, 0), dt(2023, 6, 2, 1, 0, 0), 60);
        let v: Vec<_> = r.clone().into_iter().collect();
        assert_eq!(v, vec![
            dt(2023, 6, 1, 22, 0, 0),
            dt(2023, 6, 1, 23, 0, 0),
            dt(2023, 6, 2, 0, 0, 0),
            dt(2023, 6, 2, 1, 0, 0),
        ]);
        assert_eq!(r.number_of_periods_in_shift(), 3);
        assert_eq!(v.len() as i32, r.number_of_periods_in_shift() + 1);
    }

    #[test]
    fn equality_and_hash_ignore_period_length() {
        let start = dt(2023, 7, 7, 7, 0, 0);
        let end = dt(2023, 7, 7, 9, 0, 0);
        let a = R::of_datetimes(start, end, 15);
        let b = R::of_datetimes(start, end, 30);
        assert_eq!(a, b);

        let mut ha = DefaultHasher::new();
        let mut hb = DefaultHasher::new();
        a.hash(&mut ha);
        b.hash(&mut hb);
        assert_eq!(ha.finish(), hb.finish());
    }
}
