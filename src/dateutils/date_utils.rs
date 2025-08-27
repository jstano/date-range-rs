use bigdecimal::BigDecimal;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};
use num_traits::FromPrimitive;
use std::cmp::{max, min};

/// Get the first day of the month for the given date.
pub fn first_day_of_month(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap()
}

/// Get the last day of the month for the given date.
pub fn last_day_of_month(date: NaiveDate) -> NaiveDate {
    let next_month = if date.month() == 12 { 1 } else { date.month() + 1 };
    let next_year = if date.month() == 12 { date.year() + 1 } else { date.year() };
    NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap() - Duration::days(1)
}

/// Add `days` to a date.
pub fn add_days(date: NaiveDate, days: i64) -> NaiveDate {
    date + Duration::days(days)
}

/// Subtract `days` from a date.
pub fn subtract_days(date: NaiveDate, days: i64) -> NaiveDate {
    date - Duration::days(days)
}

/// Add months to a date, safely handling month overflow.
pub fn add_months(date: NaiveDate, months: i32) -> NaiveDate {
    let mut year = date.year();
    let mut month = date.month() as i32 + months;
    while month > 12 {
        month -= 12;
        year += 1;
    }
    while month < 1 {
        month += 12;
        year -= 1;
    }
    let day = date.day().min(last_day_of_month(NaiveDate::from_ymd_opt(year, month as u32, 1).unwrap()).day());
    NaiveDate::from_ymd_opt(year, month as u32, day).unwrap()
}

/// Subtract months from a date.

pub fn subtract_months(date: NaiveDate, months: i32) -> NaiveDate {
    add_months(date, -months)
}

/// Add `years` to a date.
pub fn add_years(date: NaiveDate, years: i32) -> NaiveDate {
    add_months(date, years * 12)
}

/// Subtract `years` from a date.
pub fn subtract_years(date: NaiveDate, years: i32) -> NaiveDate {
    subtract_months(date, years * 12)
}

pub fn with_year_safe(date: NaiveDate, year: i32) -> NaiveDate {
    let month = date.month();
    let day = date.day();

    // Check if the day is valid in the new year
    if let Some(new_date) = NaiveDate::from_ymd_opt(year, month, day) {
        new_date
    } else {
        // If invalid (e.g., Feb 29 on a non-leap year), use the last valid day of the month
        let last_day = last_day_of_month_year(month, year);
        NaiveDate::from_ymd_opt(year, month, last_day).unwrap()
    }
}

fn last_day_of_month_year(month: u32, year: i32) -> u32 {
    use chrono::NaiveDate;
    // Next month, day 0 is the last day of this month
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_month_year = if month == 12 { year + 1 } else { year };
    NaiveDate::from_ymd_opt(next_month_year, next_month, 1).unwrap()
        .pred_opt().unwrap()
        .day()
}

/// Return the earlier of two NaiveDateTime values.
/// If equal, returns time1.
pub fn earliest(time1: NaiveDateTime, time2: NaiveDateTime) -> NaiveDateTime {
    min(time1, time2)
}

/// Return the earlier of two optional NaiveDateTime values.
/// If one is None, returns the other. If both are None, returns None.
/// If equal, returns time1.
pub fn earliest_opt(time1: Option<NaiveDateTime>, time2: Option<NaiveDateTime>) -> Option<NaiveDateTime> {
    match (time1, time2) {
        (None, None) => None,
        (Some(t1), None) => Some(t1),
        (None, Some(t2)) => Some(t2),
        (Some(t1), Some(t2)) => Some(if t1 <= t2 { t1 } else { t2 }),
    }
}

/// Return the latter of two NaiveDateTime values.
/// If equal, returns time1.
pub fn latest(time1: NaiveDateTime, time2: NaiveDateTime) -> NaiveDateTime {
    max(time1, time2)
}

/// Return the latter of two optional NaiveDateTime values.
/// - If one is None, returns the other.
/// - If both are None, returns None.
/// - If equal, returns time1 (mirrors Java's compareTo >= behavior).
pub fn latest_opt(time1: Option<NaiveDateTime>, time2: Option<NaiveDateTime>) -> Option<NaiveDateTime> {
    match (time1, time2) {
        (None, None) => None,
        (Some(t1), None) => Some(t1),
        (None, Some(t2)) => Some(t2),
        (Some(t1), Some(t2)) => Some(if t1 >= t2 { t1 } else { t2 }),
    }
}

/// Returns whole hours between start and end (truncating toward zero).
pub fn duration_in_hours(start: NaiveDateTime, end: NaiveDateTime) -> i32 {
    let seconds = (end - start).num_seconds();
    (seconds as i32) / 3_600
}

/// Returns whole minutes between start and end (truncating toward zero).
pub fn duration_in_minutes(start: NaiveDateTime, end: NaiveDateTime) -> i32 {
    let seconds = (end - start).num_seconds();
    (seconds as i32) / 60
}

