use std::{
    borrow::Cow,
    sync::OnceLock,
};

use fancy_regex::Regex;
use thousands::Separable;

use super::fraction_formater::format_as_fraction;

// Initialize OnceLock for each Regex
static THOUSANDS_SEP_REGEX: OnceLock<Regex> = OnceLock::new();
static SCALE_REGEX: OnceLock<Regex> = OnceLock::new();
static TRAILING_COMMA_REGEX: OnceLock<Regex> = OnceLock::new();
static FRACTION_REGEX: OnceLock<Regex> = OnceLock::new();
static SQUARE_BRACKET_REGEX: OnceLock<Regex> = OnceLock::new();
static NUMBER_REGEX: OnceLock<Regex> = OnceLock::new();

pub(crate) fn format_as_number(value: f64, format: &str) -> Cow<str> {
    // Initialize regex patterns using OnceLock
    let thousands_sep_regex = THOUSANDS_SEP_REGEX.get_or_init(|| Regex::new(r"(#,#|0,0)").unwrap());
    let scale_regex = SCALE_REGEX.get_or_init(|| Regex::new(r"(#|0)(,+)").unwrap());
    let trailing_comma_regex = TRAILING_COMMA_REGEX.get_or_init(|| Regex::new("(#|0),+").unwrap());
    let fraction_regex =
        FRACTION_REGEX.get_or_init(|| Regex::new(r"#?.*\?{1,2}\/\?{1,2}").unwrap());
    let square_bracket_regex =
        SQUARE_BRACKET_REGEX.get_or_init(|| Regex::new(r"\[[^\]]+\]").unwrap());
    let number_regex = NUMBER_REGEX.get_or_init(|| Regex::new(r"(0+)(\.?)(0*)").unwrap());

    let mut value = value.to_string();

    let mut format = format.replace(['"', '*'], "");

    let use_thousands = thousands_sep_regex.is_match(&format).unwrap_or(false);
    if use_thousands {
        format = format.replace("0,0", "00");
        format = format.replace("#,#", "##");
    }

    let mut scale: f64 = 1f64;

    if scale_regex.is_match(&format).unwrap_or(false) {
        let mut matches: Vec<String> = Vec::new();
        for ite in scale_regex.captures(&format).ok().flatten().unwrap().iter() {
            matches.push(ite.unwrap().as_str().to_string());
        }
        scale = f64::from(1000i32.pow(num_traits::cast(matches[2].len()).unwrap()));

        format = trailing_comma_regex.replace_all(&format, "$1").into();
    }
    if fraction_regex.is_match(&format).unwrap_or(false) {
        if value.parse::<usize>().is_err() {
            value = format_as_fraction(value.parse::<f64>().unwrap(), &format);
        }
    } else {
        value = (value.parse::<f64>().unwrap() / scale).to_string();
        format = format.replace('#', "0");
        format = format.replace('\\', "");
        format = format.replace("[$-.*]", "");
        format = format.trim().to_string();

        let m = square_bracket_regex.replace_all(&format, "");

        if number_regex.is_match(&m).unwrap_or(false) {
            let mut item: Vec<String> = Vec::new();
            for ite in number_regex.captures(&m).ok().flatten().unwrap().iter() {
                item.push(ite.unwrap().as_str().to_string());
            }
            value = format_straight_numeric_value(
                &value,
                &format,
                &item,
                use_thousands,
                r"(0+)(\.?)(0*)",
            );
        }
    }

    let re = Regex::new(r"\$[^0-9]*").unwrap();
    if re.find(&format).ok().flatten().is_some() {
        let mut item: Vec<String> = Vec::new();
        for ite in re.captures(&format).ok().flatten().unwrap().iter() {
            item.push(ite.unwrap().as_str().to_string());
        }
        value = format!("{}{}", item.first().unwrap(), value);
    }

    Cow::Owned(value)
}

