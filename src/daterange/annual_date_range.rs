use crate::daterange::date_range::DateRange;
use crate::dateutils::date_utils::{add_years, subtract_years};
use chrono::{Datelike, Duration, NaiveDate};

pub struct AnnualDateRange;

impl AnnualDateRange {
    pub fn with_start_date(start_date: NaiveDate) -> DateRange {
        let end_date = Self::end_for_start(start_date);

        DateRange::new_with_prior_next(
            start_date,
            end_date,
            AnnualDateRange::prior,
            AnnualDateRange::next,
        )
    }

    pub fn with_end_date(end_date: NaiveDate) -> DateRange {
        let start_date = subtract_years(end_date, 1) + Duration::days(1);

        DateRange::new_with_prior_next(
            start_date,
            end_date,
            AnnualDateRange::prior,
            AnnualDateRange::next,
        )
    }

    /// Returns the previous year.
    pub fn prior(date_range: &DateRange) -> DateRange {
        let start = subtract_years(date_range.start_date(), 1);
        let end = Self::end_for_start(start);

        DateRange::new_with_prior_next(
            start,
            end,
            AnnualDateRange::prior,
            AnnualDateRange::next,
        )
    }

    /// Returns the next year.
    pub fn next(date_range: &DateRange) -> DateRange {
        let start = add_years(date_range.start_date(), 1);
        let end = Self::end_for_start(start);

        DateRange::new_with_prior_next(
            start,
            end,
            AnnualDateRange::prior,
            AnnualDateRange::next,
        )
    }

