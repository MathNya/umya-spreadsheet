use std::borrow::Cow;

use super::fraction_formater::format_as_fraction;
use crate::helper::utils::compile_regex;

pub(crate) fn format_as_number(value: f64, format: &str) -> Cow<'_, str> {
    let thousands_sep_regex = compile_regex!(r"(#,#|0,0)");
    let scale_regex = compile_regex!(r"(#|0)(,+)");
    let trailing_comma_regex = compile_regex!("(#|0),+");
    let fraction_regex = compile_regex!(r"#?.*\?{1,2}\/\?{1,2}");
    let square_bracket_regex = compile_regex!(r"\[[^\]]+\]");
    let number_regex = compile_regex!(r"(0+)(\.?)(0*)");

    let mut value = value.to_string();

    let mut format = format.replace(['"', '*'], "");

    let use_thousands = thousands_sep_regex.is_match(&format).unwrap_or(false);
    if use_thousands {
        format = format.replace("0,0", "00");
        format = format.replace("#,#", "##");
    }

    let mut scale: f64 = 1f64;

    if scale_regex.is_match(&format).unwrap_or(false) {
        let mut matches: Vec<&str> = Vec::new();
        for ite in scale_regex.captures(&format).ok().flatten().unwrap().iter() {
            matches.push(ite.unwrap().as_str());
        }
        scale = f64::from(1000i32.pow(num_traits::cast(matches[2].len()).unwrap()));

        format = trailing_comma_regex.replace_all(&format, "$1").into();
    }
    if fraction_regex.is_match(&format).unwrap_or(false) {
        if value.parse::<usize>().is_err() {
            value = format_as_fraction(value.parse::<f64>().unwrap_or(0.0), &format);
        }
    } else {
        value = (value.parse::<f64>().unwrap_or(0.0) / scale).to_string();
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

    let re = compile_regex!(r"\$[^0-9]*");
    if re.find(&format).ok().flatten().is_some() {
        let item: Vec<&str> = re
            .captures(&format)
            .ok()
            .flatten()
            .unwrap()
            .iter()
            .map(|ite| ite.unwrap().as_str())
            .collect();
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
    let empty = String::new();
    let right = matches.get(3).unwrap_or(&empty);

    // Excel rounds the decimal display value half away from zero to the
    // format's fractional width. Work on the decimal string directly:
    // routing ties back through binary floats corrupts them
    // (39.105 * 100 = 3910.4999… would round down).
    let value = round_decimal_string(value, right.len());

    if use_thousands {
        return group_thousands(&value);
    }
    value
}

/// Round a plain decimal string (as produced by `f64::to_string`) to
/// `decimals` fractional digits, half away from zero, in pure decimal
/// arithmetic. Zero-pads when the value has fewer fractional digits than the
/// format asks for. Falls back to float formatting for exponent notation.
pub(crate) fn round_decimal_string(value: &str, decimals: usize) -> String {
    if value.contains(['e', 'E']) {
        let parsed = value.parse::<f64>().unwrap_or(0.0);
        return format!("{:.*}", decimals, parsed);
    }

    let (sign, digits) = match value.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", value),
    };
    let (int_part, frac_part) = digits.split_once('.').unwrap_or((digits, ""));

    if frac_part.len() <= decimals {
        let mut result = format!("{sign}{int_part}");
        if decimals > 0 {
            result.push('.');
            result.push_str(frac_part);
            result.push_str(&"0".repeat(decimals - frac_part.len()));
        }
        return result;
    }

    let mut kept: Vec<u8> = int_part
        .bytes()
        .chain(frac_part.bytes().take(decimals))
        .map(|b| b - b'0')
        .collect();
    if frac_part.as_bytes()[decimals] >= b'5' {
        let mut carry = 1u8;
        for digit in kept.iter_mut().rev() {
            *digit += carry;
            carry = *digit / 10;
            *digit %= 10;
            if carry == 0 {
                break;
            }
        }
        if carry > 0 {
            kept.insert(0, carry);
        }
    }

    let all: String = kept.iter().map(|d| char::from(b'0' + d)).collect();
    let split_at = all.len() - decimals;
    let int_digits = all[..split_at].trim_start_matches('0');
    let int_digits = if int_digits.is_empty() { "0" } else { int_digits };
    let mut result = format!("{sign}{int_digits}");
    if decimals > 0 {
        result.push('.');
        result.push_str(&all[split_at..]);
    }
    result
}

