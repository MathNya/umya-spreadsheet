// a:srgbClr
use super::super::StringValue;
use super::PercentageType;
use super::PositiveFixedPercentageType;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct RgbColorModelHex {
    val: StringValue,
    luminance: Option<PercentageType>,
    luminance_modulation: Option<PercentageType>,
    luminance_offset: Option<PercentageType>,
    saturation: Option<PercentageType>,
    saturation_modulation: Option<PercentageType>,
    shade: Option<PositiveFixedPercentageType>,
    alpha: Option<PositiveFixedPercentageType>,
    tint: Option<PositiveFixedPercentageType>,
}
impl RgbColorModelHex {
    pub fn get_val(&self) -> &str {
        self.val.get_value()
    }

    pub fn set_val<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.val.set_value(value);
        self
    }

    pub fn get_luminance(&self) -> &Option<PercentageType> {
        &self.luminance
    }

    pub fn get_luminance_mut(&mut self) -> &mut Option<PercentageType> {
        &mut self.luminance
    }

    pub fn set_luminance(&mut self, value: PercentageType) {
        self.luminance = Some(value);
    }

    pub fn get_luminance_modulation(&self) -> &Option<PercentageType> {
        &self.luminance_modulation
    }

    pub fn get_luminance_modulation_mut(&mut self) -> &mut Option<PercentageType> {
        &mut self.luminance_modulation
    }

    pub fn set_luminance_modulation(&mut self, value: PercentageType) {
        self.luminance_modulation = Some(value);
    }

    pub fn get_luminance_offset(&self) -> &Option<PercentageType> {
        &self.luminance_offset
    }

    pub fn get_luminance_offset_mut(&mut self) -> &mut Option<PercentageType> {
        &mut self.luminance_offset
    }

    pub fn set_luminance_offset(&mut self, value: PercentageType) {
        self.luminance_offset = Some(value);
    }

    pub fn get_saturation(&self) -> &Option<PercentageType> {
        &self.saturation
    }

    pub fn get_saturation_mut(&mut self) -> &mut Option<PercentageType> {
        &mut self.saturation
    }

    pub fn set_saturation(&mut self, value: PercentageType) {
        self.saturation = Some(value);
    }

    pub fn get_saturation_modulation(&self) -> &Option<PercentageType> {
        &self.saturation_modulation
    }

    pub fn get_saturation_modulation_mut(&mut self) -> &mut Option<PercentageType> {
        &mut self.saturation_modulation
    }

    pub fn set_saturation_modulation(&mut self, value: PercentageType) {
        self.saturation_modulation = Some(value);
    }

    pub fn get_shade(&self) -> &Option<PositiveFixedPercentageType> {
        &self.shade
    }

    pub fn get_shade_mut(&mut self) -> &mut Option<PositiveFixedPercentageType> {
        &mut self.shade
    }

    pub fn set_shade(&mut self, value: PositiveFixedPercentageType) {
        self.shade = Some(value);
    }

    pub fn get_alpha(&self) -> &Option<PositiveFixedPercentageType> {
        &self.alpha
    }

    pub fn get_alpha_mut(&mut self) -> &mut Option<PositiveFixedPercentageType> {
        &mut self.alpha
    }

    pub fn set_alpha(&mut self, value: PositiveFixedPercentageType) {
        self.alpha = Some(value);
    }

    pub fn get_tint(&self) -> &Option<PositiveFixedPercentageType> {
        &self.tint
    }

    pub fn get_tint_mut(&mut self) -> &mut Option<PositiveFixedPercentageType> {
        &mut self.tint
    }

    pub fn set_tint(&mut self, value: PositiveFixedPercentageType) {
        self.tint = Some(value);
    }

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

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:srgbClr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:srgbClr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:srgbClr
        if self.with_inner_params() {
            write_start_tag(
                writer,
                "a:srgbClr",
                vec![("val", (self.val.get_value_string()))],
                false,
            );

            // a:luminance
            match &self.luminance {
                Some(v) => {
                    v.write_to_lum(writer);
                }
                None => {}
            }

            // a:lumMod
            match &self.luminance_modulation {
                Some(v) => {
                    v.write_to_lum_mod(writer);
                }
                None => {}
            }

            // a:lumOff
            match &self.luminance_offset {
                Some(v) => {
                    v.write_to_lum_off(writer);
                }
                None => {}
            }

            // a:sat
            match &self.saturation {
                Some(v) => {
                    v.write_to_sat(writer);
                }
                None => {}
            }

            // a:satMod
            match &self.saturation_modulation {
                Some(v) => {
                    v.write_to_sat_mod(writer);
                }
                None => {}
            }

            // a:shade
            match &self.shade {
                Some(v) => {
                    v.write_to_shade(writer);
                }
                None => {}
            }

            // a:alpha
            match &self.alpha {
                Some(v) => {
                    v.write_to_alpha(writer);
                }
                None => {}
            }

            // a:tint
            match &self.tint {
                Some(v) => {
                    v.write_to_tint(writer);
                }
                None => {}
            }

            write_end_tag(writer, "a:srgbClr");
        } else {
            write_start_tag(
                writer,
                "a:srgbClr",
                vec![("val", (self.val.get_value_string()))],
                true,
            );
        }
    }
}
