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


#[cfg(test)]
mod tests {
    use super::MonthlyDateRange;
    use chrono::{Datelike, NaiveDate};

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).expect("invalid date")
    }

    // ============ Calendar-month mode (start_day = 1) ============

    #[test]
    fn with_end_date_on_first_spans_full_month_various_lengths() {
        // 31-day month
        let jan = MonthlyDateRange::with_end_date_on_first(d(2023, 1, 31));
        assert_eq!(jan.start_date(), d(2023, 1, 1));
        assert_eq!(jan.end_date(), d(2023, 1, 31));
        assert_eq!(jan.len(), 31);

        // 30-day month
        let apr = MonthlyDateRange::with_end_date_on_first(d(2023, 4, 30));
        assert_eq!(apr.start_date(), d(2023, 4, 1));
        assert_eq!(apr.end_date(), d(2023, 4, 30));
        assert_eq!(apr.len(), 30);

        // 28-day February (non-leap year)
        let feb_28 = MonthlyDateRange::with_end_date_on_first(d(2023, 2, 28));
        assert_eq!(feb_28.start_date(), d(2023, 2, 1));
        assert_eq!(feb_28.end_date(), d(2023, 2, 28));
        assert_eq!(feb_28.len(), 28);

        // 29-day February (leap year)
        let feb_29 = MonthlyDateRange::with_end_date_on_first(d(2020, 2, 29));
        assert_eq!(feb_29.start_date(), d(2020, 2, 1));
        assert_eq!(feb_29.end_date(), d(2020, 2, 29));
        assert_eq!(feb_29.len(), 29);
    }

    #[test]
    fn prior_and_next_link_calendar_months_and_handle_year_boundary() {
        let nov = MonthlyDateRange::with_end_date_on_first(d(2023, 11, 30));
        let dec = nov.next();
        assert_eq!(dec.start_date(), d(2023, 12, 1));
        assert_eq!(dec.end_date(), d(2023, 12, 31));

        let jan = dec.next();
        assert_eq!(jan.start_date(), d(2024, 1, 1));
        assert_eq!(jan.end_date(), d(2024, 1, 31));

        let back_to_dec = jan.prior();
        assert_eq!(back_to_dec.start_date(), d(2023, 12, 1));
        assert_eq!(back_to_dec.end_date(), d(2023, 12, 31));
    }

    #[test]
    fn range_containing_date_aligns_to_calendar_months() {
        let base = MonthlyDateRange::with_end_date_on_first(d(2023, 1, 31)); // Jan 2023
        let mid_march = d(2023, 3, 17);
        let found = base.range_containing_date(mid_march);
        assert_eq!(found.start_date(), d(2023, 3, 1));
        assert_eq!(found.end_date(), d(2023, 3, 31));
        assert!(found.contains_date(mid_march));
    }

    // ============ Custom start day mode (e.g., 16th) ============

    #[test]
    fn with_end_date_and_start_day_16_spans_16_to_15_across_months() {
        // Non-leap year Feb -> Mar (end on 15th)
        let mar15 = MonthlyDateRange::with_end_date_and_start_day(d(2023, 3, 15), 16);
        assert_eq!(mar15.start_date(), d(2023, 2, 16));
        assert_eq!(mar15.end_date(), d(2023, 3, 15));
        assert_eq!(mar15.len(), 28); // Feb(13 days from 16..28) + Mar(15) = 28

        // Leap year Feb -> Mar
        let mar15_leap = MonthlyDateRange::with_end_date_and_start_day(d(2020, 3, 15), 16);
        assert_eq!(mar15_leap.start_date(), d(2020, 2, 16));
        assert_eq!(mar15_leap.end_date(), d(2020, 3, 15));
        assert_eq!(mar15_leap.len(), 29); // Feb has 14 days from 16..29 + 15 = 29

        // Mar -> Apr (31-day month to 15th next month)
        let apr15 = MonthlyDateRange::with_end_date_and_start_day(d(2023, 4, 15), 16);
        assert_eq!(apr15.start_date(), d(2023, 3, 16));
        assert_eq!(apr15.end_date(), d(2023, 4, 15));
        assert_eq!(apr15.len(), 31); // Mar(16..31 = 16) + Apr(1..15 = 15)
    }

    #[test]
    fn prior_and_next_link_for_start_day_16_including_year_boundary() {
        let dec15 = MonthlyDateRange::with_end_date_and_start_day(d(2023, 12, 15), 16);
        let next_range = dec15.next();
        assert_eq!(next_range.start_date(), d(2023, 12, 16));
        assert_eq!(next_range.end_date(), d(2024, 1, 15));

        let prior_range = dec15.prior();
        // Prior to 2023-11-16..2023-12-15 is 2023-10-16..2023-11-15
        assert_eq!(prior_range.start_date(), d(2023, 10, 16));
        assert_eq!(prior_range.end_date(), d(2023, 11, 15));

        // Going forward from Jan 15 range should get to Feb 15 range
        let jan15 = next_range;
        let feb15 = jan15.next();
        assert_eq!(feb15.start_date(), d(2024, 1, 16));
        assert_eq!(feb15.end_date(), d(2024, 2, 15));
    }

    #[test]
    fn range_containing_date_aligns_to_16_to_15_segments() {
        // Base: 2023-01-16 .. 2023-02-15 (constructed by passing end_date 2023-02-15)
        let base = MonthlyDateRange::with_end_date_and_start_day(d(2023, 2, 15), 16);

        // A date in May should map to 2023-04-16 .. 2023-05-15
        let target = d(2023, 5, 1);
        let found = base.range_containing_date(target);
        assert_eq!(found.start_date().day(), 16);
        assert_eq!(found.end_date().day(), 15);
        assert!(found.contains_date(target));
        assert_eq!(found.start_date(), d(2023, 4, 16));
        assert_eq!(found.end_date(), d(2023, 5, 15));

        // A date back in December should map correctly across year boundary
        let earlier = d(2022, 12, 20);
        let found_earlier = base.range_containing_date(earlier);
        assert_eq!(found_earlier.start_date().day(), 16);
        assert_eq!(found_earlier.end_date().day(), 15);
        assert!(found_earlier.contains_date(earlier));
    }
}
