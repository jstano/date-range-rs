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

#[cfg(test)]
mod tests {
    use super::DateTimeRange;
    use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
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
    fn of_constructs_and_duration_is_end_minus_start() {
        let start = dt(2023, 3, 10, 8, 30, 0);
        let end = dt(2023, 3, 10, 17, 0, 0);
        let r = DateTimeRange::of(start, end);
        assert_eq!(r.start(), start);
        assert_eq!(r.end(), end);
        assert_eq!(r.duration(), Duration::hours(8) + Duration::minutes(30));
    }

    #[test]
    fn from_time_range_on_date_same_day_and_cross_midnight() {
        // Same day: 09:00-17:00 on 2023-03-01
        let r1 = DateTimeRange::from_time_range_on_date(t(9, 0, 0), t(17, 0, 0), d(2023, 3, 1));
        assert_eq!(r1.start(), dt(2023, 3, 1, 9, 0, 0));
        assert_eq!(r1.end(), dt(2023, 3, 1, 17, 0, 0));
        assert_eq!(r1.duration(), Duration::hours(8));

        // Cross midnight: 22:00-06:00 should end on next day 06:00
        let r2 = DateTimeRange::from_time_range_on_date(t(22, 0, 0), t(6, 0, 0), d(2023, 3, 1));
        assert_eq!(r2.start(), dt(2023, 3, 1, 22, 0, 0));
        assert_eq!(r2.end(), dt(2023, 3, 2, 6, 0, 0));
        assert_eq!(r2.duration(), Duration::hours(8));
    }

    #[test]
    fn all_day_builds_midnight_to_next_midnight_24h() {
        let r = DateTimeRange::all_day(d(2024, 2, 29)); // leap day
        assert_eq!(r.start(), dt(2024, 2, 29, 0, 0, 0));
        assert_eq!(r.end(), dt(2024, 3, 1, 0, 0, 0));
        assert_eq!(r.duration(), Duration::hours(24));
    }

    #[test]
    fn contains_and_contains_exclusive_respect_boundaries() {
        let r = DateTimeRange::of(dt(2023, 1, 1, 9, 0, 0), dt(2023, 1, 1, 17, 0, 0));
        assert!(r.contains(dt(2023, 1, 1, 9, 0, 0))); // inclusive start
        assert!(r.contains(dt(2023, 1, 1, 17, 0, 0))); // inclusive end
        assert!(r.contains_exclusive(dt(2023, 1, 1, 12, 0, 0)));
        assert!(!r.contains_exclusive(dt(2023, 1, 1, 9, 0, 0)));
        assert!(!r.contains_exclusive(dt(2023, 1, 1, 17, 0, 0)));
    }

    #[test]
    fn overlaps_variants_and_overlap_range_duration() {
        let a = DateTimeRange::of(dt(2023, 5, 10, 9, 0, 0), dt(2023, 5, 10, 12, 0, 0));
        let b = DateTimeRange::of(dt(2023, 5, 10, 11, 0, 0), dt(2023, 5, 10, 13, 0, 0));
        let c_touch_right = DateTimeRange::of(dt(2023, 5, 10, 12, 0, 0), dt(2023, 5, 10, 14, 0, 0));
        let d_disjoint = DateTimeRange::of(dt(2023, 5, 10, 14, 0, 1), dt(2023, 5, 10, 15, 0, 0));

        // overlaps is inclusive
        assert!(a.overlaps(&b));
        assert!(a.overlaps(&c_touch_right)); // touch at 12:00
        assert!(!a.overlaps(&d_disjoint));

        // overlaps_exclusive requires strict
        assert!(a.overlaps_exclusive(&b));
        assert!(!a.overlaps_exclusive(&c_touch_right));

        // overlaps_completely
        let inner = DateTimeRange::of(dt(2023, 5, 10, 9, 30, 0), dt(2023, 5, 10, 11, 0, 0));
        assert!(a.overlaps_completely(&inner));
        assert!(!inner.overlaps_completely(&a));

        // overlap_range and overlap_duration
        let r = a.overlap_range(&b).unwrap();
        assert_eq!(r.start(), dt(2023, 5, 10, 11, 0, 0));
        assert_eq!(r.end(), dt(2023, 5, 10, 12, 0, 0));
        assert_eq!(a.overlap_duration(&b), Duration::hours(1));

        // touching -> zero duration but Some(range)
        let r2 = a.overlap_range(&c_touch_right).unwrap();
        assert_eq!(r2.start(), dt(2023, 5, 10, 12, 0, 0));
        assert_eq!(r2.end(), dt(2023, 5, 10, 12, 0, 0));
        assert_eq!(a.overlap_duration(&c_touch_right), Duration::zero());

        // disjoint -> None and zero duration
        assert!(a.overlap_range(&d_disjoint).is_none());
        assert_eq!(a.overlap_duration(&d_disjoint), Duration::zero());
    }

    #[test]
    fn ordering_and_equality_and_hash() {
        let a = DateTimeRange::of(dt(2023, 1, 1, 9, 0, 0), dt(2023, 1, 1, 10, 0, 0));
        let b = DateTimeRange::of(dt(2023, 1, 1, 8, 0, 0), dt(2023, 1, 1, 9, 30, 0));
        let c = DateTimeRange::of(dt(2023, 1, 1, 9, 0, 0), dt(2023, 1, 1, 10, 0, 0));

        // Eq compares bounds
        assert_eq!(a, c);
        assert_ne!(a, b);

        // Ord sorts by start then end
        let mut v = vec![a.clone(), b.clone()];
        v.sort();
        assert_eq!(v, vec![b, a.clone()]);

        // Hash consistent with Eq (basic smoke test: equal values hash the same)
        let mut ha = DefaultHasher::new();
        let mut hc = DefaultHasher::new();
        a.hash(&mut ha);
        c.hash(&mut hc);
        assert_eq!(ha.finish(), hc.finish());
    }
}
