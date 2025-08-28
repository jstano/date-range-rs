use crate::daterange::date_range::DateRange;
use chrono::{Datelike, Duration, NaiveDate, Weekday};

pub struct WeeklyDateRange;

impl WeeklyDateRange {
    pub fn with_start_date(start_date: NaiveDate) -> DateRange {
        let end = start_date + Duration::days(6);
        DateRange::new(start_date, end)
    }

    pub fn with_end_date(end_date: NaiveDate) -> DateRange {
        let start = end_date - Duration::days(6);
        DateRange::new(start, end_date)
    }

    pub fn with_target_date(target: NaiveDate, end_day: Weekday) -> DateRange {
        let offset = calculate_day_of_week_offset(target, end_day);
        let end = target + Duration::days(offset as i64);
        let start = end - Duration::days(6);
        DateRange::new(start, end)
    }
}

fn calculate_day_of_week_offset(date: NaiveDate, end_day: Weekday) -> i64 {
    let mut offset = end_day.num_days_from_monday() as i64 - date.weekday().num_days_from_monday() as i64;
    if offset < 0 {
        offset += 7;
    }
    offset
}

#[cfg(test)]
mod tests {
    use super::WeeklyDateRange;
    use chrono::{Duration, NaiveDate, Weekday};

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).expect("invalid date")
    }

    #[test]
    fn with_start_date_spans_7_days_inclusive() {
        let start = d(2023, 3, 1);
        let dr = WeeklyDateRange::with_start_date(start);

        assert_eq!(dr.start_date(), d(2023, 3, 1));
        assert_eq!(dr.end_date(), start + Duration::days(6));
        assert_eq!(dr.len(), 7);
    }

    #[test]
    fn with_end_date_spans_7_days_inclusive() {
        let end = d(2023, 3, 7);
        let dr = WeeklyDateRange::with_end_date(end);

        assert_eq!(dr.end_date(), d(2023, 3, 7));
        assert_eq!(dr.start_date(), end - Duration::days(6));
        assert_eq!(dr.len(), 7);
    }

    #[test]
    fn prior_and_next_shift_by_one_week() {
        let start = d(2023, 5, 10);
        let dr = WeeklyDateRange::with_start_date(start);

        let prior = dr.prior();
        let next = dr.next();

        assert_eq!(prior.start_date(), start - Duration::days(7));
        assert_eq!(prior.end_date(), dr.start_date() - Duration::days(1));

        assert_eq!(next.start_date(), dr.end_date() + Duration::days(1));
        assert_eq!(next.end_date(), dr.end_date() + Duration::days(7));
    }

    #[test]
    fn with_target_date_when_target_is_end_day() {
        // If target is the end day, end should equal target and start should be 6 days prior
        // Example: target is Sunday and end_day is Sunday
        let target = d(2023, 1, 1); // 2023-01-01 was a Sunday
        let dr = WeeklyDateRange::with_target_date(target, Weekday::Sun);

        assert_eq!(dr.end_date(), target);
        assert_eq!(dr.start_date(), target - Duration::days(6));
        assert_eq!(dr.len(), 7);
    }

    #[test]
    fn with_target_date_wraps_forward_to_specified_end_day() {
        // Example: target Monday with end_day Sunday -> end is following Sunday
        let target = d(2023, 1, 2); // Monday
        let dr = WeeklyDateRange::with_target_date(target, Weekday::Sun);

        assert_eq!(dr.end_date(), d(2023, 1, 8));
        assert_eq!(dr.start_date(), d(2023, 1, 2));
    }

    #[test]
    fn with_target_date_when_target_after_end_day_wraps_to_next_week() {
        // Example: target Tuesday, end_day Monday -> wrap to next Monday
        let target = d(2023, 1, 3); // Tuesday
        let dr = WeeklyDateRange::with_target_date(target, Weekday::Mon);

        assert_eq!(dr.end_date(), d(2023, 1, 9)); // next Monday
        assert_eq!(dr.start_date(), d(2023, 1, 3)); // Tuesday
    }

    #[test]
    fn handles_year_boundary() {
        // Starting late December should end in early January next year
        let dr = WeeklyDateRange::with_start_date(d(2020, 12, 29));
        assert_eq!(dr.start_date(), d(2020, 12, 29));
        assert_eq!(dr.end_date(), d(2021, 1, 4));
    }

    #[test]
    fn leap_year_week_crossing_feb_29() {
        // A week covering Feb 29 in a leap year
        let dr = WeeklyDateRange::with_start_date(d(2020, 2, 25));
        assert_eq!(dr.start_date(), d(2020, 2, 25));
        assert_eq!(dr.end_date(), d(2020, 3, 2));
    }

    #[test]
    fn range_containing_date_aligns_by_weeks() {
        let base = WeeklyDateRange::with_start_date(d(2023, 1, 1));

        let target = d(2023, 2, 10);
        let found = base.range_containing_date(target);
        assert!(found.contains_date(target));
        let offset_days = (found.start_date() - base.start_date()).num_days();
        assert_eq!(offset_days % 7, 0);

        let earlier_target = d(2022, 12, 15);
        let found_earlier = base.range_containing_date(earlier_target);
        assert!(found_earlier.contains_date(earlier_target));
        let offset_days_earlier = (base.start_date() - found_earlier.start_date()).num_days();
        assert_eq!(offset_days_earlier % 7, 0);
    }
}
