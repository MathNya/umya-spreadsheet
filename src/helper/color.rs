/**
 * <https://ciintelligence.blogspot.com/2012/02/converting-excel-theme-color-and-tint.html>
 */

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

#[must_use]
pub fn calc_tint(rgb: &str, tint: f64) -> String {
    let mut ms_hls = convert_rgb_to_ms_hls(rgb);
    ms_hls.l = calculate_final_lum_value(tint, f64::from(ms_hls.l));
    convert_ms_hls_to_rgb(&ms_hls)
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

    hls.l = (min + max) / 2.0;

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
    num.round() as i32
}
