use chrono::{
    Duration,
    NaiveDateTime,
};
use num_traits::cast;

pub const CALENDAR_WINDOWS_1900: &str = "1900";
pub const CALENDAR_MAC_1904: &str = "1904";
pub const DEFAULT_TIMEZONE: &str = "UTC";

/// Converts an Excel timestamp to a `NaiveDateTime` object.
///
/// This function takes an Excel timestamp, which is a numeric representation of
/// a date and time, and converts it into a `NaiveDateTime` object. The integer
/// part of the timestamp represents the number of days since a base date, while
/// the fractional part represents the time of day.
///
/// # Parameters
///
/// - `excel_timestamp: f64` The Excel timestamp to be converted. If the value
///   is less than 1, it is treated as a Unix timestamp (seconds since January
///   1, 1970). If the value is greater than or equal to 1, it is treated as an
///   Excel date.
///
/// - `time_zone: Option<String>` An optional string representing the desired
///   time zone. If `None`, the function will use a default time zone obtained
///   from the `get_default_timezone()` function.
///
/// # Returns
///
/// Returns a `NaiveDateTime` object representing the converted date and time.
/// This object does not contain any timezone information.
///
/// # Behavior
///
/// - If `excel_timestamp` is less than 1, it is interpreted as a Unix
///   timestamp, using January 1, 1970, as the base date.
/// - If `excel_timestamp` is between 1 and 60, the function accounts for the
///   1900 leap year bug in Excel by using December 31, 1899, as the base date.
/// - For `excel_timestamp` values of 60 or greater, it uses December 30, 1899,
///   as the base date.
///
/// # Example
///
/// ```rust
/// let timestamp = 44204.5; // Represents 2021-01-01 12:00:00
/// let date_time = excel_to_date_time_object(timestamp, None);
/// assert_eq!(date_time.year(), 2021);
/// assert_eq!(date_time.month(), 1);
/// assert_eq!(date_time.day(), 1);
/// assert_eq!(date_time.hour(), 12);
/// assert_eq!(date_time.minute(), 0);
/// ```
///
/// # Panics
///
/// This function will panic if the parsing of the base date fails. Ensure that
/// the input timestamp is valid and within the expected range.
#[must_use]
pub fn excel_to_date_time_object(excel_timestamp: f64, time_zone: Option<String>) -> NaiveDateTime {
    let _time_zone = match time_zone {
        Some(v) => v,
        None => DEFAULT_TIMEZONE.to_owned(),
    };

    let base_date = if excel_timestamp < 1f64 {
        // Unix timestamp base date
        NaiveDateTime::parse_from_str("1970-01-01 00:00:00", "%Y-%m-%d %T").unwrap()
    } else {
        // Allow adjustment for 1900 Leap Year in MS Excel
        if excel_timestamp < 60f64 {
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
        + Duration::days(cast(days).unwrap())
        + Duration::hours(cast(hours).unwrap())
        + Duration::minutes(cast(minutes).unwrap())
        + Duration::seconds(cast(seconds).unwrap())
}

/// Converts a date and time to an Excel timestamp using the Windows 1900 date
/// system.
///
/// This function takes individual components of a date and time (year, month,
/// day, hours, minutes, and seconds) and converts them into an Excel timestamp.
/// The conversion is based on the Windows 1900 date system, which accounts for
/// the 1900 leap year bug.
///
/// # Parameters
///
/// - `year: i32` The year component of the date.
///
/// - `month: i32` The month component of the date (1-12).
///
/// - `day: i32` The day component of the date (1-31).
///
/// - `hours: i32` The hour component of the time (0-23).
///
/// - `minutes: i32` The minute component of the time (0-59).
///
/// - `seconds: i32` The second component of the time (0-59).
///
/// # Returns
///
/// Returns an `f64` representing the corresponding Excel timestamp.
///
/// # Example
///
/// ```rust
/// let timestamp = convert_date(2021, 1, 1, 12, 0, 0);
/// assert_eq!(timestamp, 44204.5); // Represents 2021-01-01 12:00:00
/// ```
///
/// # Panics
///
/// This function may panic if the provided date and time components are
/// invalid.
#[inline]
#[must_use]
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

/// Converts a date and time to an Excel timestamp using the Windows 1900 date
/// system.
///
/// This function takes individual components of a date and time (year, month,
/// day, hours, minutes, and seconds) and converts them into an Excel timestamp
/// specifically for the Windows 1900 date system, which includes the 1900 leap
/// year bug.
///
/// # Parameters
///
/// - `year: i32` The year component of the date.
///
/// - `month: i32` The month component of the date (1-12).
///
/// - `day: i32` The day component of the date (1-31).
///
/// - `hours: i32` The hour component of the time (0-23).
///
/// - `minutes: i32` The minute component of the time (0-59).
///
/// - `seconds: i32` The second component of the time (0-59).
///
/// # Returns
///
/// Returns an `f64` representing the corresponding Excel timestamp.
///
/// # Example
///
/// ```rust
/// let timestamp = convert_date_windows_1900(2021, 1, 1, 12, 0, 0);
/// assert_eq!(timestamp, 44204.5); // Represents 2021-01-01 12:00:00
/// ```
///
/// # Panics
///
/// This function may panic if the provided date and time components are
/// invalid.
#[inline]
#[must_use]
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

/// Converts a date and time to an Excel timestamp using the Mac 1904 date
/// system.
///
/// This function takes individual components of a date and time (year, month,
/// day, hours, minutes, and seconds) and converts them into an Excel timestamp
/// specifically for the Mac 1904 date system, which has a different base date
/// compared to the Windows 1900 date system.
///
/// # Parameters
///
/// - `year: i32` The year component of the date.
///
/// - `month: i32` The month component of the date (1-12).
///
/// - `day: i32` The day component of the date (1-31).
///
/// - `hours: i32` The hour component of the time (0-23).
///
/// - `minutes: i32` The minute component of the time (0-59).
///
/// - `seconds: i32` The second component of the time (0-59).
///
/// # Returns
///
/// Returns an `f64` representing the corresponding Excel timestamp for the Mac
/// 1904 date system.
///
/// # Example
///
/// ```rust
/// let timestamp = convert_date_mac_1904(2021, 1, 1, 12, 0, 0);
/// assert_eq!(timestamp, 44204.5); // Represents 2021-01-01 12:00:00
/// ```
///
/// # Panics
///
/// This function may panic if the provided date and time components are
/// invalid.
#[inline]
#[must_use]
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

/// Converts a date and time to an Excel timestamp based on the specified
/// calendar system.
///
/// This function takes individual components of a date and time (year, month,
/// day, hours, minutes, and seconds) and converts them into an Excel timestamp.
/// The conversion can be based on either the Windows 1900 date system or the
/// Mac 1904 date system, depending on the value of the
/// `is_calendar_windows_1900` parameter.
///
/// # Parameters
///
/// - `year: i32` The year component of the date.
///
/// - `month: i32` The month component of the date (1-12).
///
/// - `day: i32` The day component of the date (1-31).
///
/// - `hours: i32` The hour component of the time (0-23).
///
/// - `minutes: i32` The minute component of the time (0-59).
///
/// - `seconds: i32` The second component of the time (0-59).
///
/// - `is_calendar_windows_1900: bool` A boolean indicating whether to use the
///   Windows 1900 date system (`true`) or the Mac 1904 date system (`false`).
///
/// # Returns
///
/// Returns an `f64` representing the corresponding Excel timestamp for the
/// specified calendar system.
///
/// # Example
///
/// ```rust
/// let timestamp_windows = convert_date_crate(2021, 1, 1, 12, 0, 0, true);
/// assert_eq!(timestamp_windows, 44204.5); // Represents 2021-01-01 12:00:00 in Windows 1900 system
///
/// let timestamp_mac = convert_date_crate(2021, 1, 1, 12, 0, 0, false);
/// assert_eq!(timestamp_mac, 44204.5); // Represents 2021-01-01 12:00:00 in Mac 1904 system
/// ```
///
/// # Panics
///
/// This function may panic if the provided date and time components are
/// invalid.
#[must_use]
pub fn convert_date_crate(
    year: i32,
    month: i32,
    day: i32,
    hours: i32,
    minutes: i32,
    seconds: i32,
    is_calendar_windows_1900: bool,
) -> f64 {
    // Initialize the base date and leap year for the calendar
    let (base_date, is_leap_year) = if is_calendar_windows_1900 {
        let is_leap = i32::from(!(year == 1900 && month <= 2));
        (2_415_020, is_leap)
    } else {
        (2_416_481, 0)
    };

    // Adjust month and year for Julian date calculation
    let (year_adj, month_adj) = if month > 2 { (year, month - 3) } else { (year - 1, month + 9) };

    // Calculate the Julian date components
    let century = year_adj / 100;
    let decade = year_adj % 100;

    let julian_date = ((146_097 * century) / 4)
        + ((1461 * decade) / 4)
        + ((153 * month_adj + 2) / 5)
        + day
        + 1_721_119
        - base_date
        + is_leap_year;

    // Calculate the time portion of the date
    let time_in_days = f64::from(hours * 3600 + minutes * 60 + seconds) / 86400.0;

    // Return the final Excel date and time
    f64::from(julian_date) + time_in_days
}
