use crate::daterange::date_range::DateRange;
use crate::dateutils::date_utils::{add_months, last_day_of_month, subtract_months};
use chrono::{Datelike, Duration, Months, NaiveDate};

pub struct MonthlyDateRange;

impl MonthlyDateRange {
    pub fn with_end_date_on_first(end_date: NaiveDate) -> DateRange {
        Self::with_end_date_and_start_day(end_date, 1)
    }

    pub fn with_end_date_and_start_day(end_date: NaiveDate, start_day: usize) -> DateRange {
        let start_date = calculate_start_date_from_end_date(end_date, start_day);

        DateRange::new_with_prior_next_start_day(start_date,
                                                 end_date,
                                                 MonthlyDateRange::prior,
                                                 MonthlyDateRange::next,
                                                 Some(start_day))
    }

    fn prior(date_range: &DateRange) -> DateRange {
        if date_range.start_day().unwrap() == 1 {
            let new_end = date_range.start_date() - Duration::days(1);
            let new_start = new_end.with_day(1).unwrap();

            DateRange::new_with_prior_next_start_day(new_start,
                                                     new_end,
                                                     MonthlyDateRange::prior,
                                                     MonthlyDateRange::next,
                                                     date_range.start_day())

        } else {
            let new_start = subtract_months(date_range.start_date(), 1);
            let new_end = date_range.start_date() - Duration::days(1);

            DateRange::new_with_prior_next_start_day(new_start,
                                                     new_end,
                                                     MonthlyDateRange::prior,
                                                     MonthlyDateRange::next,
                                                     date_range.start_day())
        }
    }

    fn next(date_range: &DateRange) -> DateRange {
        if date_range.start_day().unwrap() == 1 {
            let new_start = date_range.end_date() + Duration::days(1);
            let new_end = last_day_of_month(new_start);

            DateRange::new_with_prior_next_start_day(new_start,
                                                     new_end,
                                                     MonthlyDateRange::prior,
                                                     MonthlyDateRange::next,
                                                     date_range.start_day())
        } else {
            let new_start = date_range.end_date() + Duration::days(1);
            let new_end = add_months(date_range.end_date(), 1);

            DateRange::new_with_prior_next_start_day(new_start,
                                                     new_end,
                                                     MonthlyDateRange::prior,
                                                     MonthlyDateRange::next,
                                                     date_range.start_day())
        }
    }
}

fn calculate_start_date_from_end_date(end_date: NaiveDate, start_day: usize) -> NaiveDate {
    if start_day == 1 {
        NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), 1).unwrap()
    } else {
        end_date
            .succ_opt()
            .unwrap() // plusDays(1)
            .checked_sub_months(Months::new(1))
            .unwrap()
    }
}
