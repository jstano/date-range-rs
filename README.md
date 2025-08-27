```markdown
# Date Range Utilities

A small Rust crate for working with reusable, navigable date ranges (yearly, quarterly, monthly, weekly, etc.) based on chrono’s NaiveDate. Each range exposes its start and end dates and supports moving to the prior or next adjacent range with consistent logic.

Highlights:
— Simple construction from either a start date or an end date—Inclusive ranges (start and end are included)
- Prior/next navigation across ranges
- Leap-year-aware calculations via chrono

Minimum Supported Rust Version (MSRV): 1.89.0

## Installation

Add the crate to your Cargo.toml dependencies:
```
toml
[dependencies]
date-range = "0.x"
chrono = "0.4"
```
If you are developing locally, you can use a path dependency instead:
```
toml
[dependencies]
date-range = { path = "./path/to/your/local/crate" }
chrono = "0.4"
```
## Quick start

Create an annual date range and navigate to the prior/next years.
```
rust
use chrono::NaiveDate;
use date_range::daterange::annual_date_range::AnnualDateRange;

fn main() {
// Create a range for the calendar year 2023 from a start date
let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
let dr = AnnualDateRange::with_start_date(start);
assert_eq!(dr.start_date(), NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
assert_eq!(dr.end_date(), NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());

    // Navigate to adjacent years
    let prior = dr.prior();
    let next = dr.next();
    assert_eq!(prior.start_date(), NaiveDate::from_ymd_opt(2022, 1, 1).unwrap());
    assert_eq!(next.end_date(), NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());

    // Construct from an end date
    let end = NaiveDate::from_ymd_opt(2021, 2, 28).unwrap();
    let leap_span = AnnualDateRange::with_end_date(end);
    assert_eq!(leap_span.start_date(), NaiveDate::from_ymd_opt(2020, 2, 29).unwrap());
}
```
Leap-year correctness example:
```
rust
use chrono::NaiveDate;
use date_range::daterange::annual_date_range::AnnualDateRange;

let dr = AnnualDateRange::with_start_date(NaiveDate::from_ymd_opt(2020, 2, 29).unwrap());
assert_eq!(dr.end_date(), NaiveDate::from_ymd_opt(2021, 2, 28).unwrap());
```
## Other ranges

The crate provides additional period types, each with similar APIs:
- Weekly
- Bi-weekly
- Semi-monthly
- Monthly
- Quarterly
- Semi-annual
- Annual

Typically you can:
- Build a range from a start date or from an end date
- Retrieve start_date() and end_date()
- Move to prior() and next() adjacent ranges

## Testing

Run the unit tests:
```
bash
cargo test
```
## License

This project is open source. See the repository for license details.
```
