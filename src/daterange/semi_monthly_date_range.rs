use crate::daterange::date_range::DateRange;
use crate::dateutils::date_utils::last_day_of_month;
use chrono::{Datelike, Duration, NaiveDate};

pub struct SemiMonthlyDateRange;

const FIFTEENTH_OF_MONTH: u32 = 15;

impl SemiMonthlyDateRange {
    pub fn with_end_date(end_date: NaiveDate) -> DateRange {
        let start = calculate_start_date_from_end_date(end_date);
        DateRange::new_with_prior_next(start,
                                       end_date,
                                       SemiMonthlyDateRange::prior,
                                       SemiMonthlyDateRange::next)
    }

    fn prior(date_range: &DateRange) -> DateRange {
        let end_date = date_range.start_date() - Duration::days(1);
        let start_date = if date_range.start_date().day() == FIFTEENTH_OF_MONTH {
            // prior is 1st → 15th
            NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), 1).unwrap()
        } else {
            // prior is 16th → last day of the previous month
            NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), FIFTEENTH_OF_MONTH).unwrap()
        };

        DateRange::new(start_date, end_date)
    }

    fn next(date_range: &DateRange) -> DateRange {
        let start_date = if date_range.end_date().day() == FIFTEENTH_OF_MONTH {
            // next is the 16th → last day of the month
            NaiveDate::from_ymd_opt(
                date_range.end_date().year(),
                date_range.end_date().month(),
                FIFTEENTH_OF_MONTH + 1,
            ).unwrap()
        } else {
            // next is 1st → 15th of next month
            let next_month = date_range.end_date().month() % 12 + 1;
            let year = if next_month == 1 {
                date_range.end_date().year() + 1
            } else {
                date_range.end_date().year()
            };
            NaiveDate::from_ymd_opt(year, next_month, 1).unwrap()
        };

        let end_date = if start_date.day() == 1 {
            NaiveDate::from_ymd_opt(start_date.year(), start_date.month(), FIFTEENTH_OF_MONTH).unwrap()
        } else {
            last_day_of_month(start_date)
        };

        DateRange::new(start_date, end_date)
    }
}

/// Calculate the start date given an end date.
///
/// Valid end dates are either the 15th of the month or the last day of the month.
fn calculate_start_date_from_end_date(end_date: NaiveDate) -> NaiveDate {
    if end_date.day() == FIFTEENTH_OF_MONTH {
        NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), FIFTEENTH_OF_MONTH).unwrap()
    }
}
