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
        // MS Excel calendar base dates
        if CALENDAR_WINDOWS_1900 == CALENDAR_WINDOWS_1900 {
            // Allow adjustment for 1900 Leap Year in MS Excel
            if excel_timestamp < &60f64 {
                NaiveDateTime::parse_from_str("1899-12-31 00:00:00", "%Y-%m-%d %T").unwrap()
            } else {
                NaiveDateTime::parse_from_str("1899-12-30 00:00:00", "%Y-%m-%d %T").unwrap()
            }
        } else {
            NaiveDateTime::parse_from_str("1904-01-01 00:00:00", "%Y-%m-%d %T").unwrap()
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
