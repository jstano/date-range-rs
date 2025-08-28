use chrono::{Datelike, Duration, NaiveDate, Weekday};
use std::cmp::Ordering;

/// Represents a range of dates.
#[derive(Copy, Clone, Debug)]
pub struct DateRange {
    start_date: NaiveDate,
    end_date: NaiveDate,
    len: usize,
    prior_fn: Option<fn(&DateRange) -> DateRange>,
    next_fn: Option<fn(&DateRange) -> DateRange>,
    start_day: Option<usize>,
}

impl DateRange {
    pub fn new(start_date: NaiveDate, end_date: NaiveDate) -> DateRange {
        let days = (end_date - start_date).num_days() as usize + 1;
        Self {
            start_date,
            end_date,
            len: days,
            prior_fn: None,
            next_fn: None,
            start_day: None,
        }
    }

    pub(crate) fn new_with_prior_next(
        start_date: NaiveDate,
        end_date: NaiveDate,
        prior_fn: fn(&DateRange) -> DateRange,
        next_fn: fn(&DateRange) -> DateRange,
    ) -> DateRange {
        let days = (end_date - start_date).num_days() as usize + 1;
        Self {
            start_date,
            end_date,
            len: days,
            prior_fn: Some(prior_fn),
            next_fn: Some(next_fn),
            start_day: None,
        }
    }

    pub(crate) fn new_with_prior_next_start_day(
        start_date: NaiveDate,
        end_date: NaiveDate,
        prior_fn: fn(&DateRange) -> DateRange,
        next_fn: fn(&DateRange) -> DateRange,
        start_day: Option<usize>,
    ) -> DateRange {
        let days = (end_date - start_date).num_days() as usize + 1;
        Self {
            start_date,
            end_date,
            len: days,
            prior_fn: Some(prior_fn),
            next_fn: Some(next_fn),
            start_day,
        }
    }

    /// Get the starting date in the range.
    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    /// Get the ending date in the range.
    pub fn end_date(&self) -> NaiveDate {
        self.end_date
    }

    /// Get the number of days in the range.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Get an iterator over the dates in the range.
    pub fn iter(&self) -> DateRangeIter {
        DateRangeIter {
            current: self.start_date,
            end: self.end_date,
        }
    }

    /// Get the optional start day of the range.
    pub fn start_day(&self) -> Option<usize> {
        self.start_day
    }

    /// Get the dates contained in the range in a vec.
    pub fn dates(&self) -> Vec<NaiveDate> {
        let mut dates = Vec::with_capacity(self.len());
        let mut current = self.start_date();
        while current <= self.end_date() {
            dates.push(current);
            current += Duration::days(1);
        }
        dates
    }

    /// Get the date at the specified index. If the index is outside the bounds
    //  an error will be returned.
    pub fn date_at(&self, index: usize) -> Option<NaiveDate> {
        self.dates().get(index).copied()
    }

    /// Get a list of dates from the range that match the specified DayOfWeek
    pub fn dates_for_day(&self, day: Weekday) -> Vec<NaiveDate> {
        self.dates()
            .into_iter()
            .filter(|d| d.weekday() == day)
            .collect()
    }

    /// Check if a date is contained in the range.
    pub fn contains_date(&self, date: NaiveDate) -> bool {
        date >= self.start_date() && date <= self.end_date()
    }

    /// Check if a date range is fully contained in the range.
    pub fn contains_range(&self, date_range: &DateRange) -> bool {
        date_range.start_date() >= self.start_date() && date_range.end_date() <= self.end_date()
    }

    /// Check if a date range is partially contained in the range.
    pub fn overlaps(&self, date_range: &DateRange) -> bool {
        self.start_date() <= date_range.end_date() && self.end_date() >= date_range.start_date()
    }

    /// Check if a date range is partially contained in a list of date ranges.
    pub fn overlaps_any(&self, date_ranges: &[DateRange]) -> bool {
        date_ranges.iter().any(|range| self.overlaps(range))
    }

    /// Get the DateRange that contains the specified date.
    pub fn range_containing_date(&self, date: NaiveDate) -> DateRange {
        let mut range = self.create_new_date_range(self.start_date(), self.end_date());
        while !range.contains_date(date) {
            if date > range.end_date() {
                range = range.next();
            } else {
                range = range.prior();
            }
        }
        range
    }

    /// Get a DateRange that represents the prior range to this dateRange.
    pub fn prior(&self) -> DateRange {
        if self.prior_fn.is_some() {
            self.prior_fn.unwrap()(self)
        } else {
            self.create_new_date_range(
                self.start_date() - Duration::days(self.len() as i64),
                self.end_date() - Duration::days(self.len() as i64),
            )
        }
    }

