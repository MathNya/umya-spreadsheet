use chrono::{Duration, NaiveDateTime};

pub const CALENDAR_WINDOWS_1900: &str = "1900";
pub const CALENDAR_MAC_1904: &str = "1904";

pub fn excel_to_date_time_object(
    excel_timestamp: &f64,
    time_zone: Option<String>,
) -> NaiveDateTime {
    let _time_zone = match time_zone {
        Some(v) => v,
        None => get_default_timezone(),
    };

    let base_date = if excel_timestamp < &1f64 {
        // Unix timestamp base date
        NaiveDateTime::parse_from_str("1970-01-01 00:00:00", "%Y-%m-%d %T").unwrap()
    } else {
        // Allow adjustment for 1900 Leap Year in MS Excel
        if excel_timestamp < &60f64 {
            NaiveDateTime::parse_from_str("1899-12-31 00:00:00", "%Y-%m-%d %T").unwrap()
        } else {
            NaiveDateTime::parse_from_str("1899-12-30 00:00:00", "%Y-%m-%d %T").unwrap()
        }
    };

    let days = excel_timestamp.floor();
    let part_day = excel_timestamp - days;
    let hours = (part_day * 24.0).floor();
    let part_day = part_day * 24f64 - hours;
    let minutes = (part_day * 60f64).floor();
    let part_day = part_day * 60f64 - minutes;
    let seconds = (part_day * 60f64).round();

    base_date
        + Duration::days(days as i64)
        + Duration::hours(hours as i64)
        + Duration::minutes(minutes as i64)
        + Duration::seconds(seconds as i64)
}

#[inline]
fn get_default_timezone() -> String {
    String::from("UTC")
}

#[inline]
pub fn convert_date(
    year: i32,
    month: i32,
    day: i32,
    hours: i32,
    minutes: i32,
    seconds: i32,
) -> f64 {
    convert_date_windows_1900(year, month, day, hours, minutes, seconds)
}

#[inline]
pub fn convert_date_windows_1900(
    year: i32,
    month: i32,
    day: i32,
    hours: i32,
    minutes: i32,
    seconds: i32,
) -> f64 {
    convert_date_crate(year, month, day, hours, minutes, seconds, true)
}

#[inline]
pub fn convert_date_mac_1904(
    year: i32,
    month: i32,
    day: i32,
    hours: i32,
    minutes: i32,
    seconds: i32,
) -> f64 {
    convert_date_crate(year, month, day, hours, minutes, seconds, false)
}

pub fn convert_date_crate(
    year: i32,
    month: i32,
    day: i32,
    hours: i32,
    minutes: i32,
    seconds: i32,
    is_calendar_windows_1900: bool,
) -> f64 {
    let mut year = year;
    let mut month = month;

    let myexcel_base_date = if is_calendar_windows_1900 {
        // Adjust for Excel 1900 leap year bug
        if year == 1900 && month <= 2 {
            2415020 - 1 // No leap year adjustment
        } else {
            2415020
        }
    } else {
        2416481 // Excel Mac 1904 system base date
    };

    // Julian base date adjustment
    if month > 2 {
        month -= 3;
    } else {
        month += 9;
        year -= 1;
    }

    // Calculate Julian day number
    let century = year / 100;
    let decade = year % 100;

    let excel_date = (146097 * century) / 4
        + (1461 * decade) / 4
        + (153 * month + 2) / 5
        + day
        + 1721119
        - myexcel_base_date;

    // Calculate time as fraction of a day
    let excel_time = (hours * 3600 + minutes * 60 + seconds) as f64 / 86400.0;

    excel_date as f64 + excel_time
}
