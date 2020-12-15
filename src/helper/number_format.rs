//use regex::Regex;
use onig::*;
use structs::number_format::NumberFormat;
use structs::color::Color;
use helper::string_helper::*;
use thousands::{Separable, SeparatorPolicy, digits};
use helper::date::*;

const DATE_FORMAT_REPLACEMENTS: &'static [(&'static str, &'static str)] = &[
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
    // month leading zero
    ("mm", "%m"),
    // month no leading zero
    ("m", "%m"),
    // full day of week name
    ("dddd", "%A"),
    // short day of week name
    ("ddd", "%a"),
    // days leading zero
    ("dd", "%d"),
    // days no leading zero
    ("d", "%d"),
    // seconds
    ("ss", "%S"),
    // fractional seconds - no rust equivalent
    (".s", ""),
];

const DATE_FORMAT_REPLACEMENTS_24: &'static [(&'static str, &'static str)] = &[
    ("hh", "%H"),
    ("h", "%k"),
];

const DATE_FORMAT_REPLACEMENTS_12: &'static [(&'static str, &'static str)] = &[
    ("hh", "%I"),
    ("h", "%l"),
];

pub fn to_formatted_string<S: Into<String>>(value:S, format:S)-> String {
    let mut value = value.into();
    let mut format = format.into();

    // is empty
    if &value == "" {
        return value;
    }

    // is numeric
    match &value.parse::<f64>() {
        Ok(_) => {},
        Err(_) => return value
    }

    // convert value
    if &format == NumberFormat::FORMAT_GENERAL {
        return value;
    }
    if &format == NumberFormat::FORMAT_TEXT {
        return value;
    }

    // Convert any other escaped characters to quoted strings, e.g. (\T to "T")
    let re = Regex::new(r#"(\\\(((.)(?!((AM\/PM)|(A\/P)))|([^ ])))(?=(?:[^"]|"[^"]*")*$)"#).unwrap();
    format = re.replace_all(&format, r#""$0""#);
    
    // Get the sections, there can be up to four sections, separated with a semi-colon (but only if not a quoted literal)
    let re = Regex::new(r#"(;)(?=(?:[^"]|"[^"]*")*$)"#).unwrap();
    let sections:Vec<&str> = re.split(&format).collect();

    let (color, split_format, split_value) = split_format(sections, &value.parse::<f64>().unwrap());
    format = split_format;
    value = split_value;

    // In Excel formats, "_" is used to add spacing,
    //    The following character indicates the size of the spacing, which we can't do in HTML, so we just use a standard space
    let re = Regex::new("/_./").unwrap();
    format = re.replace_all(&format, " ");

    // Let's begin inspecting the format and converting the value to a formatted string

    //  Check for date/time characters (not inside quotes)
    let re = Regex::new(r#"(\[\$[A-Z]*-[0-9A-F]*\])*[hmsdy](?=(?:[^"]|"[^"]*")*$)"#).unwrap();
    let re2 = Regex::new("%$").unwrap();
    if re.find(&format).is_some() {
        // datetime format
        value = format_as_date(&value.parse::<f64>().unwrap(), &format);
    } else {
        if &format.starts_with(r#"""#) == &true && &format.ends_with(r#"""#) == &true  {
            let conv_format = format.trim_matches('"').parse::<f64>().unwrap();
            value = conv_format.to_string();
        } else if re2.find(&format).is_some() {
            // % number format
            //formatAsPercentage(&value, &format);
        } else {
            value = format_as_number(&value.parse::<f64>().unwrap(), &format);
        }
    }
    value.trim().to_string()
}

fn split_format(sections:Vec<&str>, value:&f64)-> (String, String, String) {
    let mut converted_sections:Vec<String> = Vec::new();

    // Extract the relevant section depending on whether number is positive, negative, or zero?
    // Text not supported yet.
    // Here is how the sections apply to various values in Excel:
    //   1 section:   [POSITIVE/NEGATIVE/ZERO/TEXT]
    //   2 sections:  [POSITIVE/ZERO/TEXT] [NEGATIVE]
    //   3 sections:  [POSITIVE/TEXT] [NEGATIVE] [ZERO]
    //   4 sections:  [POSITIVE] [NEGATIVE] [ZERO] [TEXT]
    let cnt:usize = sections.len();
    let color_regex:String = format!("{}{}{}", "\\[(", Color::NAMED_COLORS.join("|"), ")\\]");
    let cond_regex = "\\[(>|>=|<|<=|=|<>)([+-]?\\d+([.]\\d+)?)\\]";
    let color_re = Regex::new(&color_regex).unwrap();
    let cond_re = Regex::new(&cond_regex).unwrap();
    
    let mut colors = [String::from(""), String::from(""), String::from(""), String::from(""), String::from("")];
    let mut condops = [String::from(""), String::from(""), String::from(""), String::from(""), String::from("")];
    let mut condvals = [String::from("0"), String::from("0"), String::from("0"), String::from("0"), String::from("0")];
    let mut idx = 0;
    for section in sections {
        let mut converted_section = section.to_string();
        if color_re.find(section).is_some() {
            let item:Vec<&str> = color_re.split(section).collect();
            std::mem::replace(&mut colors[idx], item.get(0).unwrap().to_string());
            converted_section = color_re.replace_all(section, "").to_string();
        }
        if cond_regex.find(section).is_some() {
            let item:Vec<&str> = cond_re.split(section).collect();
            std::mem::replace(&mut condops[idx], item.get(0).unwrap().to_string());
            std::mem::replace(&mut condvals[idx], item.get(1).unwrap().to_string());
            converted_section = cond_re.replace_all(section, "").to_string();
        }
        converted_sections.insert(idx, converted_section);
        idx+=1;
    }

    let mut color = colors[0].clone();
    let mut format:&str = &converted_sections[0];
    let mut absval = value.clone();
    match cnt {
        2 => {
            absval = absval.abs();
            let condval_one = &condvals[0].parse::<f64>().unwrap();
            if !split_format_compare(&value, &condops[0], condval_one, ">=", &0f64) {
                color = colors[1].clone();
                format = &converted_sections[1];
            }
        },
        3 | 4 => {
            absval = absval.abs();
            let condval_one = &condvals[0].parse::<f64>().unwrap();
            let condval_two = &condvals[1].parse::<f64>().unwrap();
            if !split_format_compare(&value, &condops[0], condval_one, ">", &0f64) {
                if split_format_compare(&value, &condops[1], condval_two, "<", &0f64) {
                    color = colors[1].clone();
                    format = &converted_sections[1];
                } else {
                    color = colors[2].clone();
                    format = &converted_sections[2];
                }
            }

        },
        _ => {},
    }
    (color.into(), format.into(), absval.to_string())
}

fn split_format_compare(value:&f64, cond:&str, val:&f64, dfcond:&str, dfval:&f64)->bool {
    let mut check_cond = cond;
    let mut check_val = val;
    if cond == "" {
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

fn format_as_date(value:&f64, format:&str)-> String {
    let mut value = value;
    let mut format = format.to_string();

    // strip off first part containing e.g. [$-F800] or [$USD-409]
    // general syntax: [$<Currency string>-<language info>]
    // language info is in hexadecimal
    // strip off chinese part like [DBNum1][$-804]
    let re = Regex::new(r#"^(\[[0-9A-Za-z]*\])*(\[\$[A-Z]*-[0-9A-F]*\])"#).unwrap();
    format = re.replace_all(&format, r#""#);

    // OpenOffice.org uses upper-case number formats, e.g. 'YYYY', convert to lower-case;
    //    but we don't want to change any quoted strings
    let re = Regex::new(r#"(?:^|")([^"]*)(?:$|")"#).unwrap();
    format = re.replace_all(&format,
        |caps: &Captures| {
            let caps_string: String = (&caps.at(0).unwrap()).parse().unwrap();
            caps_string.to_lowercase()
        }
    );

    // Only process the non-quoted blocks for date format characters
    let blocks:Vec<&str> = format.split('"').collect();
    let mut converted_blocks:Vec<String> = Vec::new();
    let mut i = 0;
    for block in blocks {
        let mut block = block.to_string();
        if &i % &2 == 0 {
            for (before, after) in DATE_FORMAT_REPLACEMENTS {
                block = block.replace(before, after);
            }
            if block.find("%P") == None {
                // 24-hour time format
                // when [h]:mm format, the [h] should replace to the hours of the value * 24
                if block.find("[h]") != None {
                    let hours = value * &24f64;
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
    format = converted_blocks.join(r#""#);

    // escape any quoted characters so that DateTime format() will render them correctly
    let re = Regex::new(r#""(.*)""#).unwrap();
    format = re.replace_all(&format,
        |caps: &Captures| {
            let caps_string: String = (&caps.at(0).unwrap()).parse().unwrap();
            caps_string.to_lowercase()
        }
    );

    let date_obj = excel_to_date_time_object(value, None);
    date_obj.format(&format).to_string()
}

fn format_as_number(value:&f64, format:&str)-> String {
    let mut converted_value = value.to_string();

    // The "_" in this string has already been stripped out,
    // so this test is never true. Furthermore, testing
    // on Excel shows this format uses Euro symbol, not "EUR".
    //if ($format === self::FORMAT_CURRENCY_EUR_SIMPLE) {
    //    return 'EUR ' . sprintf('%1.2f', $value);
    //}

    // Some non-number strings are quoted, so we'll get rid of the quotes, likewise any positional * symbols
    let mut converted_format = format.replace('"', "").replace("*", "");

    // Find out if we need thousands separator
    // This is indicated by a comma enclosed by a digit placeholder:
    //        #,#   or   0,0
    let re = Regex::new(r#"(#,#|0,0)"#).unwrap();
    let converted_format_clone = converted_format.clone();
    let use_thousands:Vec<&str> = re.split(&converted_format_clone).collect();
    if re.find(&converted_format_clone).is_some() {
        converted_format = Regex::new("0,0").unwrap().replace_all(&converted_format, "00");
        converted_format = Regex::new("#,#").unwrap().replace_all(&converted_format, "##");
    }

    // Scale thousands, millions,...
    // This is indicated by a number of commas after a digit placeholder:
    //        #,   or    0.0,,
    let mut scale:f64 = 1f64; // same as no scale
    let re = Regex::new(r#"(#|0)(,+)"#).unwrap();
    if re.find(&converted_format).is_some() {
        let matches:Vec<&str> = re.split(&converted_format).collect();
        scale = 1000f64 ** &matches[1].len() as f64;

        // strip the commas
        converted_format = Regex::new("0,+").unwrap().replace_all(&converted_format, "0");
        converted_format = Regex::new("#,+").unwrap().replace_all(&converted_format, "#");
    }

    if Regex::new(r#"#?.*\?\/\?"#).unwrap().is_match(&converted_format) {
        match &converted_value.parse::<usize>() {
            Ok(_) => {},
            Err(_) => {
                converted_value = format_as_fraction(&converted_value.parse::<f64>().unwrap(), &converted_format);
            },
        }
    } else {
        // Handle the number itself

        // scale number
        converted_value = (converted_value.parse::<f64>().unwrap() / scale).to_string();
        // Strip #
        converted_format = Regex::new(r#"\\#"#).unwrap().replace_all(&converted_format, "0");
        // Remove locale code [$-###]
        converted_format = Regex::new(r#"\[\$\-.*\]"#).unwrap().replace_all(&converted_format, "");

        let m = Regex::new(r#"\\[[^\\]]+\\]"#).unwrap().replace_all(&converted_format, "");
        let number_regex = r#"(0+)(\\.?)(0*)"#;
        let re = Regex::new(number_regex).unwrap();
        if re.find(&m).is_some() {
            let item:Vec<&str> = re.split(&m).collect();
            converted_value = format_straight_numeric_value(&converted_value, &converted_format, &item, &use_thousands, number_regex);
        }
    }

    let re = Regex::new(r#"\[\$(.*)\]"#).unwrap();
    if re.find(&converted_format).is_some() {
        let item:Vec<&str> = re.split(&converted_format).collect();
        //  Currency or Accounting
        let mut currency_code = item.get(1).unwrap().to_string();
        //[$currencyCode] = explode('-', $currencyCode);
        if currency_code == "" {
            currency_code = get_currency_code();
        }
        converted_value = Regex::new(r#"\[\$([^\]]*)\]"#).unwrap().replace_all(&converted_format, currency_code.as_str()).to_string();
    }

    converted_value
}

fn format_as_fraction(value:&f64, format:&str)-> String {
    let sign = if value < &0f64 { "-" } else { "" };

    let integer_part = value.abs().floor();
    let decimal_part = value.abs() % 1f64;
    let decimal_length = decimal_part.to_string().len();
    let decimal_divisor = (10 ** &decimal_length).to_string().parse::<f64>().unwrap();

    let gcd = gcd(&decimal_part, &decimal_divisor);

    let adjusted_decimal_part = &decimal_part / &gcd;
    let adjusted_decimal_divisor = &decimal_divisor / &gcd;

    let mut result = String::from("");
    match format.find("0") {
        Some(_) => {
            result = format!("{}{} {}/{}", &sign, &integer_part, &adjusted_decimal_part, &adjusted_decimal_divisor);
        },
        None => {
            match format.find("#") {
                Some(_) => {
                    if integer_part == 0f64 {
                        result = format!("{}{}/{}", &sign, &adjusted_decimal_part, &adjusted_decimal_divisor);
                    } else {
                        result = format!("{}{} {}/{}", &sign, &integer_part, &adjusted_decimal_part, &adjusted_decimal_divisor);
                    }
                },
                None => {
                    let check_format: String = format.chars().take(3).collect();
                    if check_format == "? ?" {
                        let mut integer_part_str = integer_part.to_string();
                        if integer_part == 0f64 {
                            integer_part_str = String::from("");
                        }
                        result = format!("{}{} {}/{}", &sign, &integer_part_str, &adjusted_decimal_part, &adjusted_decimal_divisor);
                    } else {
                        //adjusted_decimal_part += &integer_part * &adjusted_decimal_divisor.to_string().parse::<f64>().unwrap();
                        result = format!("{}{}/{}", &sign, &adjusted_decimal_part, &adjusted_decimal_divisor);
                    }
                }
            }
        
        }
    }
    result
}

fn format_straight_numeric_value(value:&str, format:&str, matches: &Vec<&str>, use_thousands:&Vec<&str>, number_regex:&str)-> String {
    let mut converted_value = value.to_string();

    let left = matches.get(1).unwrap();
    let dec = matches.get(2).unwrap();
    let right = matches.get(3).unwrap();

    // minimun width of formatted number (including dot)
    let minWidth = left.len() + dec.len() + right.len();
    if use_thousands.len() > 0 {
        converted_value = converted_value.parse::<usize>().unwrap().separate_with_commas();
        converted_value = Regex::new(&number_regex).unwrap().replace_all(&format, converted_value.as_str()).to_string();
    } else {
        if Regex::new(r#"[0#]E[+-]0"#).unwrap().is_match(&format) {
            // Scientific format
            // $value = sprintf('%5.2E', $value); TODO
        } else if Regex::new(r#"0([^\d\.]+)0"#).unwrap().is_match(&format) || format.find(".") != None {
            let mut char_cnt = 0;
            for i in format.chars() {
                if i == '.' {
                    char_cnt += 1;
                }
            }
            if char_cnt == 1 {
                //converted_value = (10 ** format.split('.').collect().get(1).len()).to_string();
            }
            converted_value = complex_number_format_mask(&converted_value.parse::<f64>().unwrap(), &format, &true);
        } else {
            let sprintf_pattern = format!("$minWidth.{}{}", right.len(), 'f');
            converted_value = format!("{}{}", &converted_value, &sprintf_pattern);
            converted_value = Regex::new(number_regex).unwrap().replace_all(&converted_value, format).to_string();
        }
    }
    converted_value
}

fn merge_complex_number_format_masks(numbers:&Vec<String>, masks:&Vec<String>)-> Vec<String> {
    let decimal_count = numbers[1].len();
    let mut post_decimal_masks:Vec<String> = Vec::new();

    for mask in masks.iter().rev() {
        post_decimal_masks.push(mask.to_string());
    }

    let mut result:Vec<String> = Vec::new();
    result.push(masks.join("."));
    result.push(post_decimal_masks.join("."));
    result
}

fn process_complex_number_formatMask(number:&f64, mask:&str)-> String {
    let mut result = number.to_string();
    let mut converted_mask = mask.to_string();
    let re = Regex::new(r#"0+"#).unwrap();
    let mut masking_blocks:Vec<String> = Vec::new();
    let re = Regex::new(r#"\[\$(.*)\]"#).unwrap();
    let masking_blocks:Vec<&str> = re.split(&mask).collect();

    if masking_blocks.len() > 1 {
        let mut number:f64 = 0f64;
        let mut offset:usize = 0;
        for block in masking_blocks.iter().rev() {
            let divisor = format!("{}{}", 1, block);
            let size = block.len();
            offset = block.parse::<usize>().unwrap();

            let blockValue = format!("{}", (number / divisor.parse::<f64>().unwrap()));
            number = (number / divisor.parse::<f64>().unwrap()).abs().floor();
            let from: String = mask.chars().skip(offset).take(size).collect();
            converted_mask = mask.replace(&from, &blockValue);
        }
        if number > 0f64 {
            let from: String = mask.chars().skip(offset).collect();
            converted_mask = mask.replace(&from, &number.to_string());
        }
        result = converted_mask;
    }
    result
}

fn complex_number_format_mask(number:&f64, mask:&str, splitOnPoint:&bool)->String {
    let sign = number < &0.0;
    let converted_number = number.abs();

    if splitOnPoint == &true && mask.find(".") != None && &converted_number.to_string().find(".") != &None {
        let converted_number_str = converted_number.to_string();
        let numbers_as:Vec<&str> = converted_number_str.split('.').collect();
        let mut numbers:Vec<String> = Vec::new();
        for number in numbers_as {
            numbers.push(number.to_string());
        }
        let masks_as:Vec<&str> = mask.split('.').collect();
        let mut masks:Vec<String> = Vec::new();
        for mask in masks_as {
            masks.push(mask.to_string());
        }
        if masks.len() > 2 {
            masks = merge_complex_number_format_masks(&numbers, &masks);
        }
        let result1 = complex_number_format_mask(&numbers[0].parse::<f64>().unwrap(), &masks[0], &false);
        let result2 = complex_number_format_mask(
            &numbers[1].chars().rev().collect::<String>().parse::<f64>().unwrap(),
            &masks[1].chars().rev().collect::<String>(),
            &false
        ).chars().rev().collect::<String>();
        
        return format!("{}{}.{}", if sign { "-" } else { "" } , result1, result2);
    }

    let result = process_complex_number_formatMask(&converted_number, mask);
    format!("{}{}", if sign { "-" } else { "" } , result)
}

fn gcd(a: &f64, b: &f64) -> f64 {
    if b == &0f64 {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}