/// Returns whole seconds between start and end (truncating toward zero).
pub fn duration_in_seconds(start: NaiveDateTime, end: NaiveDateTime) -> i32 {
    (end - start).num_seconds() as i32
}

/// Returns duration between start and end as fractional seconds,
/// rounded to 4 significant digits (similar to Java BigDecimal with MathContext(4)).
pub fn duration_in_fractional_seconds(start: NaiveDateTime, end: NaiveDateTime) -> f64 {
    let seconds = (end - start).num_milliseconds() as f64 / 1_000.0;
    round_to_sig_figs(seconds, 4)
}

/// Returns duration between start and end as fractional hours.
pub fn duration_in_fractional_hours(start: NaiveDateTime, end: NaiveDateTime) -> f64 {
    (end - start).num_seconds() as f64 / 3_600.0
}

/// Round a floating-point value to the given number of significant figures.
fn round_to_sig_figs(value: f64, sig_figs: u32) -> f64 {
    if value == 0.0 {
        return 0.0;
    }
    let abs = value.abs();
    let order = abs.log10().floor();
    let scale = 10f64.powf((sig_figs as f64 - 1.0) - order);
    (value * scale).round() / scale
}

/// Returns duration between start and end as fractional seconds (BigDecimal).
/// Computed exactly from milliseconds: seconds = millis / 1000.
pub fn duration_in_fractional_seconds_bd(start: NaiveDateTime, end: NaiveDateTime) -> BigDecimal {
    let millis = (end - start).num_milliseconds();
    let ms_bd = BigDecimal::from_i64(millis).unwrap();
    ms_bd / BigDecimal::from(1_000i32)
}

