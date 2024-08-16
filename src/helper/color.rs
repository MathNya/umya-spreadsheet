/**
 * https://ciintelligence.blogspot.com/2012/02/converting-excel-theme-color-and-tint.html
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

const RGBMAX: f64 = 255f64;
const HLSMAX: f64 = 240f64;

pub fn calc_tint(rgb: &str, tint: &f64) -> String {
    let mut ms_hls = convert_rgb_to_ms_hls(rgb);
    let calculate_final_lum_value = calculate_final_lum_value(tint, &(ms_hls.l as f64));
    ms_hls.l = calculate_final_lum_value;
    convert_ms_hls_to_rgb(&ms_hls)
}

pub fn calculate_final_lum_value(tint: &f64, lum: &f64) -> i32 {
    let mut lum1 = 0.0;

    if tint < &0.0 {
        lum1 = lum * (1.0 + tint);
    } else {
        lum1 = lum * (1.0 - tint) + (HLSMAX - HLSMAX * (1.0 - tint));
    }

    return to_i32(lum1);
}

pub fn split_rgb(rgb: &str) -> (i32, i32, i32) {
    let r_str = rgb.chars().skip(0).take(2).collect::<String>();
    let g_str = rgb.chars().skip(2).take(2).collect::<String>();
    let b_str = rgb.chars().skip(4).take(2).collect::<String>();
    let r = i32::from_str_radix(&r_str, 16).unwrap();
    let g = i32::from_str_radix(&g_str, 16).unwrap();
    let b = i32::from_str_radix(&b_str, 16).unwrap();
    (r, g, b)
}

pub fn join_rgb(r: &i32, g: &i32, b: &i32) -> String {
    format!("{:02X}{:02X}{:02X}", r, g, b)
}

pub fn convert_rgb_to_ms_hls(rgb: &str) -> MsHlsColor {
    let hls = convert_rgb_to_hls(rgb);

    let mut ms_hls = MsHlsColor::default();
    ms_hls.h = to_i32(hls.h * self::HLSMAX);
    ms_hls.l = to_i32(hls.l * self::HLSMAX);
    ms_hls.s = to_i32(hls.s * self::HLSMAX);

    ms_hls
}

pub fn convert_rgb_to_hls(rgb: &str) -> HlsColor {
    let mut hls = HlsColor::default();

    let (r_i, g_i, b_i) = split_rgb(rgb);

    let r = r_i as f64 / RGBMAX;
    let g = g_i as f64 / RGBMAX;
    let b = b_i as f64 / RGBMAX;

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

    if hls.l < 0.5 {
        hls.s = delta / (max + min);
    } else {
        hls.s = delta / (2.0 - max - min);
    }

    if r == max {
        hls.h = (g - b) / delta;
    }

    if g == max {
        hls.h = 2.0 + (b - r) / delta;
    }

    if b == max {
        hls.h = 4.0 + (r - g) / delta;
    }

    hls.h *= 60.0;

    if hls.h < 0.0 {
        hls.h += 360.0;
    }

    return hls;
}

pub fn convert_ms_hls_to_rgb(ms_hls: &MsHlsColor) -> String {
    let mut hls = HlsColor::default();
    hls.h = ms_hls.h as f64 / self::HLSMAX;
    hls.l = ms_hls.l as f64 / self::HLSMAX;
    hls.s = ms_hls.s as f64 / self::HLSMAX;
    convert_hls_to_rgb(&hls)
}

pub fn convert_hls_to_rgb(hls: &HlsColor) -> String {
    if hls.s == 0.0 {
        let rtn_l = to_i32(hls.l * RGBMAX);
        return join_rgb(&rtn_l, &rtn_l, &rtn_l);
    }

    let t1 = if hls.l < 0.5 {
        hls.l * (1.0 + hls.s)
    } else {
        hls.l + hls.s - (hls.l * hls.s)
    };

    let t2 = 2.0 * hls.l - t1;
    let h = hls.h / 360.0;
    let t_r = h + (1.0 / 3.0);
    let r = set_color(&t1, &t2, &t_r);
    let t_g = h;
    let g = set_color(&t1, &t2, &t_g);
    let t_b = h - (1.0 / 3.0);
    let b = set_color(&t1, &t2, &t_b);

    let rtn_r = to_i32(r * RGBMAX);
    let rtn_g = to_i32(g * RGBMAX);
    let rtn_b = to_i32(b * RGBMAX);
    join_rgb(&rtn_r, &rtn_g, &rtn_b)
}

pub fn set_color(t1: &f64, t2: &f64, t3: &f64) -> f64 {
    let mut t1 = t1.clone();
    let mut t2 = t2.clone();
    let mut t3 = t3 % 1.0;
    if t3 < 1.0 {
        t3 = 1.0 + t3;
    }

    let mut color: f64 = 0.0;

    if 6.0 * t3 < 1.0 {
        color = t2 + (t1 - t2) * 6.0 * t3;
    } else if 2.0 * t3 < 1.0 {
        color = t1;
    } else if 3.0 * t3 < 2.0 {
        color = t2 + (t1 - t2) * ((2.0 / 3.0) - t3) * 6.0;
    } else {
        color = t2;
    }
    color
}

fn to_i32(num: f64) -> i32 {
    num.round() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_tint_test() {
        assert_eq!(calc_tint("4E3B30", &0.4), "A78470");
    }

    #[test]
    fn split_rgb_test() {
        assert_eq!(split_rgb("77602D"), (119, 96, 45));
        assert_eq!(split_rgb("1562A9"), (21, 98, 169));
    }

    #[test]
    fn convert_rgb_to_hls_test() {
        let hls = convert_rgb_to_hls("77602D");
        assert_eq!(hls.h, 41.351351351351354);
        assert_eq!(hls.s, 0.45121951219512185);
        assert_eq!(hls.l, 0.3215686274509804);

        let hls = convert_rgb_to_hls("1562A9");
        assert_eq!(hls.h, 208.7837837837838);
        assert_eq!(hls.s, 0.7789473684210525);
        assert_eq!(hls.l, 0.37254901960784315);
    }
}