    /// Get the DateRange that represents a date range that is N ranges prior to the current DateRange.
    pub fn prior_n(&self, number: usize) -> DateRange {
        let mut range = self.prior();
        for _ in 1..number {
            range = range.prior();
        }
        range
    }

    /// Get a DateRange that represents the next range to this dateRange.
    pub fn next(&self) -> DateRange {
        if self.next_fn.is_some() {
            self.next_fn.unwrap()(self)
        } else {
            self.create_new_date_range(
                self.start_date() + Duration::days(self.len() as i64),
                self.end_date() + Duration::days(self.len() as i64),
            )
        }
    }

    /// Get the DateRange that represents a date range that is N ranges after the current DateRange.
    pub fn next_n(&self, number: usize) -> DateRange {
        let mut range = self.next();
        for _ in 1..number {
            range = range.next();
        }
        range
    }

    // Get a list of N DateRanges before this DateRange, not including this DateRange.
    pub fn ranges_before(&self, number: usize) -> Vec<DateRange> {
        self.ranges_before_impl(number, false)
    }

    // Get a list of N DateRanges before this DateRange, including this DateRange.
    pub fn ranges_before_inclusive(&self, number: usize) -> Vec<DateRange> {
        self.ranges_before_impl(number, true)
    }

    // Get a list of N DateRanges after this DateRange, not including this DateRange.
    pub fn ranges_after(&self, number: usize) -> Vec<DateRange> {
        self.ranges_after_impl(number, false)
    }

    // Get a list of N DateRanges after this DateRange, including this DateRange.
    pub fn ranges_after_inclusive(&self, number: usize) -> Vec<DateRange> {
        self.ranges_after_impl(number, true)
    }

    // Get a list of DateRanges that includes the current DateRange, N DateRanges before
    // this DateRange and N DateRanges after this date range.
    pub fn ranges_window(&self, before: usize, after: usize) -> Vec<DateRange> {
        let mut ranges = Vec::with_capacity(before + after + 1);

        // Add prior ranges
        ranges.extend(self.ranges_before_impl(before, true)); // includes self

        // Add after ranges, skip self since already included
        let mut after_ranges = self.ranges_after_impl(after, false);
        ranges.append(&mut after_ranges);

        ranges
    }

    /// Get a list of DateRanges that contain the specified dates.
    pub fn ranges_containing_span(
        &self,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Vec<DateRange> {
        // assert!(from != null);
        // assert!(from <= to);
        // assert!(from <= to);

        let mut ranges = Vec::new();
        let mut range = self.range_containing_date(from_date);
        ranges.push(range.clone());

        while range.end_date() < to_date {
            range = range.next();
            ranges.push(range.clone());
        }

        ranges.sort_by_key(|r| r.start_date());
        ranges
    }

    fn ranges_before_impl(&self, number: usize, include_self: bool) -> Vec<DateRange> {
        let mut ranges = Vec::with_capacity(number + 1);
        if include_self {
            ranges.push(self.create_new_date_range(self.start_date(), self.end_date()));
        }
        let mut current = self.create_new_date_range(self.start_date(), self.end_date());
        for _ in 0..number {
            current = current.prior();
            ranges.push(current.create_new_date_range(current.start_date(), current.end_date()));
        }
        ranges.reverse(); // to match Java order
        ranges
    }

    fn ranges_after_impl(&self, number: usize, include_self: bool) -> Vec<DateRange> {
        let mut ranges = Vec::with_capacity(number + 1);
        if include_self {
            ranges.push(self.create_new_date_range(self.start_date(), self.end_date()));
        }
        let mut current = self.create_new_date_range(self.start_date(), self.end_date());
        for _ in 0..number {
            current = current.next();
            ranges.push(current.create_new_date_range(current.start_date(), current.end_date()));
        }
        ranges
    }

    fn create_new_date_range(&self, start: NaiveDate, end: NaiveDate) -> DateRange {
        Self {
            start_date: start,
            end_date: end,
            len: (end - start).num_days() as usize + 1,
            prior_fn: self.prior_fn.clone(),
            next_fn: self.next_fn.clone(),
            start_day: self.start_day.clone(),
        }
    }
}

impl PartialEq for DateRange {
    fn eq(&self, other: &Self) -> bool {
        self.start_date == other.start_date && self.end_date == other.end_date
    }
}

impl Eq for DateRange {}

impl PartialOrd for DateRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_date.cmp(&other.start_date)
    }
}

pub struct DateRangeIter {
    current: NaiveDate,
    end: NaiveDate,
}

