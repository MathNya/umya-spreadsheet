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

    let mut base_date = if excel_timestamp < &1f64 {
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

    base_date += Duration::days(days as i64);
    base_date += Duration::hours(hours as i64);
    base_date += Duration::minutes(minutes as i64);
    base_date += Duration::seconds(seconds as i64);

    base_date
}

fn get_default_timezone() -> String {
    String::from("UTC")
}

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

fn convert_date_crate(
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
    let mut myexcel_base_date = 0;
    let mut excel1900is_leap_year = 0;

    if is_calendar_windows_1900 {
        excel1900is_leap_year = 1;
        if &year == &1900 && &month <= &2 {
            excel1900is_leap_year = 0;
        }
        myexcel_base_date = 2415020;
    } else {
        myexcel_base_date = 2416481;
    }

    // Julian base date Adjustment
    if month > 2 {
        month -= 3;
    } else {
        month += 9;
        year -= 1;
    }

    // Calculate the Julian Date, then subtract the Excel base date (JD 2415020 = 31-Dec-1899 Giving Excel Date of 0)
    let century = (year.to_string()[0..2]).parse::<i32>().unwrap();
    let decade = (year.to_string()[2..4]).parse::<i32>().unwrap();

    let excel_date = ((146097 * century) / 4) as i32
        + ((1461 * decade) / 4) as i32
        + ((153 * month + 2) / 5) as i32
        + day
        + 1721119
        - myexcel_base_date
        + excel1900is_leap_year;
    let excel_time = ((hours * 3600) + (minutes * 60) + seconds) as f64 / 86400 as f64;

    return (excel_date as f64 + excel_time) as f64;
}
