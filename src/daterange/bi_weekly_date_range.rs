use crate::daterange::date_range::DateRange;
use chrono::{Datelike, Duration, NaiveDate, Weekday};

pub struct BiWeeklyDateRange;

impl BiWeeklyDateRange {
    pub fn with_start_date(start_date: NaiveDate) -> DateRange {
        let end = start_date + Duration::days(13);
        DateRange::new(start_date, end)
    }

    pub fn with_end_date(end_date: NaiveDate) -> DateRange {
        let start = end_date - Duration::days(13);
        DateRange::new(start, end_date)
    }

    pub fn with_target_date(target: NaiveDate, end_day: Weekday) -> DateRange {
        let offset = calculate_day_of_week_offset(target, end_day);
        let end = target + Duration::days(offset as i64);
        let start = end - Duration::days(13);
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
    use super::BiWeeklyDateRange;
    use chrono::{Duration, NaiveDate};

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).expect("invalid date")
    }

    #[test]
    fn with_start_date_spans_14_days_inclusive() {
        let start = d(2023, 3, 1);
        let dr = BiWeeklyDateRange::with_start_date(start);

        assert_eq!(dr.start_date(), d(2023, 3, 1));
        assert_eq!(dr.end_date(), start + Duration::days(13));
    }

    #[test]
    fn with_end_date_spans_14_days_inclusive() {
        let end = d(2023, 3, 14);
        let dr = BiWeeklyDateRange::with_end_date(end);

        assert_eq!(dr.end_date(), d(2023, 3, 14));
        assert_eq!(dr.start_date(), end - Duration::days(13));
    }

    #[test]
    fn prior_and_next_shift_by_two_weeks() {
        let start = d(2023, 5, 10);
        let dr = BiWeeklyDateRange::with_start_date(start);

        let prior = dr.prior();
        let next = dr.next();

        assert_eq!(prior.start_date(), start - Duration::days(14));
        assert_eq!(prior.end_date(), dr.start_date() - Duration::days(1));

        assert_eq!(next.start_date(), dr.end_date() + Duration::days(1));
        assert_eq!(next.end_date(), dr.end_date() + Duration::days(14));
    }

    #[test]
    fn leap_year_boundary_is_handled() {
        // Starting on 2020-02-24 (leap year) should end on 2020-03-08 (13 days later)
        let dr = BiWeeklyDateRange::with_start_date(d(2020, 2, 24));
        assert_eq!(dr.start_date(), d(2020, 2, 24));
        assert_eq!(dr.end_date(), d(2020, 3, 8));

        // Ending on 2020-03-08 should start on 2020-02-24
        let dr2 = BiWeeklyDateRange::with_end_date(d(2020, 3, 8));
        assert_eq!(dr2.start_date(), d(2020, 2, 24));
        assert_eq!(dr2.end_date(), d(2020, 3, 8));
    }

    #[test]
    fn range_containing_date_finds_correct_segment_forward_and_backward() {
        // Base range: 2023-01-01 .. 2023-01-14
        let base = BiWeeklyDateRange::with_start_date(d(2023, 1, 1));

        // A date well ahead should land in a later segment
        let target = d(2023, 2, 10);
        let found = base.range_containing_date(target);
        assert!(found.contains_date(target));
        // The found range should align to 14-day blocks from the base start
        let offset_days = (found.start_date() - base.start_date()).num_days();
        assert_eq!(offset_days % 14, 0);

        // A date behind should land in an earlier segment
        let earlier_target = d(2022, 12, 20);
        let found_earlier = base.range_containing_date(earlier_target);
        assert!(found_earlier.contains_date(earlier_target));
        let offset_days_earlier = (base.start_date() - found_earlier.start_date()).num_days();
        assert_eq!(offset_days_earlier % 14, 0);
    }

    // ===== Additional tests for with_target_date(end_day) =====
    #[test]
    fn with_target_date_when_target_is_end_day() {
        use chrono::Weekday;
        // If target is the end day, end should equal target and start should be 13 days prior
        let target = d(2023, 1, 1); // Sunday
        let dr = BiWeeklyDateRange::with_target_date(target, Weekday::Sun);
        assert_eq!(dr.end_date(), target);
        assert_eq!(dr.start_date(), target - chrono::Duration::days(13));
        assert_eq!(dr.len(), 14);
    }

    #[test]
    fn with_target_date_wraps_forward_to_specified_end_day_same_week() {
        use chrono::Weekday;
        // Example: target Monday with end_day Sunday -> end is following Sunday of the same week
        let target = d(2023, 1, 2); // Monday
        let dr = BiWeeklyDateRange::with_target_date(target, Weekday::Sun);
        assert_eq!(dr.end_date(), d(2023, 1, 8));
        assert_eq!(dr.start_date(), d(2022, 12, 26)); // 13 days before end
        assert_eq!(dr.len(), 14);
    }

    #[test]
    fn with_target_date_when_target_after_end_day_wraps_to_next_week() {
        use chrono::Weekday;
        // Example: target Tuesday, end_day Monday -> wrap to next Monday
        let target = d(2023, 1, 3); // Tuesday
        let dr = BiWeeklyDateRange::with_target_date(target, Weekday::Mon);
        assert_eq!(dr.end_date(), d(2023, 1, 9)); // next Monday
        assert_eq!(dr.start_date(), d(2022, 12, 27));
        assert_eq!(dr.len(), 14);
    }

    #[test]
    fn with_target_date_handles_year_boundary() {
        use chrono::Weekday;
        // Late December Monday, end_day Sunday -> end in early January
        let target = d(2020, 12, 28); // Monday
        let dr = BiWeeklyDateRange::with_target_date(target, Weekday::Sun);
        assert_eq!(dr.end_date(), d(2021, 1, 3));
        assert_eq!(dr.start_date(), d(2020, 12, 21));
        assert_eq!(dr.len(), 14);
    }
}
