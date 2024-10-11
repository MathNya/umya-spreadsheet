use std::borrow::Cow;

use fancy_regex::Captures;
use fancy_regex::Matches;
use fancy_regex::Regex;
use helper::date::*;
use structs::Color;
use structs::NumberingFormat;
use thousands::Separable;

const DATE_FORMAT_REPLACEMENTS: &[(&str, &str)] = &[
    // first remove escapes related to non-format characters
    ("\\", ""),
    // 12-hour suffix
    ("am/pm", "%P"),
    // Era - no rust equivalent
    ("ggge", "%Y"),
    // 4-digit year
    ("e", "%Y"),
    ("yyyy", "%Y"),
    // 2-digit year
    ("yy", "%y"),
    // first letter of month - no rust equivalent
    ("mmmmm", "%b"),
    // full month name
    ("mmmm", "%B"),
    // short month name
    ("mmm", "%b"),
    // mm is minutes if time, but can also be month w/leading zero
    // so we try to identify times be the inclusion of a : separator in the mask
    // It isn't perfect, but the best way I know how
    (":mm", ":%M"),
    ("mm:", "%M:"),
    // month leading zero (first)
    ("mm", "MM"),
    // month no leading zero
    ("m", "%-m"),
    // month leading zero (second)
    ("MM", "%m"),
    // full day of week name
    ("dddd", "%A"),
    // short day of week name
    ("ddd", "%a"),
    // days leading zero (first)
    ("dd", "D"),
    // days no leading zero
    ("d", "%-d"),
    // days leading zero (second)
    ("D", "%d"),
    // seconds
    ("ss", "%S"),
    // fractional seconds - no rust equivalent
    (".s", ""),
];

const DATE_FORMAT_REPLACEMENTS_24: &[(&str, &str)] = &[("hh", "%H"), ("h", "%-H")];

const DATE_FORMAT_REPLACEMENTS_12: &[(&str, &str)] = &[("hh", "%I"), ("h", "%-I")];

pub(crate) fn format_as_date<'input>(value: &f64, format: &'input str) -> Cow<'input, str> {
    let format = Cow::Borrowed(format);

    // strip off first part containing e.g. [$-F800] or [$USD-409]
    // general syntax: [$<Currency string>-<language info>]
    // language info is in hexadecimal
    // strip off chinese part like [DBNum1][$-804]
    let re = Regex::new(r"^(\[[0-9A-Za-z]*\])*(\[\$[A-Z]*-[0-9A-F]*\])").unwrap();
    let format = re.replace_all(&format, r#""#);

    // OpenOffice.org uses upper-case number formats, e.g. 'YYYY', convert to lower-case;
    //    but we don't want to change any quoted strings
    let re = Regex::new(r#"(?:^|")([^"]*)(?:$|")"#).unwrap();
    let mut format = re.replace_all(&format, |caps: &Captures| {
        let caps_string = caps.get(0).unwrap().as_str();
        caps_string.to_lowercase()
    });

    // Only process the non-quoted blocks for date format characters
    let blocks: Vec<&str> = format.split('"').collect();
    let mut converted_blocks: Vec<String> = Vec::new();
    let mut i = 0;
    for block in blocks {
        let mut block = block.to_string();
        if i % 2 == 0 {
            for (before, after) in DATE_FORMAT_REPLACEMENTS {
                block = block.replace(before, after);
            }
            if !block.contains("%P") {
                // 24-hour time format
                // when [h]:mm format, the [h] should replace to the hours of the value * 24
                if block.contains("[h]") {
                    let hours = value * 24f64;
                    block = block.replace("[h]", hours.to_string().as_str());
                    converted_blocks.push(block);
                    continue;
                }
                for (before, after) in DATE_FORMAT_REPLACEMENTS_24 {
                    block = block.replace(before, after);
                }
            } else {
                for (before, after) in DATE_FORMAT_REPLACEMENTS_12 {
                    block = block.replace(before, after);
                }
            }
        }
        converted_blocks.push(block);
        i += 1;
    }
    format = Cow::Owned(converted_blocks.join(r#""#));

    // escape any quoted characters so that DateTime format() will render them correctly
    let re = Regex::new(r#""(.*)""#).unwrap();
    let format = re.replace_all(&format, |caps: &Captures| {
        let caps_string = caps.get(0).unwrap().as_str();
        caps_string.to_lowercase()
    });

    let date_obj = excel_to_date_time_object(value, None);
    Cow::Owned(date_obj.format(&format).to_string())
}
