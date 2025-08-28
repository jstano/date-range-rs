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
        let start_date = if date_range.start_date().day() == 1 {
            // current is 1..15 -> prior is 16..last of previous month
            NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), FIFTEENTH_OF_MONTH + 1).unwrap()
        } else {
            // current is 16..end -> prior is 1..15 of same month
            NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), 1).unwrap()
        };

        DateRange::new_with_prior_next(
            start_date,
            end_date,
            SemiMonthlyDateRange::prior,
            SemiMonthlyDateRange::next,
        )
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

        DateRange::new_with_prior_next(
            start_date,
            end_date,
            SemiMonthlyDateRange::prior,
            SemiMonthlyDateRange::next,
        )
    }
}

/// Calculate the start date given an end date.
///
/// Valid end dates are either the 15th of the month or the last day of the month.
fn calculate_start_date_from_end_date(end_date: NaiveDate) -> NaiveDate {
    if end_date.day() == FIFTEENTH_OF_MONTH {
        NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), 1).unwrap()
    } else {
        // When ending on the last day of the month, the semi-monthly period starts on the 16th
        NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), FIFTEENTH_OF_MONTH + 1).unwrap()
    }
}



#[cfg(test)]
mod tests {
    use super::SemiMonthlyDateRange;
    use chrono::NaiveDate;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).expect("invalid date")
    }

    #[test]
    fn with_end_date_on_15th_spans_1_to_15_inclusive() {
        let dr = SemiMonthlyDateRange::with_end_date(d(2023, 3, 15));
        assert_eq!(dr.start_date(), d(2023, 3, 1));
        assert_eq!(dr.end_date(), d(2023, 3, 15));
        assert_eq!(dr.len(), 15);
    }

    #[test]
    fn with_end_date_on_last_day_spans_16_to_last_inclusive_various_month_lengths() {
        // 31-day month (January): 16..31 length 16
        let dr_jan = SemiMonthlyDateRange::with_end_date(d(2023, 1, 31));
        assert_eq!(dr_jan.start_date(), d(2023, 1, 16));
        assert_eq!(dr_jan.end_date(), d(2023, 1, 31));
        assert_eq!(dr_jan.len(), 16);

        // 30-day month (April): 16..30 length 15
        let dr_apr = SemiMonthlyDateRange::with_end_date(d(2023, 4, 30));
        assert_eq!(dr_apr.start_date(), d(2023, 4, 16));
        assert_eq!(dr_apr.end_date(), d(2023, 4, 30));
        assert_eq!(dr_apr.len(), 15);

        // 28-day February (non-leap): 16..28 length 13
        let dr_feb = SemiMonthlyDateRange::with_end_date(d(2023, 2, 28));
        assert_eq!(dr_feb.start_date(), d(2023, 2, 16));
        assert_eq!(dr_feb.end_date(), d(2023, 2, 28));
        assert_eq!(dr_feb.len(), 13);

        // 29-day February (leap year): 16..29 length 14
        let dr_feb_leap = SemiMonthlyDateRange::with_end_date(d(2020, 2, 29));
        assert_eq!(dr_feb_leap.start_date(), d(2020, 2, 16));
        assert_eq!(dr_feb_leap.end_date(), d(2020, 2, 29));
        assert_eq!(dr_feb_leap.len(), 14);
    }

    #[test]
    fn next_links_1_15_to_16_last_and_16_last_to_next_month_1_15() {
        // 1..15 March -> 16..31 March
        let first_half = SemiMonthlyDateRange::with_end_date(d(2023, 3, 15));
        let second_half = first_half.next();
        assert_eq!(second_half.start_date(), d(2023, 3, 16));
        assert_eq!(second_half.end_date(), d(2023, 3, 31));

        // 16..31 March -> 1..15 April
        let april_first_half = second_half.next();
        assert_eq!(april_first_half.start_date(), d(2023, 4, 1));
        assert_eq!(april_first_half.end_date(), d(2023, 4, 15));
    }

    #[test]
    fn prior_links_16_last_to_1_15_and_1_15_to_prev_16_last() {
        // 16..30 April -> prior should be 1..15 April
        let second_half_apr = SemiMonthlyDateRange::with_end_date(d(2023, 4, 30));
        let first_half_apr = second_half_apr.prior();
        assert_eq!(first_half_apr.start_date(), d(2023, 4, 1));
        assert_eq!(first_half_apr.end_date(), d(2023, 4, 15));

        // 1..15 April -> prior should be 16..31 March
        let prior_mar_second_half = first_half_apr.prior();
        assert_eq!(prior_mar_second_half.start_date(), d(2023, 3, 16));
        assert_eq!(prior_mar_second_half.end_date(), d(2023, 3, 31));
    }

    #[test]
    fn handles_year_boundary_in_next_and_prior() {
        // 16..31 Dec 2023 -> next = 1..15 Jan 2024
        let dec_second = SemiMonthlyDateRange::with_end_date(d(2023, 12, 31));
        let jan_first = dec_second.next();
        assert_eq!(jan_first.start_date(), d(2024, 1, 1));
        assert_eq!(jan_first.end_date(), d(2024, 1, 15));

        // 1..15 Jan 2024 -> prior = 16..31 Dec 2023
        let jan_first = SemiMonthlyDateRange::with_end_date(d(2024, 1, 15));
        let dec_second = jan_first.prior();
        assert_eq!(dec_second.start_date(), d(2023, 12, 16));
        assert_eq!(dec_second.end_date(), d(2023, 12, 31));
    }
}
