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