fn format_straight_numeric_value(
    value: &str,
    _format: &str,
    matches: &[String],
    use_thousands: bool,
    _number_regex: &str,
) -> String {
    let mut value = value.to_string();

    let right = matches.get(3).unwrap();

    // minimun width of formatted number (including dot)
    if use_thousands {
        value = value.parse::<f64>().unwrap().separate_with_commas();
    }
    let blocks: Vec<&str> = value.split('.').collect();
    let left_value = (*blocks.first().unwrap()).to_string();
    let mut right_value = match blocks.get(1) {
        Some(v) => (*v).to_string(),
        None => String::from("0"),
    };
    if right.is_empty() {
        return left_value;
    }
    if right.len() != right_value.len() {
        if right_value == "0" {
            right_value = right.to_string();
        } else if right.len() > right_value.len() {
            let pow = 10i32.pow(num_traits::cast(right.len()).unwrap());
            right_value = format!("{}", right_value.parse::<i32>().unwrap() * pow);
        } else {
            let mut right_value_conv: String = right_value.chars().take(right.len()).collect();
            let ajst_str: String = right_value.chars().skip(right.len()).take(1).collect();
            let ajst_int = ajst_str.parse::<i32>().unwrap();
            if ajst_int > 4 {
                right_value_conv = (right_value_conv.parse::<i32>().unwrap() + 1).to_string();
            }
            right_value = right_value_conv;
        }
    }
    value = format!("{left_value}.{right_value}");
    value

    //    if use_thousands == true {
    //        value = value.parse::<f64>().unwrap().separate_with_commas();
    //        dbg!(&value);
    //        value = Regex::new(&number_regex).unwrap().replace_all(&format,
    // value.as_str());        dbg!(&value);
    //    } else {
    //        if Regex::new(r#"[0#]E[+-]0"#).unwrap().find(&format).is_some() {
    //            // Scientific format
    //            value = value.parse::<f64>().unwrap().to_string();
    //        } else if
    // Regex::new(r#"0([^\d\.]+)0"#).unwrap().find(&format).is_some() ||
    // format.find(".").is_some() {            if
    // value.parse::<f64>().unwrap() as usize as f64 ==
    // value.parse::<f64>().unwrap() && format.find(".").is_some() {
    //                let format_collect:Vec<&str> =
    // format.split('.').collect();                let pow =
    // 10i32.pow(format_collect.get(1).unwrap().len() as u32);
    // value = format!("{}", value.parse::<i32>().unwrap() * pow);
    //            }
    //            value =
    // complex_number_format_mask(&value.parse::<f64>().unwrap(), &format,
    // true);        } else {
    //            value = format!("{:0width$.len$}", value, width = min_width,
    // len = right.len());            value =
    // Regex::new(&number_regex).unwrap().replace_all(&format, value.as_str());
    //        }
    //    }
    //    value
}

#[allow(dead_code)]
fn merge_complex_number_format_masks(numbers: &[String], masks: &[String]) -> Vec<String> {
    let mut decimal_count = numbers[1].len();
    let mut post_decimal_masks: Vec<String> = Vec::new();

    for mask in masks.iter().rev() {
        post_decimal_masks.push(mask.to_string());
        decimal_count -= mask.to_string().len();
        if decimal_count == 0 {
            break;
        }
    }

    post_decimal_masks.reverse();
    vec![masks.join("."), post_decimal_masks.join(".")]
}

#[allow(dead_code)]
fn process_complex_number_format_mask(number: f64, mask: &str) -> String {
    let mut result = number.to_string();
    let mut mask = mask.to_string();
    let re = Regex::new(r"0+").unwrap();
    let mut masking_blocks: Vec<(String, usize)> = Vec::new();
    let mut masking_str: Vec<String> = Vec::new();
    let mut masking_beg: Vec<usize> = Vec::new();
    for ite in re.captures(&mask).ok().flatten().unwrap().iter() {
        masking_str.push(ite.unwrap().as_str().to_string());
    }
    for pos in re.captures(&mask).ok().flatten().unwrap().iter() {
        let beg = pos.unwrap().start();
        masking_beg.push(beg);
    }
    for i in 0..masking_str.len() {
        masking_blocks.push((
            masking_str.get(i).unwrap().clone(),
            *masking_beg.get(i).unwrap(),
        ));
    }

    if masking_blocks.len() > 1 {
        let mut number = number;
        let mut offset: usize = 0;
        for (block, pos) in masking_blocks.iter().rev() {
            let divisor = format!("{}{}", 1, block).parse::<f64>().unwrap();
            let size = block.len();
            offset = *pos;

            let block_value = format!("{:0width$}", (number % divisor), width = size);

            number /= divisor;
            let from: String = mask.chars().skip(offset).take(size).collect();
            mask = mask.replace(&from, &block_value);
        }
        if number > 0f64 {
            let from: String = mask.chars().skip(offset).collect();
            mask = mask.replace(&from, &number.to_string());
        }
        result = mask;
    }
    result
}

#[allow(dead_code)]
fn complex_number_format_mask(number: f64, mask: &str, split_on_point: bool) -> String {
    let sign = number < 0.0;
    let number = number.abs();

    if split_on_point && mask.contains('.') && number.to_string().contains('.') {
        let number_str = number.to_string();
        let numbers_as: Vec<&str> = number_str.split('.').collect();
        let mut numbers: Vec<String> = Vec::new();
        for n in numbers_as {
            numbers.push(n.to_string());
        }
        let masks_as: Vec<&str> = mask.split('.').collect();
        let mut masks: Vec<String> = Vec::new();
        for mask in masks_as {
            masks.push(mask.to_string());
        }
        if masks.len() > 2 {
            masks = merge_complex_number_format_masks(&numbers, &masks);
        }
        let result1 =
            complex_number_format_mask(numbers[0].parse::<f64>().unwrap(), &masks[0], false);
        let result2 = complex_number_format_mask(
            numbers[1]
                .chars()
                .rev()
                .collect::<String>()
                .parse::<f64>()
                .unwrap(),
            &masks[1].chars().rev().collect::<String>(),
            false,
        )
        .chars()
        .rev()
        .collect::<String>();

        return format!("{}{}.{}", if sign { "-" } else { "" }, result1, result2);
    }

    let result = process_complex_number_format_mask(number, mask);
    format!("{}{}", if sign { "-" } else { "" }, result)
}
