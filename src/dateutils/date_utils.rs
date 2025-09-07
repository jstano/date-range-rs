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
    use bigdecimal::FromPrimitive as _;
    use chrono::NaiveDate;
    use num_traits::ToPrimitive;
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

    #[test]
    fn test_with_year_safe() {
        let feb29_2028 = NaiveDate::from_ymd_opt(2028, 2, 29).unwrap();
        // Non-leap year fallback to Feb 28
        let y2029 = with_year_safe(feb29_2028, 2029);
        assert_eq!(y2029, NaiveDate::from_ymd_opt(2029, 2, 28).unwrap());
        // Leap year remains Feb 29
        let y2032 = with_year_safe(feb29_2028, 2032);
        assert_eq!(y2032, NaiveDate::from_ymd_opt(2032, 2, 29).unwrap());
        // A normal date remains same day
        let aug20_2025 = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap();
        let y2030 = with_year_safe(aug20_2025, 2030);
        assert_eq!(y2030, NaiveDate::from_ymd_opt(2030, 8, 20).unwrap());
    }

    #[test]
    fn test_earliest_and_latest() {
        use chrono::NaiveDate;
        use chrono::NaiveDateTime;
        let t1: NaiveDateTime = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap().and_hms_opt(10, 0, 0).unwrap();
        let t2: NaiveDateTime = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap().and_hms_opt(12, 0, 0).unwrap();
        assert_eq!(earliest(t1, t2), t1);
        assert_eq!(earliest(t2, t1), t1);
        assert_eq!(earliest(t1, t1), t1);
        assert_eq!(latest(t1, t2), t2);
        assert_eq!(latest(t2, t1), t2);
        assert_eq!(latest(t1, t1), t1);
    }

    #[test]
    fn test_earliest_latest_opt() {
        use chrono::NaiveDate;
        use chrono::NaiveDateTime;
        let t1: NaiveDateTime = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap().and_hms_opt(10, 0, 0).unwrap();
        let t2: NaiveDateTime = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap().and_hms_opt(12, 0, 0).unwrap();
        // None combinations
        assert_eq!(earliest_opt(None, None), None);
        assert_eq!(earliest_opt(Some(t1), None), Some(t1));
        assert_eq!(earliest_opt(None, Some(t2)), Some(t2));
        assert_eq!(earliest_opt(Some(t1), Some(t2)), Some(t1));
        assert_eq!(earliest_opt(Some(t2), Some(t1)), Some(t1));
        assert_eq!(earliest_opt(Some(t1), Some(t1)), Some(t1));
        // latest_opt mirrors >= behavior
        assert_eq!(latest_opt(None, None), None);
        assert_eq!(latest_opt(Some(t1), None), Some(t1));
        assert_eq!(latest_opt(None, Some(t2)), Some(t2));
        assert_eq!(latest_opt(Some(t1), Some(t2)), Some(t2));
        assert_eq!(latest_opt(Some(t2), Some(t1)), Some(t2));
        assert_eq!(latest_opt(Some(t1), Some(t1)), Some(t1));
    }

    #[test]
    fn test_durations_whole_units() {
        use chrono::NaiveDate;
        use chrono::NaiveDateTime;
        let start: NaiveDateTime = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap().and_hms_opt(10, 0, 0).unwrap();
        let end: NaiveDateTime = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap().and_hms_opt(12, 34, 56).unwrap();
        assert_eq!(duration_in_hours(start, end), 2); // 2h 34m -> truncates to 2
        assert_eq!(duration_in_minutes(start, end), 154); // 2h*60 + 34m = 154
        assert_eq!(duration_in_seconds(start, end), 9296); // 2*3600 + 34*60 + 56 = 9296
        // Negative range truncation toward zero due to integer division semantics
        assert_eq!(duration_in_hours(end, start), -2);
        assert_eq!(duration_in_minutes(end, start), -154);
        assert_eq!(duration_in_seconds(end, start), -9296);
        // Zero duration
        assert_eq!(duration_in_hours(start, start), 0);
        assert_eq!(duration_in_minutes(start, start), 0);
        assert_eq!(duration_in_seconds(start, start), 0);
    }

    #[test]
    fn test_fractional_seconds_rounding() {
        use chrono::NaiveDate;
        use chrono::NaiveDateTime;
        let s: NaiveDateTime = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap().and_hms_milli_opt(10, 0, 0, 0).unwrap();
        let e1 = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap().and_hms_milli_opt(10, 0, 1, 234).unwrap();
        // 1.234 seconds -> rounded to 4 sig figs stays 1.234
        let v1 = duration_in_fractional_seconds(s, e1);
        assert!((v1 - 1.234).abs() < 1e-12, "{}", v1);
        // 0.0012345 should round to 0.001235 with 4 sig figs; use 0.001 seconds for exact
        let e2 = s + chrono::Duration::microseconds(1000); // 0.001 s
        let v2 = duration_in_fractional_seconds(s, e2);
        assert!((v2 - 0.001).abs() < 1e-12, "{}", v2);
        // Negative direction should keep sign and rounding
        let v3 = duration_in_fractional_seconds(e1, s);
        assert!((v3 + 1.234).abs() < 1e-12, "{}", v3);
    }

    #[test]
    fn test_fractional_hours() {
        use chrono::NaiveDate;
        use chrono::NaiveDateTime;
        let s: NaiveDateTime = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap().and_hms_opt(10, 0, 0).unwrap();
        let e = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap().and_hms_opt(12, 30, 0).unwrap();
        let v = duration_in_fractional_hours(s, e);
        assert!((v - 2.5).abs() < 1e-12, "{}", v);
        let vn = duration_in_fractional_hours(e, s);
        assert!((vn + 2.5).abs() < 1e-12, "{}", vn);
    }

    #[test]
    fn test_bigdecimal_fractional_durations() {
        use chrono::NaiveDate;
        use chrono::NaiveDateTime;
        let s: NaiveDateTime = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap().and_hms_milli_opt(10, 0, 0, 0).unwrap();
        let e = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap().and_hms_milli_opt(10, 1, 2, 500).unwrap(); // 62.5 seconds
        let secs_bd = duration_in_fractional_seconds_bd(s, e);
        // Compare via numeric equality by converting to f64 (safe here due to simple value)
        assert!((secs_bd.to_f64().unwrap() - 62.5).abs() < 1e-12);
        let hours_bd = duration_in_fractional_hours_bd(s, e);
        // The implementation computes hours from whole seconds (truncating 62.5s -> 62s)
        let expected_hours_bd = BigDecimal::from(62) / BigDecimal::from(3600);
        assert_eq!(hours_bd, expected_hours_bd);
    }
    
    #[test]
    fn test_round_to_sig_figs_zero_branch() {
        // Exact zero should return zero regardless of sig figs
        assert_eq!(round_to_sig_figs(0.0, 1), 0.0);
        assert_eq!(round_to_sig_figs(0.0, 4), 0.0);
        assert_eq!(round_to_sig_figs(0.0, 10), 0.0);
        // Negative zero input should also return canonical 0.0
        let neg_zero: f64 = -0.0;
        let r = round_to_sig_figs(neg_zero, 4);
        assert_eq!(r, 0.0);
        // Ensure it's finite (not NaN/Inf)
        assert!(r.is_finite());
    }

}
