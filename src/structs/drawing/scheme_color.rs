// a:schemeClr
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    super::EnumValue,
    PercentageType,
    PositiveFixedPercentageType,
    SchemeColorValues,
};
use crate::{
    reader::driver::{
        get_attribute,
        xml_read_loop,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct SchemeColor {
    val:                   EnumValue<SchemeColorValues>,
    luminance:             Option<PercentageType>,
    luminance_modulation:  Option<PercentageType>,
    luminance_offset:      Option<PercentageType>,
    saturation:            Option<PercentageType>,
    saturation_modulation: Option<PercentageType>,
    shade:                 Option<PositiveFixedPercentageType>,
    alpha:                 Option<PositiveFixedPercentageType>,
    tint:                  Option<PositiveFixedPercentageType>,
}

impl SchemeColor {
    #[inline]
    #[must_use]
    pub fn val(&self) -> &SchemeColorValues {
        self.val.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use val()")]
    pub fn get_val(&self) -> &SchemeColorValues {
        self.val()
    }

    #[inline]
    pub fn set_val(&mut self, value: SchemeColorValues) -> &mut Self {
        self.val.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn luminance(&self) -> Option<&PercentageType> {
        self.luminance.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use luminance()")]
    pub fn get_luminance(&self) -> Option<&PercentageType> {
        self.luminance()
    }

    #[inline]
    pub fn luminance_mut(&mut self) -> Option<&mut PercentageType> {
        self.luminance.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use luminance_mut()")]
    pub fn get_luminance_mut(&mut self) -> Option<&mut PercentageType> {
        self.luminance_mut()
    }

    #[inline]
    pub fn set_luminance(&mut self, value: PercentageType) {
        self.luminance = Some(value);
    }

    #[inline]
    #[must_use]
    pub fn luminance_modulation(&self) -> Option<&PercentageType> {
        self.luminance_modulation.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use luminance_modulation()")]
    pub fn get_luminance_modulation(&self) -> Option<&PercentageType> {
        self.luminance_modulation()
    }

    #[inline]
    pub fn luminance_modulation_mut(&mut self) -> Option<&mut PercentageType> {
        self.luminance_modulation.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use luminance_modulation_mut()")]
    pub fn get_luminance_modulation_mut(&mut self) -> Option<&mut PercentageType> {
        self.luminance_modulation_mut()
    }

    #[inline]
    pub fn set_luminance_modulation(&mut self, value: PercentageType) {
        self.luminance_modulation = Some(value);
    }

    #[inline]
    #[must_use]
    pub fn luminance_offset(&self) -> Option<&PercentageType> {
        self.luminance_offset.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use luminance_offset()")]
    pub fn get_luminance_offset(&self) -> Option<&PercentageType> {
        self.luminance_offset()
    }

    #[inline]
    pub fn luminance_offset_mut(&mut self) -> Option<&mut PercentageType> {
        self.luminance_offset.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use luminance_offset_mut()")]
    pub fn get_luminance_offset_mut(&mut self) -> Option<&mut PercentageType> {
        self.luminance_offset_mut()
    }

    #[inline]
    pub fn set_luminance_offset(&mut self, value: PercentageType) {
        self.luminance_offset = Some(value);
    }

    #[inline]
    #[must_use]
    pub fn saturation(&self) -> Option<&PercentageType> {
        self.saturation.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use saturation()")]
    pub fn get_saturation(&self) -> Option<&PercentageType> {
        self.saturation()
    }

    #[inline]
    pub fn saturation_mut(&mut self) -> Option<&mut PercentageType> {
        self.saturation.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use saturation_mut()")]
    pub fn get_saturation_mut(&mut self) -> Option<&mut PercentageType> {
        self.saturation_mut()
    }

    #[inline]
    pub fn set_saturation(&mut self, value: PercentageType) {
        self.saturation = Some(value);
    }

    #[inline]
    #[must_use]
    pub fn saturation_modulation(&self) -> Option<&PercentageType> {
        self.saturation_modulation.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use saturation_modulation()")]
    pub fn get_saturation_modulation(&self) -> Option<&PercentageType> {
        self.saturation_modulation()
    }

    #[inline]
    pub fn saturation_modulation_mut(&mut self) -> Option<&mut PercentageType> {
        self.saturation_modulation.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use saturation_modulation_mut()")]
    pub fn get_saturation_modulation_mut(&mut self) -> Option<&mut PercentageType> {
        self.saturation_modulation_mut()
    }

    #[inline]
    pub fn set_saturation_modulation(&mut self, value: PercentageType) {
        self.saturation_modulation = Some(value);
    }

    #[inline]
    #[must_use]
    pub fn shade(&self) -> Option<&PositiveFixedPercentageType> {
        self.shade.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use shade()")]
    pub fn get_shade(&self) -> Option<&PositiveFixedPercentageType> {
        self.shade()
    }

    #[inline]
    pub fn shade_mut(&mut self) -> Option<&mut PositiveFixedPercentageType> {
        self.shade.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use shade_mut()")]
    pub fn get_shade_mut(&mut self) -> Option<&mut PositiveFixedPercentageType> {
        self.shade_mut()
    }

    #[inline]
    pub fn set_shade(&mut self, value: PositiveFixedPercentageType) {
        self.shade = Some(value);
    }

    #[inline]
    #[must_use]
    pub fn alpha(&self) -> Option<&PositiveFixedPercentageType> {
        self.alpha.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use alpha()")]
    pub fn get_alpha(&self) -> Option<&PositiveFixedPercentageType> {
        self.alpha()
    }

    #[inline]
    pub fn alpha_mut(&mut self) -> Option<&mut PositiveFixedPercentageType> {
        self.alpha.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use alpha_mut()")]
    pub fn get_alpha_mut(&mut self) -> Option<&mut PositiveFixedPercentageType> {
        self.alpha_mut()
    }

    #[inline]
    pub fn set_alpha(&mut self, value: PositiveFixedPercentageType) {
        self.alpha = Some(value);
    }

    #[inline]
    #[must_use]
    pub fn tint(&self) -> Option<&PositiveFixedPercentageType> {
        self.tint.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use tint()")]
    pub fn get_tint(&self) -> Option<&PositiveFixedPercentageType> {
        self.tint()
    }

    #[inline]
    pub fn tint_mut(&mut self) -> Option<&mut PositiveFixedPercentageType> {
        self.tint.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use tint_mut()")]
    pub fn get_tint_mut(&mut self) -> Option<&mut PositiveFixedPercentageType> {
        self.tint_mut()
    }

    #[inline]
    pub fn set_tint(&mut self, value: PositiveFixedPercentageType) {
        self.tint = Some(value);
    }

    #[inline]
    pub(crate) fn with_inner_params(&self) -> bool {
        self.luminance.is_some()
            || self.luminance_modulation.is_some()
            || self.luminance_offset.is_some()
            || self.saturation.is_some()
            || self.saturation_modulation.is_some()
            || self.shade.is_some()
            || self.alpha.is_some()
            || self.tint.is_some()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"a:lum" => {
                        let mut obj = PercentageType::default();
                        obj.set_attributes(reader, e);
                        self.luminance = Some(obj);
                    }
                    b"a:lumMod" => {
                        let mut obj = PercentageType::default();
                        obj.set_attributes(reader, e);
                        self.luminance_modulation = Some(obj);
                    }
                    b"a:lumOff" => {
                        let mut obj = PercentageType::default();
                        obj.set_attributes(reader, e);
                        self.luminance_offset = Some(obj);
                    }
                    b"a:sat" => {
                        let mut obj = PercentageType::default();
                        obj.set_attributes(reader, e);
                        self.saturation = Some(obj);
                    }
                    b"a:satMod" => {
                        let mut obj = PercentageType::default();
                        obj.set_attributes(reader, e);
                        self.saturation_modulation = Some(obj);
                    }
                    b"a:shade" => {
                        let mut obj = PositiveFixedPercentageType::default();
                        obj.set_attributes(reader, e);
                        self.shade = Some(obj);
                    }
                    b"a:alpha" => {
                        let mut obj = PositiveFixedPercentageType::default();
                        obj.set_attributes(reader, e);
                        self.alpha = Some(obj);
                    }
                    b"a:tint" => {
                        let mut obj = PositiveFixedPercentageType::default();
                        obj.set_attributes(reader, e);
                        self.tint = Some(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:schemeClr" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:schemeClr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:schemeClr
        if self.with_inner_params() {
            write_start_tag(
                writer,
                "a:schemeClr",
                vec![("val", self.val.value_string()).into()],
                false,
            );

            // a:luminance
            if let Some(v) = &self.luminance {
                v.write_to_lum(writer);
            }

            // a:lumMod
            if let Some(v) = &self.luminance_modulation {
                v.write_to_lum_mod(writer);
            }

            // a:lumOff
            if let Some(v) = &self.luminance_offset {
                v.write_to_lum_off(writer);
            }

            // a:sat
            if let Some(v) = &self.saturation {
                v.write_to_sat(writer);
            }

            // a:satMod
            if let Some(v) = &self.saturation_modulation {
                v.write_to_sat_mod(writer);
            }

            // a:shade
            if let Some(v) = &self.shade {
                v.write_to_shade(writer);
            }

            // a:alpha
            if let Some(v) = &self.alpha {
                v.write_to_alpha(writer);
            }

            // a:tint
            if let Some(v) = &self.tint {
                v.write_to_tint(writer);
            }

            write_end_tag(writer, "a:schemeClr");
        } else {
            write_start_tag(
                writer,
                "a:schemeClr",
                vec![("val", self.val.value_string()).into()],
                true,
            );
        }
    }
}
