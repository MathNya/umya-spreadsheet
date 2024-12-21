// a:bevelT
use super::super::EnumValue;
use super::super::Int64Value;
use super::BevelPresetValues;
use crate::reader::driver::{get_attribute, set_string_from_xml};
use crate::writer::driver::write_start_tag;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct BevelTop {
    width: Int64Value,
    height: Int64Value,
    preset: EnumValue<BevelPresetValues>,
}

impl BevelTop {
    #[inline]
    #[must_use]
    pub fn get_width(&self) -> i64 {
        self.width.get_value()
    }

    #[inline]
    pub fn set_width(&mut self, value: i64) -> &mut BevelTop {
        self.width.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_height(&self) -> i64 {
        self.height.get_value()
    }

    #[inline]
    pub fn set_height(&mut self, value: i64) -> &mut BevelTop {
        self.height.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_preset(&self) -> &BevelPresetValues {
        self.preset.get_value()
    }

    #[inline]
    pub fn set_preset(&mut self, value: BevelPresetValues) -> &mut BevelTop {
        self.preset.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, width, "w");
        set_string_from_xml!(self, e, height, "h");
        set_string_from_xml!(self, e, preset, "prst");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:bevelT
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let width = self.width.get_value_string();
        if self.width.has_value() {
            attributes.push(("w", &width));
        }
        let height = self.height.get_value_string();
        if self.height.has_value() {
            attributes.push(("h", &height));
        }
        if self.preset.has_value() {
            attributes.push(("prst", self.preset.get_value_string()));
        }

        write_start_tag(writer, "a:bevelT", attributes, true);
    }
}
