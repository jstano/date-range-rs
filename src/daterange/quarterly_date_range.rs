use crate::daterange::date_range::DateRange;
use crate::dateutils::date_utils::{add_months, first_day_of_month, last_day_of_month, subtract_months};
use chrono::NaiveDate;

pub struct QuarterlyDateRange;

impl QuarterlyDateRange {
    /// Creates a quarterly range starting at the given start_date.
    pub fn with_start_date(start_date: NaiveDate) -> DateRange {
        let start = first_day_of_month(start_date);
        let end = add_months(first_day_of_month(start_date), 2);

        DateRange::new_with_prior_next(start,
                                       end,
                                       QuarterlyDateRange::prior,
                                       QuarterlyDateRange::next)
    }

    /// Creates a quarterly range ending at the given end_date.
    pub fn with_end_date(end_date: NaiveDate) -> DateRange {
        let start = subtract_months(first_day_of_month(end_date), 2);
        let end = last_day_of_month(end_date);

        DateRange::new_with_prior_next(start,
                                       end,
                                       QuarterlyDateRange::prior,
                                       QuarterlyDateRange::next)
    }

    /// Returns the previous quarter.
    pub fn prior(date_range: &DateRange) -> DateRange {
        let start = subtract_months(date_range.start_date(), 3);
        let end = last_day_of_month(subtract_months(first_day_of_month(date_range.end_date()), 3));

        DateRange::new_with_prior_next(start,
                                       end,
                                       QuarterlyDateRange::prior,
                                       QuarterlyDateRange::next)
    }

    /// Returns the next quarter.
    pub fn next(date_range: &DateRange) -> DateRange {
        let start = add_months(date_range.start_date(), 3);
        let end = last_day_of_month(add_months(first_day_of_month(date_range.end_date()), 3));

        DateRange::new_with_prior_next(start,
                                       end,
                                       QuarterlyDateRange::prior,
                                       QuarterlyDateRange::next)
    }
}