/// Insert comma group separators into the integer part of a plain decimal
/// string, preserving sign and fraction. String-based so values beyond the
/// integer range keep their digits.
pub(crate) fn group_thousands(value: &str) -> String {
    let (sign, digits) = match value.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", value),
    };
    let (int_part, frac_part) = match digits.split_once('.') {
        Some((int_part, frac_part)) => (int_part, Some(frac_part)),
        None => (digits, None),
    };

    let mut grouped = String::with_capacity(int_part.len() + int_part.len() / 3);
    for (index, ch) in int_part.chars().enumerate() {
        if index > 0 && (int_part.len() - index) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(ch);
    }

    match frac_part {
        Some(frac) => format!("{sign}{grouped}.{frac}"),
        None => format!("{sign}{grouped}"),
    }
}

#[allow(dead_code)]
fn merge_complex_number_format_masks(numbers: &[String], masks: &[String]) -> Vec<String> {
    let mut decimal_count = numbers[1].len();
    let mut post_decimal_masks: Vec<&str> = Vec::new();

    for mask in masks.iter().rev() {
        post_decimal_masks.push(mask);
        decimal_count -= mask.clone().len();
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
    let re = compile_regex!(r"0+");
    let mut masking_blocks: Vec<(&str, usize)> = Vec::new();
    let mut masking_str: Vec<&str> = Vec::new();
    let mut masking_beg: Vec<usize> = Vec::new();
    for ite in re.captures(mask).ok().flatten().unwrap().iter() {
        masking_str.push(ite.unwrap().as_str());
    }
    for pos in re.captures(mask).ok().flatten().unwrap().iter() {
        let beg = pos.unwrap().start();
        masking_beg.push(beg);
    }
    for i in 0..masking_str.len() {
        masking_blocks.push((masking_str.get(i).unwrap(), *masking_beg.get(i).unwrap()));
    }
    let mut mask = mask.to_string();

    if masking_blocks.len() > 1 {
        let mut number = number;
        let mut offset: usize = 0;
        for (block, pos) in masking_blocks.iter().rev() {
            let divisor = format!("{}{}", 1, block).parse::<f64>().unwrap_or(1.0);
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
            complex_number_format_mask(numbers[0].parse::<f64>().unwrap_or(0.0), &masks[0], false);
        let result2 = complex_number_format_mask(
            numbers[1]
                .chars()
                .rev()
                .collect::<String>()
                .parse::<f64>()
                .unwrap_or(0.0),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_as_number_rounds_half_away_from_zero() {
        // Excel rounds display values; truncating turned 107310.6 into 107,310.
        assert_eq!(format_as_number(107310.6, "#,##0"), "107,311");
        assert_eq!(format_as_number(12.5, "0"), "13");
        assert_eq!(format_as_number(-12.5, "0"), "-13");
        assert_eq!(format_as_number(99999.5, "0"), "100000");
    }

    #[test]
    fn format_as_number_pads_decimals_to_format_width() {
        // 39.1 with a two-decimal format printed "39.100" (the fractional
        // digits were multiplied instead of zero-padded).
        assert_eq!(format_as_number(39.1, "0.00"), "39.10");
        assert_eq!(format_as_number(2229.7, "#,##0.00"), "2,229.70");
        assert_eq!(format_as_number(84.2, "0.00"), "84.20");
        assert_eq!(format_as_number(39.0, "0.00"), "39.00");
    }

    #[test]
    fn format_as_number_rounds_excess_decimals() {
        assert_eq!(format_as_number(39.105, "0.00"), "39.11");
        assert_eq!(format_as_number(0.125, "0.00"), "0.13");
        assert_eq!(format_as_number(1.005, "0.00"), "1.01");
        assert_eq!(format_as_number(57.67, "0.00"), "57.67");
    }

    #[test]
    fn format_as_number_keeps_currency_prefix() {
        assert_eq!(format_as_number(39.1, "$0.00"), "$39.10");
        assert_eq!(format_as_number(107310.6, "$#,##0"), "$107,311");
    }

    #[test]
    fn format_as_number_thousands_grouping_survives_rounding() {
        assert_eq!(format_as_number(999999.5, "#,##0"), "1,000,000");
        assert_eq!(format_as_number(1234567.891, "#,##0.00"), "1,234,567.89");
        assert_eq!(format_as_number(-1234.5, "#,##0"), "-1,235");
    }
}
