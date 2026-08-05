use std::borrow::Cow;

use super::number_formater::round_decimal_string;

pub(crate) fn format_as_percentage(value: f64, format: &str) -> Cow<'_, str> {
    let format = format.replace('%', "");
    let blocks: Vec<&str> = format.split('.').collect();
    let decimals = match blocks.get(1) {
        Some(v) => v.len(),
        None => 0,
    };
    // Shift the decimal string right by two places (×100) and round half
    // away from zero. Multiplying the binary double corrupts ties: 1.065
    // becomes 106.4999… and displays as 106% where Excel shows 107%.
    let shifted = shift_decimal_right_two(&value.to_string());
    Cow::Owned(format!("{}%", round_decimal_string(&shifted, decimals)))
}

/// Multiply a plain decimal string by 100 by moving the decimal point,
/// keeping the digits exact. Falls back to float math for exponent notation.
fn shift_decimal_right_two(value: &str) -> String {
    if value.contains(['e', 'E']) {
        return (value.parse::<f64>().unwrap_or(0.0) * 100.0).to_string();
    }
    let (sign, digits) = match value.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", value),
    };
    let (int_part, frac_part) = digits.split_once('.').unwrap_or((digits, ""));
    let mut frac = frac_part.to_string();
    while frac.len() < 2 {
        frac.push('0');
    }
    let moved = &frac[..2];
    let rest = &frac[2..];
    let combined = format!("{int_part}{moved}");
    let trimmed = combined.trim_start_matches('0');
    let int_final = if trimmed.is_empty() { "0" } else { trimmed };
    if rest.is_empty() {
        format!("{sign}{int_final}")
    } else {
        format!("{sign}{int_final}.{rest}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_as_percentage_keeps_decimal_precision() {
        // Rounding to an integer before applying the decimal format code
        // turned 17.309...% into "17.0%".
        assert_eq!(
            format_as_percentage(0.173_090_909_090_909_1, "0.0%"),
            "17.3%"
        );
    }

    #[test]
    fn format_as_percentage_integer_format() {
        assert_eq!(format_as_percentage(0.173_090_909_090_909_1, "0%"), "17%");
    }

    #[test]
    fn format_as_percentage_two_decimals() {
        assert_eq!(format_as_percentage(0.5, "0.00%"), "50.00%");
    }

    #[test]
    fn format_as_percentage_rounds_half_away_from_zero() {
        // 21,300 / 20,000 = 1.065 → Excel displays 107%, not 106%: the
        // binary double is 106.4999…, but Excel rounds the 15-digit decimal
        // display value (106.5) away from zero.
        assert_eq!(format_as_percentage(1.065, "0%"), "107%");
        assert_eq!(format_as_percentage(0.125, "0.0%"), "12.5%");
        assert_eq!(format_as_percentage(0.106_499_999_999_999_99, "0%"), "11%");
    }
}
