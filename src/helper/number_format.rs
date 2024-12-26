mod date_formater;
mod fraction_formater;
mod number_formater;
mod percentage_formater;

use std::borrow::Cow;

use fancy_regex::{
    Matches,
    Regex,
};

use crate::{
    helper::utils::compile_regex,
    structs::NumberingFormat,
};

pub struct Split<'r, 't> {
    finder: Matches<'r, 't>,
    last:   usize,
}

#[inline]
#[must_use]
pub fn split<'r, 't>(regex: &'r Regex, text: &'t str) -> Split<'r, 't> {
    Split {
        finder: regex.find_iter(text),
        last:   0,
    }
}

impl<'t> Iterator for Split<'_, 't> {
    type Item = &'t str;

    fn next(&mut self) -> Option<Self::Item> {
        let text = self.finder.text();
        match self.finder.next() {
            Some(Ok(m)) => {
                let matched = &text[self.last..m.start()];
                self.last = m.end();
                Some(matched)
            }
            _ => {
                if self.last > text.len() {
                    None
                } else {
                    let s = &text[self.last..];
                    self.last = text.len() + 1;
                    Some(s)
                }
            }
        }
    }
}

pub fn get_escape_regex() -> &'static Regex {
    compile_regex!(r#"(\\\(((.)(?!((AM\/PM)|(A\/P)))|([^ ])))(?=(?:[^"]|"[^"]*")*$)"#)
}

pub fn get_section_regex() -> &'static Regex {
    compile_regex!(r#"(;)(?=(?:[^"]|"[^"]*")*$)"#)
}

pub fn get_date_time_regex() -> &'static Regex {
    compile_regex!(r#"(\[\$[A-Z]*-[0-9A-F]*\])*[hmsdy](?=(?:[^"]|"[^"]*")*$)"#)
}

pub fn get_percent_dollar_regex() -> &'static Regex {
    compile_regex!("%$")
}

