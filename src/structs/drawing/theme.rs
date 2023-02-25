use super::super::StringValue;
use super::EffectList;
use super::EffectStyle;
use super::GradientFill;
use super::GradientStop;
use super::LinearGradientFill;
use super::Miter;
use super::OuterShadow;
use super::Outline;
use super::PenAlignmentValues;
use super::PercentageType;
use super::PositiveFixedPercentageType;
use super::PresetDash;
use super::PresetLineDashValues;
use super::RgbColorModelHex;
use super::SchemeColor;
use super::SchemeColorValues;
use super::SolidFill;
use super::SystemColor;
use super::SystemColorValues;
use super::ThemeElements;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Theme {
    name: StringValue,
    theme_elements: ThemeElements,
}
impl Theme {
    pub fn get_name(&self) -> &str {
        self.name.get_value()
    }

    pub fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    pub fn get_theme_elements(&self) -> &ThemeElements {
        &self.theme_elements
    }

    pub fn get_theme_elements_mut(&mut self) -> &mut ThemeElements {
        &mut self.theme_elements
    }

    pub fn set_theme_elements(&mut self, value: ThemeElements) -> &mut Self {
        self.theme_elements = value;
        self
    }

    pub(crate) fn get_defalut_value() -> Theme {
        let mut def = Theme::default();
        def.set_name("Office Theme");

        // color_scheme
        def.get_theme_elements_mut()
            .get_color_scheme_mut()
            .set_name("Office");

        let mut dk1 = SystemColor::default();
        dk1.set_val(SystemColorValues::WindowText);
        dk1.set_last_color("000000");
        def.get_theme_elements_mut()
            .get_color_scheme_mut()
            .get_dk1_mut()
            .set_system_color(dk1); // dk1

        let mut lt1 = SystemColor::default();
        lt1.set_val(SystemColorValues::Window);
        lt1.set_last_color("FFFFFF");
        def.get_theme_elements_mut()
            .get_color_scheme_mut()
            .get_lt1_mut()
            .set_system_color(lt1); // lt1

        let mut dk2 = RgbColorModelHex::default();
        dk2.set_val("44546A");
        def.get_theme_elements_mut()
            .get_color_scheme_mut()
            .get_dk2_mut()
            .set_rgb_color_model_hex(dk2); // dk2

        let mut lt2 = RgbColorModelHex::default();
        lt2.set_val("E7E6E6");
        def.get_theme_elements_mut()
            .get_color_scheme_mut()
            .get_lt2_mut()
            .set_rgb_color_model_hex(lt2); // lt2

        let mut accent1 = RgbColorModelHex::default();
        accent1.set_val("4472C4");
        def.get_theme_elements_mut()
            .get_color_scheme_mut()
            .get_accent1_mut()
            .set_rgb_color_model_hex(accent1); // accent1

        let mut accent2 = RgbColorModelHex::default();
        accent2.set_val("ED7D31");
        def.get_theme_elements_mut()
            .get_color_scheme_mut()
            .get_accent2_mut()
            .set_rgb_color_model_hex(accent2); // accent2

        let mut accent3 = RgbColorModelHex::default();
        accent3.set_val("A5A5A5");
        def.get_theme_elements_mut()
            .get_color_scheme_mut()
            .get_accent3_mut()
            .set_rgb_color_model_hex(accent3); // accent3

        let mut accent4 = RgbColorModelHex::default();
        accent4.set_val("FFC000");
        def.get_theme_elements_mut()
            .get_color_scheme_mut()
            .get_accent4_mut()
            .set_rgb_color_model_hex(accent4); // accent4

        let mut accent5 = RgbColorModelHex::default();
        accent5.set_val("5B9BD5");
        def.get_theme_elements_mut()
            .get_color_scheme_mut()
            .get_accent5_mut()
            .set_rgb_color_model_hex(accent5); // accent5

        let mut accent6 = RgbColorModelHex::default();
        accent6.set_val("70AD47");
        def.get_theme_elements_mut()
            .get_color_scheme_mut()
            .get_accent6_mut()
            .set_rgb_color_model_hex(accent6); // accent6

        let mut hlink = RgbColorModelHex::default();
        hlink.set_val("0563C1");
        def.get_theme_elements_mut()
            .get_color_scheme_mut()
            .get_hlink_mut()
            .set_rgb_color_model_hex(hlink); // hlink

        let mut fol_hlink = RgbColorModelHex::default();
        fol_hlink.set_val("954F72");
        def.get_theme_elements_mut()
            .get_color_scheme_mut()
            .get_fol_hlink_mut()
            .set_rgb_color_model_hex(fol_hlink); // folHlink

        // font_scheme
        def.get_theme_elements_mut()
            .get_font_scheme_mut()
            .set_name("Office");

        def.get_theme_elements_mut()
            .get_font_scheme_mut()
            .get_major_font_mut()
            .set_defalut_value_major();

        def.get_theme_elements_mut()
            .get_font_scheme_mut()
            .get_minor_font_mut()
            .set_defalut_value_minor();

        // format_scheme
        def.get_theme_elements_mut()
            .get_format_scheme_mut()
            .set_name("Office");

        let mut solid_fill = SolidFill::default();
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        solid_fill.set_scheme_color(scheme_color);
        def.get_theme_elements_mut()
            .get_format_scheme_mut()
            .get_fill_style_list_mut()
            .set_solid_fill(solid_fill);

        let mut gradient_fill = GradientFill::default();
        gradient_fill.set_rotate_with_shape(true);
        //
        let mut gradient_stop = GradientStop::default();
        gradient_stop.set_position(0);
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        let mut lum_mod = PercentageType::default();
        lum_mod.set_val(110000);
        scheme_color.set_luminance_modulation(lum_mod);
        let mut sat_mod = PercentageType::default();
        sat_mod.set_val(105000);
        scheme_color.set_saturation_modulation(sat_mod);
        let mut tint = PositiveFixedPercentageType::default();
        tint.set_val(67000);
        scheme_color.set_tint(tint);
        gradient_stop.set_scheme_color(scheme_color);
        gradient_fill
            .get_gradient_stop_list_mut()
            .add_gradient_stop(gradient_stop);
        //
        let mut gradient_stop = GradientStop::default();
        gradient_stop.set_position(50000);
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        let mut lum_mod = PercentageType::default();
        lum_mod.set_val(105000);
        scheme_color.set_luminance_modulation(lum_mod);
        let mut sat_mod = PercentageType::default();
        sat_mod.set_val(103000);
        scheme_color.set_saturation_modulation(sat_mod);
        let mut tint = PositiveFixedPercentageType::default();
        tint.set_val(73000);
        scheme_color.set_tint(tint);
        gradient_stop.set_scheme_color(scheme_color);
        gradient_fill
            .get_gradient_stop_list_mut()
            .add_gradient_stop(gradient_stop);
        //
        let mut gradient_stop = GradientStop::default();
        gradient_stop.set_position(100000);
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        let mut lum_mod = PercentageType::default();
        lum_mod.set_val(105000);
        scheme_color.set_luminance_modulation(lum_mod);
        let mut sat_mod = PercentageType::default();
        sat_mod.set_val(109000);
        scheme_color.set_saturation_modulation(sat_mod);
        let mut tint = PositiveFixedPercentageType::default();
        tint.set_val(81000);
        scheme_color.set_tint(tint);
        gradient_stop.set_scheme_color(scheme_color);
        gradient_fill
            .get_gradient_stop_list_mut()
            .add_gradient_stop(gradient_stop);
        //
        let mut linear_gradient_fill = LinearGradientFill::default();
        linear_gradient_fill.set_angle(5400000);
        linear_gradient_fill.set_scaled(false);
        gradient_fill.set_linear_gradient_fill(linear_gradient_fill);
        //
        def.get_theme_elements_mut()
            .get_format_scheme_mut()
            .get_fill_style_list_mut()
            .add_gradient_fill_collection(gradient_fill);

        let mut gradient_fill = GradientFill::default();
        gradient_fill.set_rotate_with_shape(true);
        //
        let mut gradient_stop = GradientStop::default();
        gradient_stop.set_position(0);
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        let mut tint = PositiveFixedPercentageType::default();
        tint.set_val(94000);
        scheme_color.set_tint(tint);
        let mut sat_mod = PercentageType::default();
        sat_mod.set_val(103000);
        scheme_color.set_saturation_modulation(sat_mod);
        let mut lum_mod = PercentageType::default();
        lum_mod.set_val(102000);
        scheme_color.set_luminance_modulation(lum_mod);
        gradient_stop.set_scheme_color(scheme_color);
        gradient_fill
            .get_gradient_stop_list_mut()
            .add_gradient_stop(gradient_stop);
        //
        let mut gradient_stop = GradientStop::default();
        gradient_stop.set_position(50000);
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        let mut sat_mod = PercentageType::default();
        sat_mod.set_val(110000);
        scheme_color.set_saturation_modulation(sat_mod);
        let mut shade = PositiveFixedPercentageType::default();
        shade.set_val(100000);
        scheme_color.set_shade(shade);
        let mut lum_mod = PercentageType::default();
        lum_mod.set_val(100000);
        scheme_color.set_luminance_modulation(lum_mod);
        gradient_stop.set_scheme_color(scheme_color);
        gradient_fill
            .get_gradient_stop_list_mut()
            .add_gradient_stop(gradient_stop);
        //
        let mut gradient_stop = GradientStop::default();
        gradient_stop.set_position(100000);
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        let mut shade = PositiveFixedPercentageType::default();
        shade.set_val(78000);
        scheme_color.set_shade(shade);
        let mut sat_mod = PercentageType::default();
        sat_mod.set_val(120000);
        scheme_color.set_saturation_modulation(sat_mod);
        let mut lum_mod = PercentageType::default();
        lum_mod.set_val(99000);
        scheme_color.set_luminance_modulation(lum_mod);
        gradient_stop.set_scheme_color(scheme_color);
        gradient_fill
            .get_gradient_stop_list_mut()
            .add_gradient_stop(gradient_stop);
        //
        let mut linear_gradient_fill = LinearGradientFill::default();
        linear_gradient_fill.set_angle(5400000);
        linear_gradient_fill.set_scaled(false);
        gradient_fill.set_linear_gradient_fill(linear_gradient_fill);
        //
        def.get_theme_elements_mut()
            .get_format_scheme_mut()
            .get_fill_style_list_mut()
            .add_gradient_fill_collection(gradient_fill);

        let mut outline = Outline::default();
        outline.set_width(6350);
        outline.set_cap_type("flat");
        outline.set_compound_line_type("sng");
        outline.set_alignment(PenAlignmentValues::Center);
        //
        let mut solid_fill = SolidFill::default();
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        solid_fill.set_scheme_color(scheme_color);
        outline.set_solid_fill(solid_fill);
        //
        let mut preset_dash = PresetDash::default();
        preset_dash.set_val(PresetLineDashValues::Solid);
        outline.set_preset_dash(preset_dash);
        //
        let mut miter = Miter::default();
        miter.set_limit(800000);
        outline.set_miter(miter);
        //
        def.get_theme_elements_mut()
            .get_format_scheme_mut()
            .get_line_style_list_mut()
            .add_outline_collection(outline);

        let mut outline = Outline::default();
        outline.set_width(12700);
        outline.set_cap_type("flat");
        outline.set_compound_line_type("sng");
        outline.set_alignment(PenAlignmentValues::Center);
        //
        let mut solid_fill = SolidFill::default();
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        solid_fill.set_scheme_color(scheme_color);
        outline.set_solid_fill(solid_fill);
        //
        let mut preset_dash = PresetDash::default();
        preset_dash.set_val(PresetLineDashValues::Solid);
        outline.set_preset_dash(preset_dash);
        //
        let mut miter = Miter::default();
        miter.set_limit(800000);
        outline.set_miter(miter);
        //
        def.get_theme_elements_mut()
            .get_format_scheme_mut()
            .get_line_style_list_mut()
            .add_outline_collection(outline);

        let mut outline = Outline::default();
        outline.set_width(19050);
        outline.set_cap_type("flat");
        outline.set_compound_line_type("sng");
        outline.set_alignment(PenAlignmentValues::Center);
        //
        let mut solid_fill = SolidFill::default();
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        solid_fill.set_scheme_color(scheme_color);
        outline.set_solid_fill(solid_fill);
        //
        let mut preset_dash = PresetDash::default();
        preset_dash.set_val(PresetLineDashValues::Solid);
        outline.set_preset_dash(preset_dash);
        //
        let mut miter = Miter::default();
        miter.set_limit(800000);
        outline.set_miter(miter);
        //
        def.get_theme_elements_mut()
            .get_format_scheme_mut()
            .get_line_style_list_mut()
            .add_outline_collection(outline);

        let mut effect_style = EffectStyle::default();
        let effect_list = EffectList::default();
        effect_style.set_effect_list(effect_list);
        def.get_theme_elements_mut()
            .get_format_scheme_mut()
            .get_effect_style_list_mut()
            .add_effect_style_collection(effect_style);

        let mut effect_style = EffectStyle::default();
        let effect_list = EffectList::default();
        effect_style.set_effect_list(effect_list);
        def.get_theme_elements_mut()
            .get_format_scheme_mut()
            .get_effect_style_list_mut()
            .add_effect_style_collection(effect_style);

        let mut effect_style = EffectStyle::default();
        let mut effect_list = EffectList::default();
        let mut outer_shadow = OuterShadow::default();
        outer_shadow.set_blur_radius("57150");
        outer_shadow.set_distance("19050");
        outer_shadow.set_direction("5400000");
        outer_shadow.set_alignment("ctr");
        outer_shadow.set_rotate_with_shape("0");
        let mut srgb_clr = RgbColorModelHex::default();
        srgb_clr.set_val("000000");
        let mut alpha = PositiveFixedPercentageType::default();
        alpha.set_val(63000);
        srgb_clr.set_alpha(alpha);
        outer_shadow.set_rgb_color_model_hex(srgb_clr);
        effect_list.set_outer_shadow(outer_shadow);
        effect_style.set_effect_list(effect_list);
        def.get_theme_elements_mut()
            .get_format_scheme_mut()
            .get_effect_style_list_mut()
            .add_effect_style_collection(effect_style);

        let mut solid_fill = SolidFill::default();
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        solid_fill.set_scheme_color(scheme_color);
        def.get_theme_elements_mut()
            .get_format_scheme_mut()
            .get_background_fill_style_list_mut()
            .add_solid_fill(solid_fill);
        //
        let mut solid_fill = SolidFill::default();
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        let mut sat_mod = PercentageType::default();
        sat_mod.set_val(170000);
        scheme_color.set_saturation_modulation(sat_mod);
        let mut tint = PositiveFixedPercentageType::default();
        tint.set_val(95000);
        scheme_color.set_tint(tint);
        solid_fill.set_scheme_color(scheme_color);
        def.get_theme_elements_mut()
            .get_format_scheme_mut()
            .get_background_fill_style_list_mut()
            .add_solid_fill(solid_fill);

        let mut gradient_fill = GradientFill::default();
        gradient_fill.set_rotate_with_shape(true);
        //
        let mut gradient_stop = GradientStop::default();
        gradient_stop.set_position(0);
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        let mut tint = PositiveFixedPercentageType::default();
        tint.set_val(93000);
        scheme_color.set_tint(tint);
        let mut shade = PositiveFixedPercentageType::default();
        shade.set_val(98000);
        scheme_color.set_shade(shade);
        let mut sat_mod = PercentageType::default();
        sat_mod.set_val(150000);
        scheme_color.set_saturation_modulation(sat_mod);
        let mut lum_mod = PercentageType::default();
        lum_mod.set_val(102000);
        scheme_color.set_luminance_modulation(lum_mod);
        gradient_stop.set_scheme_color(scheme_color);
        gradient_fill
            .get_gradient_stop_list_mut()
            .add_gradient_stop(gradient_stop);
        //
        let mut gradient_stop = GradientStop::default();
        gradient_stop.set_position(50000);
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        let mut tint = PositiveFixedPercentageType::default();
        tint.set_val(98000);
        scheme_color.set_tint(tint);
        let mut sat_mod = PercentageType::default();
        sat_mod.set_val(130000);
        scheme_color.set_saturation_modulation(sat_mod);
        let mut shade = PositiveFixedPercentageType::default();
        shade.set_val(90000);
        scheme_color.set_shade(shade);
        let mut lum_mod = PercentageType::default();
        lum_mod.set_val(103000);
        scheme_color.set_luminance_modulation(lum_mod);
        gradient_stop.set_scheme_color(scheme_color);
        gradient_fill
            .get_gradient_stop_list_mut()
            .add_gradient_stop(gradient_stop);
        //
        let mut gradient_stop = GradientStop::default();
        gradient_stop.set_position(100000);
        let mut scheme_color = SchemeColor::default();
        scheme_color.set_val(SchemeColorValues::PhColor);
        let mut shade = PositiveFixedPercentageType::default();
        shade.set_val(63000);
        scheme_color.set_shade(shade);
        let mut sat_mod = PercentageType::default();
        sat_mod.set_val(120000);
        scheme_color.set_saturation_modulation(sat_mod);
        gradient_stop.set_scheme_color(scheme_color);
        gradient_fill
            .get_gradient_stop_list_mut()
            .add_gradient_stop(gradient_stop);
        //
        let mut linear_gradient_fill = LinearGradientFill::default();
        linear_gradient_fill.set_angle(5400000);
        linear_gradient_fill.set_scaled(false);
        gradient_fill.set_linear_gradient_fill(linear_gradient_fill);
        //
        def.get_theme_elements_mut()
            .get_format_scheme_mut()
            .get_background_fill_style_list_mut()
            .add_gradient_fill_collection(gradient_fill);

        def
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"name") {
            Some(v) => {
                self.name.set_value(v);
            }
            _ => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:themeElements" => {
                        self.theme_elements.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:theme" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:theme"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:theme
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push((
            "xmlns:a",
            "http://schemas.openxmlformats.org/drawingml/2006/main",
        ));
        if self.name.has_value() {
            attributes.push(("name", self.name.get_value_string()));
        }
        write_start_tag(writer, "a:theme", attributes, false);

        // a:themeElements
        let _ = &self.theme_elements.write_to(writer);

        // a:objectDefaults
        write_start_tag(writer, "a:objectDefaults", vec![], true);

        // a:extraClrSchemeLst
        write_start_tag(writer, "a:extraClrSchemeLst", vec![], true);

        write_end_tag(writer, "a:theme");
    }
}
