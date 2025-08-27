use crate::daterange::date_range::DateRange;
use crate::dateutils::date_utils::{add_years, subtract_years};
use chrono::{Duration, NaiveDate};

pub struct AnnualDateRange;

impl AnnualDateRange {
    pub fn with_start_date(start_date: NaiveDate) -> DateRange {
        let end_date = add_years(start_date, 1) - Duration::days(1);

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
        let end = subtract_years(date_range.end_date(), 1);

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
        let end = add_years(date_range.end_date(), 1);

        DateRange::new_with_prior_next(
            start,
            end,
            AnnualDateRange::prior,
            AnnualDateRange::next,
        )
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
    #[ignore]
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
    #[ignore]
    fn round_trip_prior_next_keeps_original_bounds() {
        let dr = AnnualDateRange::with_start_date(d(2019, 3, 1));
        let back = dr.next().prior();

        assert_eq!(back.start_date(), dr.start_date());
        assert_eq!(back.end_date(), dr.end_date());
    }
}