pub fn to_formatted_string<S: AsRef<str>, P: AsRef<str>>(value: S, format: P) -> String {
    let mut value: Cow<str> = Cow::Borrowed(value.as_ref());
    let format = Cow::Borrowed(format.as_ref());

    // is empty
    if value.is_empty() {
        return value.to_string();
    }
    // is numeric
    match &value.parse::<f64>() {
        // convert value
        Ok(val) if format == NumberingFormat::FORMAT_GENERAL => {
            return val.to_string();
        }
        Ok(_) if format == NumberingFormat::FORMAT_TEXT => return value.to_string(),
        Err(_) => return value.to_string(),
        _ => {}
    }

    // Convert any other escaped characters to quoted strings, e.g. (\T to "T")

    let mut format = get_escape_regex().replace_all(&format, r#""$0""#);

    // Get the sections, there can be up to four sections, separated with a
    // semi-colon (but only if not a quoted literal)

    let sections: Vec<&str> = split(get_section_regex(), &format).collect();

    let (_, split_format, split_value) = split_format(sections, value.parse::<f64>().unwrap());
    format = Cow::Owned(split_format);
    value = Cow::Owned(split_value);

    // In Excel formats, "_" is used to add spacing,
    //    The following character indicates the size of the spacing, which we can't
    // do in HTML, so we just use a standard space
    let re = compile_regex!("_.");
    let format = re.replace_all(&format, " ");

    // Let's begin inspecting the format and converting the value to a formatted
    // string

    //  Check for date/time characters (not inside quotes)

    if get_date_time_regex().is_match(&format).unwrap_or(false) {
        // datetime format
        value = date_formater::format_as_date(value.parse::<f64>().unwrap(), &format);
    } else if format.starts_with('"') && format.ends_with('"') {
        let conv_format = format.trim_matches('"').parse::<f64>().unwrap();
        value = Cow::Owned(conv_format.to_string());
    } else if get_percent_dollar_regex()
        .is_match(&format)
        .unwrap_or(false)
    {
        // % number format
        value = percentage_formater::format_as_percentage(value.parse::<f64>().unwrap(), &format);
    } else {
        value = number_formater::format_as_number(value.parse::<f64>().unwrap(), &format);
    }
    value.trim().to_string()
}

fn split_format(sections: Vec<&str>, value: f64) -> (String, String, String) {
    let mut converted_sections: Vec<String> = Vec::new();

    // Extract the relevant section depending on whether number is positive,
    // negative, or zero? Text not supported yet.
    // Here is how the sections apply to various values in Excel:
    //   1 section:   [POSITIVE/NEGATIVE/ZERO/TEXT]
    //   2 sections:  [POSITIVE/ZERO/TEXT] [NEGATIVE]
    //   3 sections:  [POSITIVE/TEXT] [NEGATIVE] [ZERO]
    //   4 sections:  [POSITIVE] [NEGATIVE] [ZERO] [TEXT]
    let cnt: usize = sections.len();

    // let color_re: String = format!("{}{}{}", "\\[(",
    // Color::NAMED_COLORS.join("|"), ")\\]");
    let color_re = compile_regex!(r"\[(Black|White|Red|Green|Blue|Yellow|Magenta|Cyan)\]");
    let cond_re = compile_regex!(r"\[(>|>=|<|<=|=|<>)([+-]?\d+([.]\d+)?)\]");

    let mut colors = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    let mut condops = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    let mut condvals = [
        String::from("0"),
        String::from("0"),
        String::from("0"),
        String::from("0"),
        String::from("0"),
    ];
    sections.into_iter().enumerate().for_each(|(idx, section)| {
        let mut converted_section = section.to_string();

        // Process color matching
        if let Some(captures) = color_re.captures(section).ok().flatten() {
            let items: Vec<String> = captures
                .iter()
                .filter_map(|cap| cap.map(|c| c.as_str().to_string()))
                .collect();

            if let Some(first_item) = items.first() {
                colors[idx].clone_from(first_item);
            }

            converted_section = color_re.replace_all(section, "").to_string();
        }

        // Process conditional matching
        if let Some(captures) = cond_re.captures(section).ok().flatten() {
            let items: Vec<String> = captures
                .iter()
                .filter_map(|cap| cap.map(|c| c.as_str().to_string()))
                .collect();

            if let Some(v) = items.get(1) {
                condops[idx].clone_from(v);
            }
            if let Some(v) = items.get(2) {
                condvals[idx].clone_from(v);
            }

            converted_section = cond_re.replace_all(section, "").to_string();
        }

        converted_sections.insert(idx, converted_section);
    });

    let mut color = &colors[0];
    let mut format: &str = &converted_sections[0];
    let mut absval = value;
    match cnt {
        2 => {
            absval = absval.abs();
            let condval_one = condvals[0].parse::<f64>().unwrap();
            if !split_format_compare(value, &condops[0], condval_one, ">=", 0f64) {
                color = &colors[1];
                format = &converted_sections[1];
            }
        }
        3 | 4 => {
            absval = absval.abs();
            let condval_one = condvals[0].parse::<f64>().unwrap();
            let condval_two = condvals[1].parse::<f64>().unwrap();
            if !split_format_compare(value, &condops[0], condval_one, ">", 0f64) {
                if split_format_compare(value, &condops[1], condval_two, "<", 0f64) {
                    color = &colors[1];
                    format = &converted_sections[1];
                } else {
                    color = &colors[2];
                    format = &converted_sections[2];
                }
            }
        }
        _ => {}
    }
    (color.to_string(), format.into(), absval.to_string())
}

#[allow(clippy::float_cmp)]
fn split_format_compare(value: f64, cond: &str, val: f64, dfcond: &str, dfval: f64) -> bool {
    let mut check_cond = cond;
    let mut check_val = val;
    if cond.is_empty() {
        check_cond = dfcond;
        check_val = dfval;
    }
    match check_cond {
        ">" => return value > check_val,
        "<" => return value < check_val,
        "<=" => return value <= check_val,
        "<>" => return value != check_val,
        "=" => return value == check_val,
        _ => {}
    }
    value >= check_val
}

#[test]
fn test_to_formatted_string_date() {
    let value = String::from("45435"); // 2024/5/23
    assert_eq!(
        r"2024-05-23",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_YYYYMMDD2)
    );
    assert_eq!(
        r"2024-05-23",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_YYYYMMDD)
    );
    assert_eq!(
        r"23-05-2024",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_DDMMYYYY)
    );
    assert_eq!(
        r"23/05/2024",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_DDMMYYYYSLASH)
    );
    assert_eq!(
        r"23/5/24",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_DMYSLASH)
    );
    assert_eq!(
        r"23-5-24",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_DMYMINUS)
    );
    assert_eq!(
        r"23-5",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_DMMINUS)
    );
    assert_eq!(
        r"5-24",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_MYMINUS)
    );
    assert_eq!(
        r"05-23-24",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_XLSX14)
    );
    assert_eq!(
        r"23-May-24",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_XLSX15)
    );
    assert_eq!(
        r"23-May",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_XLSX16)
    );
    assert_eq!(
        r"May-24",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_XLSX17)
    );
    assert_eq!(
        r"5/23/24 0:00",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_XLSX22)
    );
    assert_eq!(
        r"23/5/24 0:00",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_DATETIME)
    );
    assert_eq!(
        r"12:00 am",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME1)
    );
    assert_eq!(
        r"12:00:00 am",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME2)
    );
    assert_eq!(
        r"0:00",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME3)
    );
    assert_eq!(
        r"0:00:00",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME4)
    );
    assert_eq!(
        r"00:00",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME5)
    );
    assert_eq!(
        r"0:00:00",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME6)
    );
    assert_eq!(
        r"0:00:00",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME8)
    );
    assert_eq!(
        r"2024/05/23",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_YYYYMMDDSLASH)
    );

    let value = String::from("44349.211134259262"); // 2021/06/02 05:04:02
    assert_eq!(
        r"2021-06-02",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_YYYYMMDD2)
    );
    assert_eq!(
        r"2021-06-02",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_YYYYMMDD)
    );
    assert_eq!(
        r"02-06-2021",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_DDMMYYYY)
    );
    assert_eq!(
        r"02/06/2021",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_DDMMYYYYSLASH)
    );
    assert_eq!(
        r"2/6/21",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_DMYSLASH)
    );
    assert_eq!(
        r"2-6-21",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_DMYMINUS)
    );
    assert_eq!(
        r"2-6",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_DMMINUS)
    );
    assert_eq!(
        r"6-21",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_MYMINUS)
    );
    assert_eq!(
        r"06-02-21",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_XLSX14)
    );
    assert_eq!(
        r"2-Jun-21",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_XLSX15)
    );
    assert_eq!(
        r"2-Jun",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_XLSX16)
    );
    assert_eq!(
        r"Jun-21",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_XLSX17)
    );
    assert_eq!(
        r"6/2/21 5:04",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_XLSX22)
    );
    assert_eq!(
        r"2/6/21 5:04",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_DATETIME)
    );
    assert_eq!(
        r"5:04 am",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME1)
    );
    assert_eq!(
        r"5:04:02 am",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME2)
    );
    assert_eq!(
        r"5:04",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME3)
    );
    assert_eq!(
        r"5:04:02",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME4)
    );
    assert_eq!(
        r"04:02",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME5)
    );
    assert_eq!(
        r"5:04:02",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME6)
    );
    assert_eq!(
        r"5:04:02",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_TIME8)
    );
    assert_eq!(
        r"2021/06/02",
        to_formatted_string(&value, NumberingFormat::FORMAT_DATE_YYYYMMDDSLASH)
    );
    assert_eq!(r"2", to_formatted_string(&value, "d"));
}
