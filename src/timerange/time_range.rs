use chrono::{Duration, NaiveTime};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct TimeRange {
    start: NaiveTime,
    end: NaiveTime,
}

impl TimeRange {
    pub fn of(start: NaiveTime, end: NaiveTime) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> NaiveTime {
        self.start
    }

    pub fn end(&self) -> NaiveTime {
        self.end
    }

    pub fn duration(&self) -> Duration {
        self.end - self.start
    }

    pub fn overlaps(&self, other: &TimeRange) -> bool {
        let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

        if self.end == midnight && other.end == midnight {
            return true;
        }

        if self.end == midnight {
            return other.end >= self.start;
        }

        if other.end == midnight {
            return other.start <= self.end;
        }

        self.start <= other.end && self.end >= other.start
    }
}

impl PartialEq for TimeRange {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for TimeRange {}

impl Hash for TimeRange {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.end.hash(state);
    }
}

impl PartialOrd for TimeRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeRange {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.start.cmp(&other.start) {
            Ordering::Equal => self.end.cmp(&other.end),
            ord => ord,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use std::collections::HashSet;

    fn t(h: u32, m: u32, s: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(h, m, s).unwrap()
    }

    #[test]
    fn of_and_accessors_work() {
        let tr = TimeRange::of(t(9, 0, 0), t(10, 30, 0));
        assert_eq!(tr.start(), t(9, 0, 0));
        assert_eq!(tr.end(), t(10, 30, 0));
    }

    #[test]
    fn duration_is_computed_correctly() {
        let tr = TimeRange::of(t(1, 0, 0), t(3, 30, 0));
        assert_eq!(tr.duration(), Duration::minutes(150));
    }

    #[test]
    fn overlaps_basic_true_when_intervals_intersect() {
        let a = TimeRange::of(t(9, 0, 0), t(12, 0, 0));
        let b = TimeRange::of(t(11, 0, 0), t(13, 0, 0));
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn overlaps_true_when_touching_at_edge_inclusive() {
        let a = TimeRange::of(t(9, 0, 0), t(10, 0, 0));
        let b = TimeRange::of(t(10, 0, 0), t(11, 0, 0));
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn overlaps_false_when_disjoint() {
        let a = TimeRange::of(t(9, 0, 0), t(10, 0, 0));
        let b = TimeRange::of(t(10, 1, 0), t(11, 0, 0));
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn overlaps_midnight_special_both_end_at_midnight_true() {
        let a = TimeRange::of(t(22, 0, 0), t(0, 0, 0));
        let b = TimeRange::of(t(1, 0, 0), t(0, 0, 0));
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn overlaps_self_ends_at_midnight_depends_on_other_end() {
        // self ends at midnight: overlap if other.end >= self.start
        let self_tr = TimeRange::of(t(22, 0, 0), t(0, 0, 0));
        let other_false = TimeRange::of(t(19, 0, 0), t(21, 59, 59));
        let other_true = TimeRange::of(t(19, 0, 0), t(22, 0, 0)); // equal to start is allowed

        assert!(!self_tr.overlaps(&other_false));
        assert!(self_tr.overlaps(&other_true));
    }

    #[test]
    fn overlaps_other_ends_at_midnight_depends_on_other_start() {
        // other ends at midnight: overlap if other.start <= self.end
        let other_tr = TimeRange::of(t(22, 0, 0), t(0, 0, 0));
        let self_false = TimeRange::of(t(20, 0, 0), t(21, 59, 59));
        let self_true = TimeRange::of(t(20, 0, 0), t(22, 0, 0)); // equal to other.start is allowed

        assert!(!self_false.overlaps(&other_tr));
        assert!(self_true.overlaps(&other_tr));
    }

    #[test]
    fn equality_and_hash_consistency() {
        let a1 = TimeRange::of(t(9, 0, 0), t(10, 0, 0));
        let a2 = TimeRange::of(t(9, 0, 0), t(10, 0, 0));
        let b = TimeRange::of(t(9, 0, 0), t(11, 0, 0));

        assert_eq!(a1, a2);
        assert_ne!(a1, b);

        let mut set = HashSet::new();
        set.insert(a1.clone());
        assert!(set.contains(&a2));
        assert!(!set.contains(&b));
    }

    #[test]
    fn ordering_is_by_start_then_end() {
        let mut v = vec![
            TimeRange::of(t(9, 0, 0), t(10, 0, 0)),
            TimeRange::of(t(8, 0, 0), t(9, 0, 0)),
            TimeRange::of(t(9, 0, 0), t(9, 30, 0)),
            TimeRange::of(t(8, 0, 0), t(8, 30, 0)),
        ];
        v.sort();

        let starts_ends: Vec<(NaiveTime, NaiveTime)> = v.iter().map(|tr| (tr.start(), tr.end())).collect();

        assert_eq!(
            starts_ends,
            vec![
                (t(8, 0, 0), t(8, 30, 0)), // same start 8:00, end 8:30 before 9:00
                (t(8, 0, 0), t(9, 0, 0)),
                (t(9, 0, 0), t(9, 30, 0)), // same start 9:00, end 9:30 before 10:00
                (t(9, 0, 0), t(10, 0, 0)),
            ]
        );
    }
}
