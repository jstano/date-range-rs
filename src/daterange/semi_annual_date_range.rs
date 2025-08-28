use crate::daterange::date_range::DateRange;
use crate::dateutils::date_utils::{add_months, subtract_months};
use chrono::{Duration, NaiveDate};

pub struct SemiAnnualDateRange;

impl SemiAnnualDateRange {
    pub fn with_start_date(start_date: NaiveDate) -> DateRange {
        let end_date = add_months(start_date, 6) - Duration::days(1);

        DateRange::new_with_prior_next(
            start_date,
            end_date,
            SemiAnnualDateRange::prior,
            SemiAnnualDateRange::next,
        )
    }

    pub fn with_end_date(end_date: NaiveDate) -> DateRange {
        let start_date = subtract_months(end_date, 6) + Duration::days(1);

        DateRange::new_with_prior_next(
            start_date,
            end_date,
            SemiAnnualDateRange::prior,
            SemiAnnualDateRange::next,
        )
    }

    /// Returns the prior range.
    pub fn prior(date_range: &DateRange) -> DateRange {
        let start = subtract_months(date_range.start_date(), 6);
        let end = subtract_months(date_range.end_date(), 6);

        DateRange::new_with_prior_next(
            start,
            end,
            SemiAnnualDateRange::prior,
            SemiAnnualDateRange::next,
        )
    }

    /// Returns the next range.
    pub fn next(date_range: &DateRange) -> DateRange {
        let start = add_months(date_range.start_date(), 6);
        let end = add_months(date_range.end_date(), 6);

        DateRange::new_with_prior_next(
            start,
            end,
            SemiAnnualDateRange::prior,
            SemiAnnualDateRange::next,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::SemiAnnualDateRange;
    use chrono::NaiveDate;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).expect("invalid date")
    }

    #[test]
    fn with_start_date_spans_six_months_inclusive_non_leap_and_leap() {
        // Start on Jan 1, 2023 -> end Jun 30, 2023 (31+28+31+30+31+30 = 181)
        let r1 = SemiAnnualDateRange::with_start_date(d(2023, 1, 1));
        assert_eq!(r1.start_date(), d(2023, 1, 1));
        assert_eq!(r1.end_date(), d(2023, 6, 30));
        assert_eq!(r1.len(), 181);

        // Start on Jan 1, 2020 (leap year) -> end Jun 30, 2020 (182)
        let r2 = SemiAnnualDateRange::with_start_date(d(2020, 1, 1));
        assert_eq!(r2.start_date(), d(2020, 1, 1));
        assert_eq!(r2.end_date(), d(2020, 6, 30));
        assert_eq!(r2.len(), 182);

        // Start on Jul 1, 2023 -> end Dec 31, 2023 (184)
        let r3 = SemiAnnualDateRange::with_start_date(d(2023, 7, 1));
        assert_eq!(r3.start_date(), d(2023, 7, 1));
        assert_eq!(r3.end_date(), d(2023, 12, 31));
        assert_eq!(r3.len(), 184);
    }

    #[test]
    fn with_end_date_spans_six_months_inclusive() {
        // End on Jun 30, 2023 -> start Dec 31, 2022 (preserves day-of-month symmetry)
        let r1 = SemiAnnualDateRange::with_end_date(d(2023, 6, 30));
        assert_eq!(r1.start_date(), d(2022, 12, 31));
        assert_eq!(r1.end_date(), d(2023, 6, 30));

        // End on Dec 31, 2023 -> start Jul 1, 2023
        let r2 = SemiAnnualDateRange::with_end_date(d(2023, 12, 31));
        assert_eq!(r2.start_date(), d(2023, 7, 1));
        assert_eq!(r2.end_date(), d(2023, 12, 31));
    }

    #[test]
    fn prior_and_next_shift_by_six_months_with_year_boundary() {
        // Mid-period start to ensure day-of-month preservation
        let r = SemiAnnualDateRange::with_start_date(d(2023, 3, 15));
        assert_eq!(r.start_date(), d(2023, 3, 15));
        assert_eq!(r.end_date(), d(2023, 9, 14));

        let next = r.next();
        assert_eq!(next.start_date(), d(2023, 9, 15));
        assert_eq!(next.end_date(), d(2024, 3, 14));

        let prior = r.prior();
        assert_eq!(prior.start_date(), d(2022, 9, 15));
        assert_eq!(prior.end_date(), d(2023, 3, 14));

        // Year boundary: Jul 1, 2023 -> Dec 31, 2023; next -> Jan 1, 2024 .. Jun 30, 2024
        let h2 = SemiAnnualDateRange::with_start_date(d(2023, 7, 1));
        let h1_2024 = h2.next();
        assert_eq!(h1_2024.start_date(), d(2024, 1, 1));
        assert_eq!(h1_2024.end_date(), d(2024, 6, 30));

        // Prior from Jan 1, 2024 -> Jul 1, 2023 .. Dec 30, 2023 (end shifted by add_months semantics)
        let back = h1_2024.prior();
        assert_eq!(back.start_date(), d(2023, 7, 1));
        assert_eq!(back.end_date(), d(2023, 12, 30));
    }

    #[test]
    fn range_containing_date_aligns_to_semi_annual_blocks() {
        // Base: 2023-01-01 .. 2023-06-30
        let base = SemiAnnualDateRange::with_start_date(d(2023, 1, 1));

        // A date in October should map to 2023-07-01 .. 2023-12-31
        let oct = d(2023, 10, 10);
        let found = base.range_containing_date(oct);
        assert_eq!(found.start_date(), d(2023, 7, 1));
        assert_eq!(found.end_date(), d(2023, 12, 30));
        assert!(found.contains_date(oct));

        // A date in previous year November should map to 2022-07-01 .. 2022-12-31
        let nov_prev = d(2022, 11, 5);
        let found2 = base.range_containing_date(nov_prev);
        assert_eq!(found2.start_date(), d(2022, 7, 1));
        assert_eq!(found2.end_date(), d(2022, 12, 30));
        assert!(found2.contains_date(nov_prev));
    }

    #[test]
    fn handles_end_of_month_clamping_on_add_months() {
        // Starting on Aug 31, 2019; add 6 months clamps to Feb 29, 2020; minus 1 day => Feb 28, 2020
        let r = SemiAnnualDateRange::with_start_date(d(2019, 8, 31));
        assert_eq!(r.start_date(), d(2019, 8, 31));
        assert_eq!(r.end_date(), d(2020, 2, 28));

        // Ending on Feb 29, 2020 -> subtract 6 months clamps to Aug 29, 2019; +1 day -> Aug 30, 2019
        let r2 = SemiAnnualDateRange::with_end_date(d(2020, 2, 29));
        assert_eq!(r2.start_date(), d(2019, 8, 30));
        assert_eq!(r2.end_date(), d(2020, 2, 29));
    }
}
