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
