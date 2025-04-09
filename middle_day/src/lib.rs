use chrono::{NaiveDate, Weekday as wd};
use chrono::Datelike;

pub fn middle_day(year: i32) -> Option<wd> {
    let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    
    if is_leap {
        return None;
    }

    let middle_day_ordinal = (365 + 1) / 2;
    
    let date = NaiveDate::from_yo(year, middle_day_ordinal as u32);
    
    Some(date.weekday())
}