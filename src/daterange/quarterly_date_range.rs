use crate::daterange::date_range::DateRange;
use crate::dateutils::date_utils::{add_months, first_day_of_month, last_day_of_month, subtract_months};
use chrono::NaiveDate;

pub struct QuarterlyDateRange;

impl QuarterlyDateRange {
    /// Creates a quarterly range starting at the given start_date.
    pub fn with_start_date(start_date: NaiveDate) -> DateRange {
        let start = first_day_of_month(start_date);
        let end = last_day_of_month(add_months(first_day_of_month(start_date), 2));

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

#[cfg(test)]
mod tests {
    use super::QuarterlyDateRange;
    use chrono::NaiveDate;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).expect("invalid date")
    }

    #[test]
    fn with_start_date_spans_full_quarter_non_leap_and_leap() {
        // Q1 2023: Jan 1 .. Mar 31 (31 + 28 + 31 = 90)
        let q1_2023 = QuarterlyDateRange::with_start_date(d(2023, 1, 15)); // any date in Jan
        assert_eq!(q1_2023.start_date(), d(2023, 1, 1));
        // Expect last day of March
        assert_eq!(q1_2023.end_date(), d(2023, 3, 31));
        assert_eq!(q1_2023.len(), 90);

        // Q1 2020 (leap year): Jan 1 .. Mar 31 (31 + 29 + 31 = 91)
        let q1_2020 = QuarterlyDateRange::with_start_date(d(2020, 1, 10));
        assert_eq!(q1_2020.start_date(), d(2020, 1, 1));
        assert_eq!(q1_2020.end_date(), d(2020, 3, 31));
        assert_eq!(q1_2020.len(), 91);
    }

    #[test]
    fn with_end_date_spans_full_quarter() {
        // Quarter ending Mar 31 should start Jan 1
        let q_end_mar = QuarterlyDateRange::with_end_date(d(2023, 3, 31));
        assert_eq!(q_end_mar.start_date(), d(2023, 1, 1));
        assert_eq!(q_end_mar.end_date(), d(2023, 3, 31));

        // Quarter ending Jun 30 should start Apr 1
        let q_end_jun = QuarterlyDateRange::with_end_date(d(2023, 6, 30));
        assert_eq!(q_end_jun.start_date(), d(2023, 4, 1));
        assert_eq!(q_end_jun.end_date(), d(2023, 6, 30));
    }

    #[test]
    fn prior_and_next_shift_by_one_quarter_with_year_boundary() {
        // Q2 2023 (Apr 1..Jun 30)
        let q2 = QuarterlyDateRange::with_end_date(d(2023, 6, 30));

        let q1 = q2.prior();
        assert_eq!(q1.start_date(), d(2023, 1, 1));
        assert_eq!(q1.end_date(), d(2023, 3, 31));

        let q3 = q2.next();
        assert_eq!(q3.start_date(), d(2023, 7, 1));
        assert_eq!(q3.end_date(), d(2023, 9, 30));

        // Year boundary: Q4 2023 -> Q1 2024
        let q4 = QuarterlyDateRange::with_end_date(d(2023, 12, 31));
        let q1_2024 = q4.next();
        assert_eq!(q1_2024.start_date(), d(2024, 1, 1));
        assert_eq!(q1_2024.end_date(), d(2024, 3, 31));

        // Prior from Q1 2024 -> Q4 2023
        let back_to_q4 = q1_2024.prior();
        assert_eq!(back_to_q4.start_date(), d(2023, 10, 1));
        assert_eq!(back_to_q4.end_date(), d(2023, 12, 31));
    }

    #[test]
    fn range_containing_date_aligns_to_quarters() {
        // Base: Q1 2023, constructed from a Jan date
        let base = QuarterlyDateRange::with_start_date(d(2023, 1, 5));

        // Date in May should map to Q2: Apr 1 .. Jun 30
        let may_date = d(2023, 5, 10);
        let found = base.range_containing_date(may_date);
        assert_eq!(found.start_date(), d(2023, 4, 1));
        assert_eq!(found.end_date(), d(2023, 6, 30));
        assert!(found.contains_date(may_date));

        // Date in November should map to Q4: Oct 1 .. Dec 31
        let nov_date = d(2023, 11, 20);
        let found2 = base.range_containing_date(nov_date);
        assert_eq!(found2.start_date(), d(2023, 10, 1));
        assert_eq!(found2.end_date(), d(2023, 12, 31));
        assert!(found2.contains_date(nov_date));
    }
}