    fn end_for_start(start_date: NaiveDate) -> NaiveDate {
        // Normally, the end is the day before the same calendar date next year.
        // Special-case Feb 29: the anniversary next year is clamped to Feb 28, and
        // subtracting 1 day would yield Feb 27. The correct end for a range starting
        // on Feb 29 is Feb 28 of the following year.
        if start_date.month() == 2 && start_date.day() == 29 {
            NaiveDate::from_ymd_opt(start_date.year() + 1, 2, 28).unwrap()
        } else {
            add_years(start_date, 1) - Duration::days(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AnnualDateRange;
    use chrono::NaiveDate;

    // Helper to parse YYYY-MM-DD easily
    fn d(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).expect("invalid date")
    }

    #[test]
    fn with_start_date_regular_year() {
        // Jan 1, 2023 -> Dec 31, 2023
        let start = d(2023, 1, 1);
        let dr = AnnualDateRange::with_start_date(start);

        assert_eq!(dr.start_date(), d(2023, 1, 1));
        assert_eq!(dr.end_date(), d(2023, 12, 31));
    }

    #[test]
    fn with_end_date_regular_year() {
        // Dec 31, 2023 -> Jan 1, 2023
        let end = d(2023, 12, 31);
        let dr = AnnualDateRange::with_end_date(end);

        assert_eq!(dr.start_date(), d(2023, 1, 1));
        assert_eq!(dr.end_date(), d(2023, 12, 31));
    }

    #[test]
    fn prior_and_next_shift_one_year() {
        let dr_2023 = AnnualDateRange::with_start_date(d(2023, 1, 1));
        let prior = dr_2023.prior();
        let next = dr_2023.next();

        assert_eq!(prior.start_date(), d(2022, 1, 1));
        assert_eq!(prior.end_date(), d(2022, 12, 31));

        assert_eq!(next.start_date(), d(2024, 1, 1));
        assert_eq!(next.end_date(), d(2024, 12, 31));
    }

    #[test]
    fn leap_year_with_start_date_feb_29() {
        // Starting on leap day 2020-02-29 should end on 2021-02-28
        let dr = AnnualDateRange::with_start_date(d(2020, 2, 29));

        assert_eq!(dr.start_date(), d(2020, 2, 29));
        assert_eq!(dr.end_date(), d(2021, 2, 28));
    }

    #[test]
    fn leap_year_with_end_date_feb_28_following_non_leap() {
        // Ending on 2021-02-28 should start on 2020-02-29 (inclusive range)
        let dr = AnnualDateRange::with_end_date(d(2021, 2, 28));

        assert_eq!(dr.start_date(), d(2020, 2, 29));
        assert_eq!(dr.end_date(), d(2021, 2, 28));
    }

    #[test]
    fn round_trip_prior_next_keeps_original_bounds() {
        let dr = AnnualDateRange::with_start_date(d(2019, 3, 1));
        let back = dr.next().prior();

        assert_eq!(back.start_date(), dr.start_date());
        assert_eq!(back.end_date(), dr.end_date());
    }

    #[test]
    fn calendar_year_lengths_365_and_366() {
        let y2023 = AnnualDateRange::with_start_date(d(2023, 1, 1));
        assert_eq!(y2023.start_date(), d(2023, 1, 1));
        assert_eq!(y2023.end_date(), d(2023, 12, 31));
        assert_eq!(y2023.len(), 365);

        let y2024 = AnnualDateRange::with_start_date(d(2024, 1, 1));
        assert_eq!(y2024.start_date(), d(2024, 1, 1));
        assert_eq!(y2024.end_date(), d(2024, 12, 31));
        assert_eq!(y2024.len(), 366); // leap year
    }

    #[test]
    fn fiscal_year_starting_july_1_has_correct_length_across_leap() {
        // 2019-07-01 .. 2020-06-30 includes Feb 29, 2020 -> 366 days
        let fy_2019 = AnnualDateRange::with_start_date(d(2019, 7, 1));
        assert_eq!(fy_2019.start_date(), d(2019, 7, 1));
        assert_eq!(fy_2019.end_date(), d(2020, 6, 30));
        assert_eq!(fy_2019.len(), 366);

        // 2021-07-01 .. 2022-06-30 does not include a Feb 29 -> 365 days
        let fy_2021 = AnnualDateRange::with_start_date(d(2021, 7, 1));
        assert_eq!(fy_2021.start_date(), d(2021, 7, 1));
        assert_eq!(fy_2021.end_date(), d(2022, 6, 30));
        assert_eq!(fy_2021.len(), 365);
    }

    #[test]
    fn with_end_date_for_non_december_end() {
        // End 2020-06-30 -> start 2019-07-01 (366 days span)
        let r1 = AnnualDateRange::with_end_date(d(2020, 6, 30));
        assert_eq!(r1.start_date(), d(2019, 7, 1));
        assert_eq!(r1.end_date(), d(2020, 6, 30));
        assert_eq!(r1.len(), 366);

        // End 2021-06-30 -> start 2020-07-01 (365 days span)
        let r2 = AnnualDateRange::with_end_date(d(2021, 6, 30));
        assert_eq!(r2.start_date(), d(2020, 7, 1));
        assert_eq!(r2.end_date(), d(2021, 6, 30));
        assert_eq!(r2.len(), 365);
    }

    #[test]
    fn range_containing_date_aligns_to_calendar_years() {
        let base = AnnualDateRange::with_start_date(d(2022, 1, 1)); // 2022-01-01..2022-12-31

        let target = d(2025, 5, 10);
        let found = base.range_containing_date(target);
        assert_eq!(found.start_date(), d(2025, 1, 1));
        assert_eq!(found.end_date(), d(2025, 12, 31));
        assert!(found.contains_date(target));

        // Earlier target
        let earlier = d(2020, 3, 15);
        let found_earlier = base.range_containing_date(earlier);
        assert_eq!(found_earlier.start_date(), d(2020, 1, 1));
        assert_eq!(found_earlier.end_date(), d(2020, 12, 31));
        assert!(found_earlier.contains_date(earlier));
    }

    #[test]
    fn range_containing_date_aligns_to_fiscal_year_starting_july() {
        let base = AnnualDateRange::with_start_date(d(2023, 7, 1)); // 2023-07-01..2024-06-30

        let last_day = d(2024, 6, 30);
        let found = base.range_containing_date(last_day);
        assert_eq!(found.start_date(), d(2023, 7, 1));
        assert_eq!(found.end_date(), d(2024, 6, 30));
        assert!(found.contains_date(last_day));

        let next_year_day = d(2024, 7, 1);
        let found2 = base.range_containing_date(next_year_day);
        assert_eq!(found2.start_date(), d(2024, 7, 1));
        assert_eq!(found2.end_date(), d(2025, 6, 30));
        assert!(found2.contains_date(next_year_day));
    }
}
