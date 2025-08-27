use crate::daterange::date_range::DateRange;
use chrono::{Datelike, Duration, NaiveDate, Weekday};

pub struct BiWeeklyDateRange;

impl BiWeeklyDateRange {
    pub fn with_start_date(start_date: NaiveDate) -> DateRange {
        let end = start_date + Duration::days(13);
        DateRange::new(start_date, end)
    }

    pub fn with_end_date(end_date: NaiveDate) -> DateRange {
        let start = end_date - Duration::days(13);
        DateRange::new(start, end_date)
    }

    pub fn with_target_date(target: NaiveDate, end_day: Weekday) -> DateRange {
        let offset = calculate_day_of_week_offset(target, end_day);
        let end = target + Duration::days(offset as i64);
        let start = end - Duration::days(13);
        DateRange::new(start, end)
    }
}

fn calculate_day_of_week_offset(date: NaiveDate, end_day: Weekday) -> i64 {
    let mut offset = end_day.num_days_from_monday() as i64 - date.weekday().num_days_from_monday() as i64;
    if offset < 0 {
        offset += 7;
    }
    offset
}