/// Returns duration between start and end as fractional hours (BigDecimal).
/// Computed from whole seconds: hours = seconds / 3600.
pub fn duration_in_fractional_hours_bd(start: NaiveDateTime, end: NaiveDateTime) -> BigDecimal {
    let secs = (end - start).num_seconds();
    let sec_bd = BigDecimal::from_i64(secs).unwrap();
    sec_bd / BigDecimal::from(3_600i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::rstest;

    #[test]
    fn test_first_day_of_month() {
        let date = first_day_of_month(NaiveDate::from_ymd_opt(2025, 8, 20).unwrap());

        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), 8);
        assert_eq!(date.day(), 1);
    }

    #[rstest]
    #[case(NaiveDate::from_ymd_opt(2025, 8, 20).unwrap(), 2025, 8, 31)]
    #[case(NaiveDate::from_ymd_opt(2025, 12, 15).unwrap(), 2025, 12, 31)]
    #[case(NaiveDate::from_ymd_opt(2025, 2, 15).unwrap(), 2025, 2, 28)]
    #[case(NaiveDate::from_ymd_opt(2028, 2, 15).unwrap(), 2028, 2, 29)]
    fn test_last_day_of_month(
        #[case] input: NaiveDate,
        #[case] expected_year: i32,
        #[case] expected_month: u32,
        #[case] expected_day: u32,
    ) {
        let result = last_day_of_month(input);
        assert_eq!(result.year(), expected_year, "Failed for {:?}", input);
        assert_eq!(result.month(), expected_month, "Failed for {:?}", input);
        assert_eq!(result.day(), expected_day, "Failed for {:?}", input);
    }

    #[rstest]
    #[case(NaiveDate::from_ymd_opt(2025, 8, 20).unwrap(), 1, 2025, 8, 21)]
    #[case(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(), 2, 2026, 1, 2)]
    #[case(NaiveDate::from_ymd_opt(2025, 2, 28).unwrap(), 1, 2025, 3, 1)]
    #[case(NaiveDate::from_ymd_opt(2028, 2, 28).unwrap(), 1, 2028, 2, 29)]
    fn test_add_days(
        #[case] input: NaiveDate,
        #[case] days: i64,
        #[case] expected_year: i32,
        #[case] expected_month: u32,
        #[case] expected_day: u32,
    ) {
        let result = add_days(input, days);
        assert_eq!(result.year(), expected_year, "Failed for {:?}", input);
        assert_eq!(result.month(), expected_month, "Failed for {:?}", input);
        assert_eq!(result.day(), expected_day, "Failed for {:?}", input);
    }

    #[rstest]
    #[case(NaiveDate::from_ymd_opt(2025, 8, 20).unwrap(), 1, 2025, 8, 19)]
    #[case(NaiveDate::from_ymd_opt(2026, 1, 2).unwrap(), 2, 2025, 12, 31)]
    #[case(NaiveDate::from_ymd_opt(2025, 3, 1).unwrap(), 1, 2025, 2, 28)]
    #[case(NaiveDate::from_ymd_opt(2028, 2, 29).unwrap(), 1, 2028, 2, 28)]
    fn test_subtract_days(
        #[case] input: NaiveDate,
        #[case] days: i64,
        #[case] expected_year: i32,
        #[case] expected_month: u32,
        #[case] expected_day: u32,
    ) {
        let result = subtract_days(input, days);
        assert_eq!(result.year(), expected_year, "Failed for {:?}", input);
        assert_eq!(result.month(), expected_month, "Failed for {:?}", input);
        assert_eq!(result.day(), expected_day, "Failed for {:?}", input);
    }

    #[rstest]
    #[case(NaiveDate::from_ymd_opt(2025, 8, 20).unwrap(), 1, 2025, 9, 20)]
    #[case(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(), 2, 2026, 2, 28)]
    #[case(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(), 2, 2026, 2, 28)]
    #[case(NaiveDate::from_ymd_opt(2025, 2, 28).unwrap(), 1, 2025, 3, 28)]
    #[case(NaiveDate::from_ymd_opt(2028, 2, 28).unwrap(), 1, 2028, 3, 28)]
    #[case(NaiveDate::from_ymd_opt(2028, 2, 29).unwrap(), 1, 2028, 3, 29)]
    fn test_add_months(
        #[case] input: NaiveDate,
        #[case] months: i32,
        #[case] expected_year: i32,
        #[case] expected_month: u32,
        #[case] expected_day: u32,
    ) {
        let result = add_months(input, months);
        assert_eq!(result.year(), expected_year, "Failed for {:?}", input);
        assert_eq!(result.month(), expected_month, "Failed for {:?}", input);
        assert_eq!(result.day(), expected_day, "Failed for {:?}", input);
    }

    #[rstest]
    #[case(NaiveDate::from_ymd_opt(2025, 9, 20).unwrap(), 1, 2025, 8, 20)]
    #[case(NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(), 2, 2025, 12, 28)]
    #[case(NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(), 2, 2025, 12, 28)]
    #[case(NaiveDate::from_ymd_opt(2025, 3, 28).unwrap(), 1, 2025, 2, 28)]
    #[case(NaiveDate::from_ymd_opt(2028, 3, 28).unwrap(), 1, 2028, 2, 28)]
    #[case(NaiveDate::from_ymd_opt(2028, 3, 29).unwrap(), 1, 2028, 2, 29)]
    fn test_subtract_months(
        #[case] input: NaiveDate,
        #[case] months: i32,
        #[case] expected_year: i32,
        #[case] expected_month: u32,
        #[case] expected_day: u32,
    ) {
        let result = subtract_months(input, months);
        assert_eq!(result.year(), expected_year, "Failed for {:?}", input);
        assert_eq!(result.month(), expected_month, "Failed for {:?}", input);
        assert_eq!(result.day(), expected_day, "Failed for {:?}", input);
    }

    #[rstest]
    #[case(NaiveDate::from_ymd_opt(2025, 8, 20).unwrap(), 1, 2026, 8, 20)]
    #[case(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(), 2, 2027, 12, 31)]
    #[case(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(), 3, 2028, 12, 31)]
    #[case(NaiveDate::from_ymd_opt(2025, 2, 28).unwrap(), 1, 2026, 2, 28)]
    #[case(NaiveDate::from_ymd_opt(2028, 2, 29).unwrap(), 1, 2029, 2, 28)]
    fn test_add_years(
        #[case] input: NaiveDate,
        #[case] years: i32,
        #[case] expected_year: i32,
        #[case] expected_month: u32,
        #[case] expected_day: u32,
    ) {
        let result = add_years(input, years);
        assert_eq!(result.year(), expected_year, "Failed for {:?}", input);
        assert_eq!(result.month(), expected_month, "Failed for {:?}", input);
        assert_eq!(result.day(), expected_day, "Failed for {:?}", input);
    }

    #[rstest]
    #[case(NaiveDate::from_ymd_opt(2026, 8, 20).unwrap(), 1, 2025, 8, 20)]
    #[case(NaiveDate::from_ymd_opt(2027, 12, 31).unwrap(), 2, 2025, 12, 31)]
    #[case(NaiveDate::from_ymd_opt(2028, 12, 31).unwrap(), 3, 2025, 12, 31)]
    #[case(NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(), 1, 2025, 2, 28)]
    #[case(NaiveDate::from_ymd_opt(2029, 2, 28).unwrap(), 1, 2028, 2, 28)]
    fn test_subtract_years(
        #[case] input: NaiveDate,
        #[case] years: i32,
        #[case] expected_year: i32,
        #[case] expected_month: u32,
        #[case] expected_day: u32,
    ) {
        let result = subtract_years(input, years);
        assert_eq!(result.year(), expected_year, "Failed for {:?}", input);
        assert_eq!(result.month(), expected_month, "Failed for {:?}", input);
        assert_eq!(result.day(), expected_day, "Failed for {:?}", input);
    }
}
