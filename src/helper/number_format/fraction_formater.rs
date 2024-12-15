pub(crate) fn format_as_fraction(value: f64, format: &str) -> String {
    let sign = if value < 0f64 { "-" } else { "" };

    let integer_part = value.abs().floor();
    let decimal_part = (value.abs() % 1f64)
        .to_string()
        .replace("0.", "")
        .parse::<f64>()
        .unwrap();
    let decimal_length = decimal_part.to_string().len();
    let decimal_divisor = 10f64.powi(decimal_length as i32);

    let gcd = gcd(decimal_part, decimal_divisor);

    let mut adjusted_decimal_part = decimal_part / gcd;
    let adjusted_decimal_divisor = decimal_divisor / gcd;

    let result: String;
    if format.contains('0') {
        return format!(
            "{}{} {}/{}",
            &sign, &integer_part, &adjusted_decimal_part, &adjusted_decimal_divisor
        );
    }

    if format.contains('#') {
        if integer_part == 0f64 {
            result = format!(
                "{}{}/{}",
                &sign, &adjusted_decimal_part, &adjusted_decimal_divisor
            );
        } else {
            result = format!(
                "{}{} {}/{}",
                &sign, &integer_part, &adjusted_decimal_part, &adjusted_decimal_divisor
            );
        }
        return result;
    }

    let check_format: String = format.chars().take(3).collect();
    if check_format == "? ?" {
        let mut integer_part_str = integer_part.to_string();
        if integer_part == 0f64 {
            integer_part_str = String::from("");
        }
        result = format!(
            "{}{} {}/{}",
            &sign, &integer_part_str, &adjusted_decimal_part, &adjusted_decimal_divisor
        );
    } else {
        adjusted_decimal_part += integer_part * adjusted_decimal_divisor;
        result = format!(
            "{}{}/{}",
            &sign, &adjusted_decimal_part, &adjusted_decimal_divisor
        );
    }

    result
}

#[inline]
fn gcd(a: f64, b: f64) -> f64 {
    if b == 0f64 {
        a
    } else {
        gcd(b, a % b)
    }
}