impl Iterator for DateRangeIter {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            None
        } else {
            let result = self.current;
            self.current += Duration::days(1);
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DateRange;
    use chrono::{NaiveDate, Weekday};

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).expect("invalid date")
    }

    // Custom prior/next that shift by exactly 1 day rather than by len()
    fn prior_shift_by_one(dr: &DateRange) -> DateRange {
        DateRange::new_with_prior_next_start_day(
            dr.start_date() - chrono::Duration::days(1),
            dr.end_date() - chrono::Duration::days(1),
            prior_shift_by_one,
            next_shift_by_one,
            dr.start_day(),
        )
    }
    fn next_shift_by_one(dr: &DateRange) -> DateRange {
        DateRange::new_with_prior_next_start_day(
            dr.start_date() + chrono::Duration::days(1),
            dr.end_date() + chrono::Duration::days(1),
            prior_shift_by_one,
            next_shift_by_one,
            dr.start_day(),
        )
    }

    #[test]
    fn new_and_accessors_and_len_inclusive() {
        let r = DateRange::new(d(2023, 1, 1), d(2023, 1, 7));
        assert_eq!(r.start_date(), d(2023, 1, 1));
        assert_eq!(r.end_date(), d(2023, 1, 7));
        assert_eq!(r.len(), 7); // inclusive
    }

    #[test]
    fn iter_and_dates_and_date_at_bounds() {
        let r = DateRange::new(d(2023, 3, 28), d(2023, 4, 2)); // 6 days inclusive
        let v = r.dates();
        assert_eq!(v.len(), r.len());
        assert_eq!(v.first().copied(), Some(d(2023, 3, 28)));
        assert_eq!(v.last().copied(), Some(d(2023, 4, 2)));

        // iterator yields same sequence
        let it: Vec<_> = r.iter().collect();
        assert_eq!(it, v);

        // date_at within and out-of-bounds
        assert_eq!(r.date_at(0), Some(d(2023, 3, 28)));
        assert_eq!(r.date_at(r.len() - 1), Some(d(2023, 4, 2)));
        assert_eq!(r.date_at(r.len()), None);
    }

    #[test]
    fn dates_for_day_filters_correct_weekdays() {
        // Week spanning Mon..Sun
        let r = DateRange::new(d(2023, 5, 1), d(2023, 5, 7));
        let mondays = r.dates_for_day(Weekday::Mon);
        assert_eq!(mondays, vec![d(2023, 5, 1)]);
        let sunday = r.dates_for_day(Weekday::Sun);
        assert_eq!(sunday, vec![d(2023, 5, 7)]);
    }

    #[test]
    fn predicates_contains_overlaps() {
        let r = DateRange::new(d(2023, 1, 10), d(2023, 1, 20));
        assert!(r.contains_date(d(2023, 1, 10)));
        assert!(r.contains_date(d(2023, 1, 15)));
        assert!(r.contains_date(d(2023, 1, 20)));
        assert!(!r.contains_date(d(2023, 1, 9)));
        assert!(!r.contains_date(d(2023, 1, 21)));

        // contained range
        let inner = DateRange::new(d(2023, 1, 12), d(2023, 1, 18));
        assert!(r.contains_range(&inner));

        // overlapping ranges
        let left_overlap = DateRange::new(d(2023, 1, 5), d(2023, 1, 12));
        let right_overlap = DateRange::new(d(2023, 1, 18), d(2023, 1, 25));
        let non_overlap = DateRange::new(d(2023, 1, 21), d(2023, 1, 25));
        assert!(r.overlaps(&left_overlap));
        assert!(r.overlaps(&right_overlap));
        assert!(!r.overlaps(&non_overlap));

        // overlaps_any
        let many = vec![non_overlap, left_overlap, right_overlap];
        assert!(r.overlaps_any(&many));
        let none = vec![DateRange::new(d(2023, 1, 1), d(2023, 1, 5))];
        assert!(!r.overlaps_any(&none));
    }

    #[test]
    fn prior_next_default_shift_by_len_and_n_variants() {
        let r = DateRange::new(d(2023, 1, 1), d(2023, 1, 7)); // len 7
        let p = r.prior();
        assert_eq!(p.start_date(), d(2022, 12, 25));
        assert_eq!(p.end_date(), d(2022, 12, 31));
        let n = r.next();
        assert_eq!(n.start_date(), d(2023, 1, 8));
        assert_eq!(n.end_date(), d(2023, 1, 14));

        // prior_n and next_n
        let p2 = r.prior_n(2);
        assert_eq!(p2.start_date(), d(2022, 12, 18));
        assert_eq!(p2.end_date(), d(2022, 12, 24));
        let n3 = r.next_n(3);
        assert_eq!(n3.start_date(), d(2023, 1, 22));
        assert_eq!(n3.end_date(), d(2023, 1, 28));
    }

    #[test]
    fn prior_next_with_custom_functions_shift_by_one_day() {
        let base = DateRange::new_with_prior_next_start_day(
            d(2023, 6, 10),
            d(2023, 6, 15),
            prior_shift_by_one,
            next_shift_by_one,
            Some(16),
        );
        // start_day should propagate
        assert_eq!(base.start_day(), Some(16));

        let p = base.prior();
        assert_eq!(p.start_date(), d(2023, 6, 9));
        assert_eq!(p.end_date(), d(2023, 6, 14));

        let n = base.next();
        assert_eq!(n.start_date(), d(2023, 6, 11));
        assert_eq!(n.end_date(), d(2023, 6, 16));
    }

    #[test]
    fn range_containing_date_moves_forward_and_backward() {
        // Base week Jan 1..Jan 7
        let base = DateRange::new(d(2023, 1, 1), d(2023, 1, 7));
        // Forward date Jan 19 -> should find Jan 15..21
        let f = base.range_containing_date(d(2023, 1, 19));
        assert_eq!(f.start_date(), d(2023, 1, 15));
        assert_eq!(f.end_date(), d(2023, 1, 21));
        // Backward date Dec 20 -> should find Dec 18..24
        let b = base.range_containing_date(d(2022, 12, 20));
        assert_eq!(b.start_date(), d(2022, 12, 18));
        assert_eq!(b.end_date(), d(2022, 12, 24));
    }

    #[test]
    fn ranges_before_and_after_and_window_orders() {
        let base = DateRange::new(d(2023, 1, 1), d(2023, 1, 7));
        let before = base.ranges_before(2);
        assert_eq!(before.len(), 2);
        assert_eq!(before[0].start_date(), d(2022, 12, 18));
        assert_eq!(before[0].end_date(), d(2022, 12, 24));
        assert_eq!(before[1].start_date(), d(2022, 12, 25));
        assert_eq!(before[1].end_date(), d(2022, 12, 31));

        let before_incl = base.ranges_before_inclusive(2);
        assert_eq!(before_incl.len(), 3);
        assert_eq!(before_incl[0].start_date(), d(2022, 12, 18));
        assert_eq!(before_incl[1].start_date(), d(2022, 12, 25));
        assert_eq!(before_incl[2].start_date(), d(2023, 1, 1));

        let after = base.ranges_after(2);
        assert_eq!(after.len(), 2);
        assert_eq!(after[0].start_date(), d(2023, 1, 8));
        assert_eq!(after[1].start_date(), d(2023, 1, 15));

        let after_incl = base.ranges_after_inclusive(2);
        assert_eq!(after_incl.len(), 3);
        assert_eq!(after_incl[0].start_date(), d(2023, 1, 1));
        assert_eq!(after_incl[1].start_date(), d(2023, 1, 8));
        assert_eq!(after_incl[2].start_date(), d(2023, 1, 15));

        let window = base.ranges_window(2, 2);
        let expected = vec![
            DateRange::new(d(2022, 12, 18), d(2022, 12, 24)),
            DateRange::new(d(2022, 12, 25), d(2022, 12, 31)),
            DateRange::new(d(2023, 1, 1), d(2023, 1, 7)),
            DateRange::new(d(2023, 1, 8), d(2023, 1, 14)),
            DateRange::new(d(2023, 1, 15), d(2023, 1, 21)),
        ];
        assert_eq!(window, expected);
    }

    #[test]
    fn ranges_containing_span_covers_all_intervening_ranges() {
        let base = DateRange::new(d(2023, 1, 1), d(2023, 1, 7));
        let ranges = base.ranges_containing_span(d(2023, 1, 5), d(2023, 1, 25));
        let starts: Vec<_> = ranges.iter().map(|r| r.start_date()).collect();
        assert_eq!(starts, vec![
            d(2023, 1, 1),
            d(2023, 1, 8),
            d(2023, 1, 15),
            d(2023, 1, 22),
        ]);
        assert!(ranges.first().unwrap().contains_date(d(2023, 1, 5)));
        assert!(ranges.last().unwrap().contains_date(d(2023, 1, 25)));
    }

    #[test]
    fn ordering_and_equality_semantics() {
        let a = DateRange::new(d(2023, 1, 1), d(2023, 1, 7));
        let b = DateRange::new(d(2023, 1, 8), d(2023, 1, 14));
        let c_same_as_a = DateRange::new(d(2023, 1, 1), d(2023, 1, 7));
        // equality compares start/end only
        assert_eq!(a, c_same_as_a);
        assert!(a < b);
        let mut v = vec![b, a];
        v.sort();
        assert_eq!(v, vec![a, b]);
    }
}
