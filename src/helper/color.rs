/// <https://ciintelligence.blogspot.com/2012/02/converting-excel-theme-color-and-tint.html>

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct HlsColor {
    pub h: f64,
    pub l: f64,
    pub s: f64,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct MsHlsColor {
    pub h: i32,
    pub l: i32,
    pub s: i32,
}

const RGBMAX: f64 = 255.0;
const HLSMAX: f64 = 255.0;
const EXCEL_RGB_MAX: i32 = 255;
const EXCEL_HLS_MAX: i32 = 240;
const EXCEL_TINT_SHORT_MAX: f64 = 32_767.0;
const PERCENT_SCALE: f64 = 100.0;

#[must_use]
pub fn calc_tint(rgb: &str, tint: f64) -> String {
    let tint = normalize_excel_tint(tint);
    let (hue, luminance, saturation) = convert_rgb_to_excel_hls(rgb);
    let moved = if tint < 0.0 {
        f64::from(luminance) * (1.0 + tint)
    } else {
        f64::from(luminance) * (1.0 - tint) + f64::from(EXCEL_HLS_MAX) * tint
    };
    convert_excel_hls_to_rgb(
        hue,
        num_traits::cast::<f64, i32>(moved)
            .unwrap()
            .clamp(0, EXCEL_HLS_MAX),
        saturation,
    )
}

/// Recover near-whole-percent tints produced by Excel-compatible encoders.
///
/// Workbook fixtures commonly express 80% as `0.79998168889431442`, matching
/// one step of the legacy signed-short tint range. Treat values within that
/// step as their nominal whole percentage, but retain more distant arbitrary
/// OOXML values such as `0.7999`.
fn normalize_excel_tint(tint: f64) -> f64 {
    let whole_percent = (tint * PERCENT_SCALE).round() / PERCENT_SCALE;
    if (tint - whole_percent).abs() <= 1.0 / EXCEL_TINT_SHORT_MAX {
        whole_percent
    } else {
        tint
    }
}

/// Convert RGB to the 240-step integer HLS space Excel uses for theme tints.
///
/// Using the floating-point 255-step conversion below changes native export
/// colours by one level. The RGB/HLS conversions round at their integer
/// stages, while the tint-adjusted luminance is deliberately truncated before
/// the reverse conversion.
fn convert_rgb_to_excel_hls(rgb: &str) -> (i32, i32, i32) {
    let (red, green, blue) = split_rgb(rgb);
    let brightest = red.max(green).max(blue);
    let darkest = red.min(green).min(blue);
    let sum = brightest + darkest;
    let span = brightest - darkest;
    let luminance = (sum * EXCEL_HLS_MAX + EXCEL_RGB_MAX) / (2 * EXCEL_RGB_MAX);
    if span == 0 {
        return (0, luminance, 0);
    }

    let saturation = if luminance <= EXCEL_HLS_MAX / 2 {
        (span * EXCEL_HLS_MAX + sum / 2) / sum
    } else {
        (span * EXCEL_HLS_MAX + (2 * EXCEL_RGB_MAX - sum) / 2) / (2 * EXCEL_RGB_MAX - sum)
    };
    let distance = |channel: i32| ((brightest - channel) * (EXCEL_HLS_MAX / 6) + span / 2) / span;
    let hue = if red == brightest {
        distance(blue) - distance(green)
    } else if green == brightest {
        EXCEL_HLS_MAX / 3 + distance(red) - distance(blue)
    } else {
        2 * EXCEL_HLS_MAX / 3 + distance(green) - distance(red)
    };
    (hue.rem_euclid(EXCEL_HLS_MAX), luminance, saturation)
}

/// Convert Excel's integer HLS representation back to RGB.
fn convert_excel_hls_to_rgb(hue: i32, luminance: i32, saturation: i32) -> String {
    let upper = if luminance <= EXCEL_HLS_MAX / 2 {
        (luminance * (EXCEL_HLS_MAX + saturation) + EXCEL_HLS_MAX / 2) / EXCEL_HLS_MAX
    } else {
        luminance + saturation - (luminance * saturation + EXCEL_HLS_MAX / 2) / EXCEL_HLS_MAX
    };
    let lower = 2 * luminance - upper;
    let channel = |hue_offset: i32| {
        let level = excel_hue_level(lower, upper, hue + hue_offset);
        ((level * EXCEL_RGB_MAX + EXCEL_HLS_MAX / 2) / EXCEL_HLS_MAX).clamp(0, EXCEL_RGB_MAX)
    };
    join_rgb(
        channel(EXCEL_HLS_MAX / 3),
        channel(0),
        channel(-EXCEL_HLS_MAX / 3),
    )
}

/// Interpolate one RGB channel around Excel's integer hue wheel.
fn excel_hue_level(lower: i32, upper: i32, hue: i32) -> i32 {
    let hue = hue.rem_euclid(EXCEL_HLS_MAX);
    let sixth = EXCEL_HLS_MAX / 6;
    if hue < sixth {
        lower + ((upper - lower) * hue + EXCEL_HLS_MAX / 12) / sixth
    } else if hue < EXCEL_HLS_MAX / 2 {
        upper
    } else if hue < 2 * EXCEL_HLS_MAX / 3 {
        lower + ((upper - lower) * (2 * EXCEL_HLS_MAX / 3 - hue) + EXCEL_HLS_MAX / 12) / sixth
    } else {
        lower
    }
}

#[must_use]
pub fn calculate_final_lum_value(tint: f64, lum: f64) -> i32 {
    let lum1 = if tint < 0.0 {
        lum * (1.0 + tint)
    } else {
        lum * (1.0 - tint) + (HLSMAX - HLSMAX * (1.0 - tint))
    };

    to_i32(lum1)
}

#[must_use]
pub fn split_rgb(rgb: &str) -> (i32, i32, i32) {
    let r = i32::from_str_radix(&rgb[0..2], 16).unwrap();
    let g = i32::from_str_radix(&rgb[2..4], 16).unwrap();
    let b = i32::from_str_radix(&rgb[4..6], 16).unwrap();
    (r, g, b)
}

#[inline]
#[must_use]
pub fn join_rgb(r: i32, g: i32, b: i32) -> String {
    format!("{r:02X}{g:02X}{b:02X}")
}

#[must_use]
pub fn convert_rgb_to_ms_hls(rgb: &str) -> MsHlsColor {
    let hls = convert_rgb_to_hls(rgb);
    MsHlsColor {
        h: to_i32(hls.h * HLSMAX),
        l: to_i32(hls.l * HLSMAX),
        s: to_i32(hls.s * HLSMAX),
    }
}

#[must_use]
#[allow(clippy::float_cmp)]
pub fn convert_rgb_to_hls(rgb: &str) -> HlsColor {
    let mut hls = HlsColor::default();

    let (r_i, g_i, b_i) = split_rgb(rgb);

    let r = f64::from(r_i) / RGBMAX;
    let g = f64::from(g_i) / RGBMAX;
    let b = f64::from(b_i) / RGBMAX;

    let mut min = r;
    if min > g {
        min = g;
    }
    if min > b {
        min = b;
    }

    let mut max = r;
    if max < g {
        max = g;
    }
    if max < b {
        max = b;
    }

    let delta = max - min;

    if max == min {
        hls.h = 0.0;
        hls.s = 0.0;
        hls.l = max;
        return hls;
    }

    hls.l = f64::midpoint(min, max);

    if hls.l <= 0.5 {
        hls.s = delta / (max + min);
    } else {
        hls.s = delta / (2.0 - max - min);
    }

    let rc = (max - r) / delta;
    let gc = (max - g) / delta;
    let bc = (max - b) / delta;

    if r == max {
        hls.h = bc - gc;
    } else if g == max {
        hls.h = 2.0 + rc - bc;
    } else {
        hls.h = 4.0 + gc - rc;
    }

    hls.h = positive_decimal_part(hls.h / 6.0);

    hls
}

#[must_use]
pub fn convert_ms_hls_to_rgb(ms_hls: &MsHlsColor) -> String {
    let hls = HlsColor {
        h: (f64::from(ms_hls.h) / HLSMAX),
        l: (f64::from(ms_hls.l) / HLSMAX),
        s: (f64::from(ms_hls.s) / HLSMAX),
    };
    convert_hls_to_rgb(&hls)
}

#[must_use]
pub fn convert_hls_to_rgb(hls: &HlsColor) -> String {
    if hls.s == 0.0 {
        let rtn_l = to_i32(hls.l * RGBMAX);
        return join_rgb(rtn_l, rtn_l, rtn_l);
    }

    let t1 = if hls.l < 0.5 {
        hls.l * (1.0 + hls.s)
    } else {
        hls.l + hls.s - (hls.l * hls.s)
    };

    let t2 = 2.0 * hls.l - t1;
    let h = hls.h;
    let t_r = h + (1.0 / 3.0);
    let r = set_color(t1, t2, t_r);
    let t_g = h;
    let g = set_color(t1, t2, t_g);
    let t_b = h - (1.0 / 3.0);
    let b = set_color(t1, t2, t_b);

    let rtn_r = to_i32(r * RGBMAX);
    let rtn_g = to_i32(g * RGBMAX);
    let rtn_b = to_i32(b * RGBMAX);
    join_rgb(rtn_r, rtn_g, rtn_b)
}

#[must_use]
pub fn set_color(t1: f64, t2: f64, t3: f64) -> f64 {
    let t3 = positive_decimal_part(t3);

    if 6.0 * t3 < 1.0 {
        t2 + (t1 - t2) * 6.0 * t3
    } else if 2.0 * t3 < 1.0 {
        t1
    } else if 3.0 * t3 < 2.0 {
        t2 + (t1 - t2) * ((2.0 / 3.0) - t3) * 6.0
    } else {
        t2
    }
}

#[inline]
fn positive_decimal_part(hue: f64) -> f64 {
    let hue = hue % 1.0;

    if hue >= 0.0 {
        return hue;
    }
    1.0 + hue
}

#[inline]
fn to_i32(num: f64) -> i32 {
    num_traits::cast(num.round()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{
        EXCEL_TINT_SHORT_MAX,
        calc_tint,
        normalize_excel_tint,
    };

    #[test]
    fn excel_theme_tints_use_the_native_hls_quantization() {
        let measured = [
            ("DAB6BA", 0.7999, "F8EFF0"),
            ("4F81BD", 0.8, "DCE6F1"),
            ("4F81BD", 0.4, "95B3D7"),
            ("C0504D", 0.8, "F2DCDB"),
            ("C0504D", 0.4, "DA9694"),
            ("5B9BD5", 0.8, "DDEBF7"),
            ("5B9BD5", 0.4, "9BC2E6"),
            ("FFFFFF", -0.15, "D9D9D9"),
        ];

        for (source, tint, expected) in measured {
            assert_eq!(
                calc_tint(source, tint),
                expected,
                "{source} tinted by {tint}"
            );
        }
    }

    #[test]
    fn excel_encoded_percent_tints_are_recovered_before_hls_math() {
        let measured = [
            ("000000", 0.499_984_740_745_262, "808080"),
            ("4EA72E", 0.799_981_688_894_314_4, "DAF2D0"),
            ("4EA72E", 0.599_993_896_298_104_8, "B5E6A2"),
            ("4EA72E", 0.399_975_585_192_419_2, "8ED973"),
            ("4E3B30", 0.399_975_585_192_419_2, "A78570"),
        ];

        for (source, tint, expected) in measured {
            assert_eq!(
                calc_tint(source, tint),
                expected,
                "{source} tinted by {tint}"
            );
        }

        assert_eq!(normalize_excel_tint(0.799_981_688_894_314_4), 0.8);
        assert_eq!(normalize_excel_tint(0.7999), 0.7999);

        let threshold = 1.0 / EXCEL_TINT_SHORT_MAX;
        assert_eq!(normalize_excel_tint(0.8 - threshold), 0.8);
        assert_eq!(
            normalize_excel_tint(0.8 - threshold - f64::EPSILON),
            0.8 - threshold - f64::EPSILON
        );
    }

    #[test]
    fn tint_adjusted_luminance_is_truncated_before_rgb_conversion() {
        // Rounding the 229.5948 luminance would produce F8F1F2 instead.
        assert_eq!(calc_tint("DAB6BA", 0.7999), "F8EFF0");
    }
}
