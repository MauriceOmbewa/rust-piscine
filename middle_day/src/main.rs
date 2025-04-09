// In your Cargo.toml, be sure to include:
// [dependencies]
// chrono = "0.4"

use chrono::{NaiveDate, Weekday as wd};
use chrono::Datelike;

pub fn middle_day(year: i32) -> Option<wd> {
    // Determine if the year is a leap year.
    // A leap year has an even number of days (366) and is defined as:
    // divisible by 4, except if divisible by 100 unless also divisible by 400.
    let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    
    // If the year is a leap year, return None.
    if is_leap {
        return None;
    }

    // For non-leap years (365 days), the middle day is:
    // (365 + 1) / 2 = 183 (1-indexed day number)
    let middle_day_ordinal = (365 + 1) / 2;
    
    // Create a NaiveDate from the year and the day-of-year.
    let date = NaiveDate::from_yo(year, middle_day_ordinal as u32);
    
    // Return the weekday of that date wrapped in Some.
    Some(date.weekday())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_middle_day_non_leap() {
        // Year 1022 is a non-leap year.
        // Expected output: Tue
        let weekday = middle_day(1022).unwrap();
        assert_eq!(format!("{:?}", weekday), "Tue");
    }
    
    #[test]
    fn test_middle_day_leap() {
        // For a leap year, we should return None.
        assert!(middle_day(2020).is_none());
    }
}

fn main() {
    // The following will print the weekday of the middle day of the year 1022.
    println!("{:?}", middle_day(1022).unwrap());
}
