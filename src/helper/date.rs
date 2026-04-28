use chrono::{
    Duration,
    NaiveDateTime,
};
use jiff::ToSpan as _;
use num_traits::cast;

pub const CALENDAR_WINDOWS_1900: &str = "1900";
pub const CALENDAR_MAC_1904: &str = "1904";
pub const DEFAULT_TIMEZONE: &str = "UTC";

/// Converts an Excel timestamp to a [`NaiveDateTime`] object.
///
/// This function takes an Excel timestamp, which is a numeric representation of
/// a date and time, and converts it into a [`NaiveDateTime`] object. The
/// integer part of the timestamp represents the number of days since a base
/// date, while the fractional part represents the time of day.
///
/// # Parameters
///
/// - `excel_timestamp: f64` The Excel timestamp to be converted and it is
///   treated as an Excel date.
///
/// # Returns
///
/// Returns a [`NaiveDateTime`] object representing the converted date and time.
/// This object does not contain any timezone information.
///
/// # Behavior
///
/// - If `excel_timestamp` is between 1 and 60, the function accounts for the
///   1900 leap year bug in Excel by using December 31, 1899, as the base date.
/// - For `excel_timestamp` values of 60 or greater, it uses December 30, 1899,
///   as the base date.
/// - This uses the Windows 1900 scheme which is more common.
/// - Since 1900-02-29 is not a valid date this function returns 1900-03-01 in
///   that case.
///
/// # Example
///
/// ```rust
/// # use umya_spreadsheet::helper::date::excel_to_date_time_chrono;
/// # use chrono::{Datelike, Timelike};
/// let timestamp = 44197.5; // Represents 2021-01-01 12:00:00
/// let date_time = excel_to_date_time_chrono(timestamp);
/// assert_eq!(date_time.year(), 2021);
/// assert_eq!(date_time.month(), 1);
/// assert_eq!(date_time.day(), 1);
/// assert_eq!(date_time.hour(), 12);
/// assert_eq!(date_time.minute(), 0);
/// ```
#[must_use]
pub fn excel_to_date_time_chrono(excel_timestamp: f64) -> NaiveDateTime {
    let base_date = if excel_timestamp < 61f64 {
        // Allow adjustment for 1900 Leap Year in MS Excel
        NaiveDateTime::parse_from_str("1899-12-31 00:00:00", "%Y-%m-%d %T").unwrap()
    } else {
        NaiveDateTime::parse_from_str("1899-12-30 00:00:00", "%Y-%m-%d %T").unwrap()
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

/// Converts an Excel timestamp to a [`jiff::civil::DateTime`] object.
///
/// This function takes an Excel timestamp, which is a numeric representation of
/// a date and time, and converts it into a [`jiff::civil::DateTime`] object.
/// The integer part of the timestamp represents the number of days since a base
/// date, while the fractional part represents the time of day.
///
/// # Parameters
///
/// - `excel_timestamp: f64` The Excel timestamp to be converted and it is
///   treated as an Excel date.
///
/// # Returns
///
/// Returns a [`jiff::civil::DateTime`] object representing the converted date
/// and time. This object does not contain any timezone information.
///
/// # Behavior
///
/// - If `excel_timestamp` is between 1 and 60, the function accounts for the
///   1900 leap year bug in Excel by using December 31, 1899, as the base date.
/// - For `excel_timestamp` values of 60 or greater, it uses December 30, 1899,
///   as the base date.
/// - This uses the Windows 1900 scheme which is more common.
/// - Since 1900-02-29 is not a valid date this function returns 1900-03-01 in
///   that case.
///
/// # Example
///
/// ```rust
/// # use umya_spreadsheet::helper::date::excel_to_date_time_jiff;
/// let timestamp = 44197.5; // Represents 2021-01-01 12:00:00
/// let date_time = excel_to_date_time_jiff(timestamp);
/// assert_eq!(date_time.year(), 2021);
/// assert_eq!(date_time.month(), 1);
/// assert_eq!(date_time.day(), 1);
/// assert_eq!(date_time.hour(), 12);
/// assert_eq!(date_time.minute(), 0);
/// ```
#[must_use]
pub fn excel_to_date_time_jiff(excel_timestamp: f64) -> jiff::civil::DateTime {
    let base_date = if excel_timestamp < 61f64 {
        // Allow adjustment for 1900 Leap Year in MS Excel
        jiff::civil::datetime(1899, 12, 31, 0, 0, 0, 0)
    } else {
        jiff::civil::datetime(1899, 12, 30, 0, 0, 0, 0)
    };

    let days = excel_timestamp.floor();
    let seconds: i64 = cast(((excel_timestamp - days) * (24.0 * 60.0 * 60.0)).round()).unwrap();
    let days: i64 = cast(days).unwrap();

    base_date + days.day().seconds(seconds)
}

/// See docs for `excel_to_date_time_chrono` for details on how this function
/// works. Note that the `time_zone` is not used and is ignored. Excel doesn't
/// store associated timezone info with the dates.
#[must_use]
#[deprecated(
    since = "3.0.0",
    note = "Please use `excel_to_date_time_jiff` or `excel_to_date_time_chrono` instead"
)]
pub fn excel_to_date_time_object(
    excel_timestamp: f64,
    _time_zone: Option<String>,
) -> NaiveDateTime {
    excel_to_date_time_chrono(excel_timestamp)
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
/// # use umya_spreadsheet::helper::date::convert_date;
/// let timestamp = convert_date(2021, 1, 1, 12, 0, 0);
/// assert_eq!(timestamp, 44197.5); // Represents 2021-01-01 12:00:00
/// ```
///
/// # Note
///
/// - Earliest valid date in excel under the Windows 1900 system is 1899-12-31
///   at 00:00. This function will return a negative number for earlier dates
///   which is not valid in excel.
/// - Invalid components are not check and will just return invalid values
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
/// # use umya_spreadsheet::helper::date::convert_date_windows_1900;
/// let timestamp = convert_date_windows_1900(2021, 1, 1, 12, 0, 0);
/// assert_eq!(timestamp, 44197.5); // Represents 2021-01-01 12:00:00
/// ```
///
/// # Note
///
/// - Earliest valid date in excel under the Windows 1900 system is 1899-12-31
///   at 00:00. This function will return a negative number for earlier dates
///   which is not valid in excel.
/// - Invalid components are not check and will just return invalid values
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
/// # use umya_spreadsheet::helper::date::convert_date_mac_1904;
/// let timestamp = convert_date_mac_1904(2021, 1, 1, 12, 0, 0);
/// assert_eq!(timestamp, 42735.5); // Represents 2021-01-01 12:00:00
/// ```
///
/// # Note
///
/// - Earliest valid date in excel under the Mac 1904 system is 1904-01-01 at
///   00:00. This function will return a negative number for earlier dates which
///   is not valid in excel.
/// - Invalid components are not check and will just return invalid values
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
/// # use umya_spreadsheet::helper::date::convert_date_crate;
/// let timestamp_windows = convert_date_crate(2021, 1, 1, 12, 0, 0, true);
/// assert_eq!(timestamp_windows, 44197.5); // Represents 2021-01-01 12:00:00 in Windows 1900 system
///
/// let timestamp_mac = convert_date_crate(2021, 1, 1, 12, 0, 0, false);
/// assert_eq!(timestamp_mac, 42735.5); // Represents 2021-01-01 12:00:00 in Mac 1904 system
/// ```
///
/// # Note
///
/// - Earliest valid date in excel under the Windows 1900 system is 1899-12-31
///   at 00:00. This function will return a negative number for earlier dates
///   which is not valid in excel.
/// - Earliest valid date in excel under the Mac 1904 system is 1904-01-01 at
///   00:00. This function will return a negative number for earlier dates which
///   is not valid in excel.
/// - Invalid components are not check and will just return invalid values
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
        let is_leap = i32::from(!((year == 1900 && month <= 2) || (year == 1899)));
        (2_415_020, is_leap)
    } else {
        (2_416_481, 0)
    };

    // Adjust month and year for Julian date calculation
    let (year_adj, month_adj) = if month > 2 {
        (year, month - 3)
    } else {
        (year - 1, month + 9)
    };

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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    const ALLOWED_ERROR_FLOAT_CMP: f64 = 0.0001;

    #[rstest]
    #[case(1899, 12, 31, 0, 0, 0, 0.0)]
    #[case(1899, 12, 31, 6, 0, 0, 0.25)]
    #[case(1899, 12, 31, 12, 0, 0, 0.5)]
    #[case(1900, 1, 1, 0, 0, 0, 1.0)]
    #[case(1900, 1, 1, 12, 0, 0, 1.5)]
    #[case(1900, 1, 30, 0, 0, 0, 30.0)]
    #[case(1900, 2, 28, 6, 0, 0, 59.25)]
    #[case(1900, 2, 28, 23, 45, 36, 59.99)]
    #[case(1900, 2, 29, 0, 0, 0, 60.0)]
    #[case(1900, 2, 29, 12, 0, 0, 60.5)]
    #[case(1900, 3, 1, 18, 0, 0, 61.75)]
    #[case(1900, 3, 2, 0, 0, 0, 62.0)]
    #[case(1900, 4, 9, 0, 0, 0, 100.0)]
    #[case(1900, 7, 18, 0, 0, 0, 200.0)]
    #[case(2021, 1, 1, 12, 0, 0, 44197.5)]
    #[case(2016, 12, 31, 12, 0, 0, 42735.5)]
    fn test_convert_date_windows_1900(
        #[case] year: i32,
        #[case] month: i32,
        #[case] day: i32,
        #[case] hours: i32,
        #[case] minutes: i32,
        #[case] seconds: i32,
        #[case] excel_timestamp: f64,
    ) {
        let expected = excel_timestamp;
        let actual = convert_date_windows_1900(year, month, day, hours, minutes, seconds);
        assert!(
            (actual - expected).abs() < ALLOWED_ERROR_FLOAT_CMP,
            "Expected: {expected}, Actual: {actual} - Tolerance: {ALLOWED_ERROR_FLOAT_CMP}"
        );
    }

    #[rstest]
    #[case(1904, 1, 1, 0, 0, 0, 0.0)]
    #[case(1904, 1, 1, 6, 0, 0, 0.25)]
    #[case(1904, 1, 1, 12, 0, 0, 0.5)]
    #[case(1904, 1, 2, 0, 0, 0, 1.0)]
    #[case(1904, 1, 2, 12, 0, 0, 1.5)]
    #[case(1904, 1, 31, 0, 0, 0, 30.0)]
    #[case(1904, 2, 29, 6, 0, 0, 59.25)]
    #[case(1904, 2, 29, 23, 45, 36, 59.99)]
    #[case(1904, 3, 1, 0, 0, 0, 60.0)]
    #[case(1904, 3, 1, 12, 0, 0, 60.5)]
    #[case(1904, 3, 2, 18, 0, 0, 61.75)]
    #[case(1904, 3, 3, 0, 0, 0, 62.0)]
    #[case(1904, 4, 10, 0, 0, 0, 100.0)]
    #[case(1904, 7, 19, 0, 0, 0, 200.0)]
    #[case(2025, 1, 2, 12, 0, 0, 44197.5)]
    #[case(2021, 1, 1, 12, 0, 0, 42735.5)]
    fn test_convert_date_mac_1904(
        #[case] year: i32,
        #[case] month: i32,
        #[case] day: i32,
        #[case] hours: i32,
        #[case] minutes: i32,
        #[case] seconds: i32,
        #[case] excel_timestamp: f64,
    ) {
        let expected = excel_timestamp;
        let actual = convert_date_mac_1904(year, month, day, hours, minutes, seconds);
        assert!(
            (actual - expected).abs() < ALLOWED_ERROR_FLOAT_CMP,
            "Expected: {expected}, Actual: {actual} - Tolerance: {ALLOWED_ERROR_FLOAT_CMP}"
        );
    }

    #[rstest]
    #[case(0.0, "1899-12-31 00:00:00")]
    #[case(0.25, "1899-12-31 06:00:00")]
    #[case(0.5, "1899-12-31 12:00:00")]
    #[case(1.0, "1900-01-01 00:00:00")]
    #[case(1.5, "1900-01-01 12:00:00")]
    #[case(30.0, "1900-01-30 00:00:00")]
    #[case(59.25, "1900-02-28 06:00:00")]
    #[case(59.99, "1900-02-28 23:45:36")]
    #[case(60.0, "1900-03-01 00:00:00")] // Shows in excel as 1900-02-29 but this date is not real and not representable
    #[case(60.5, "1900-03-01 12:00:00")] // Shows in excel as 1900-02-29 but this date is not real and not representable
    #[case(61.75, "1900-03-01 18:00:00")]
    #[case(62.0, "1900-03-02 00:00:00")]
    #[case(100.0, "1900-04-09 00:00:00")]
    #[case(200.0, "1900-07-18 00:00:00")]
    #[case(44197.5, "2021-01-01 12:00:00")]
    #[case(42735.5, "2016-12-31 12:00:00")]
    fn excel_to_date_time(#[case] excel_timestamp: f64, #[case] expected: &str) {
        let actual = excel_to_date_time_jiff(excel_timestamp);
        assert_eq!(
            expected,
            actual.strftime("%F %T").to_string(),
            "jiff conversion is incorrect"
        );

        let actual = excel_to_date_time_chrono(excel_timestamp);
        assert_eq!(
            expected,
            actual.format("%F %T").to_string(),
            "chrono conversion is incorrect"
        );
    }
}
